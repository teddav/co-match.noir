use axum::{Router, routing::get};
use co_noir::{
    AcirFormat, Address, Bn254, CrsParser, NetworkConfig, NetworkParty, PartyID, Poseidon2Sponge,
    Rep3AcvmType, Rep3CoUltraHonk, Rep3MpcNet, UltraHonk, Utils,
};
use co_ultrahonk::prelude::{Crs, ZeroKnowledge};
use noirc_artifacts::program::ProgramArtifact;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::sync::Arc;
use std::thread;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{Duration, Instant},
};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE | FmtSpan::ENTER);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");

    // connect to network
    let parties = vec![
        NetworkParty::new(
            PartyID::ID0.into(),
            Address::new("localhost".to_string(), 10000),
            CertificateDer::from(std::fs::read(dir.join("cert0.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID1.into(),
            Address::new("localhost".to_string(), 10001),
            CertificateDer::from(std::fs::read(dir.join("cert1.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID2.into(),
            Address::new("localhost".to_string(), 10002),
            CertificateDer::from(std::fs::read(dir.join("cert2.der"))?).into_owned(),
        ),
    ];

    let program_artifact = Utils::get_program_artifact_from_file(dir.join("circuit.json"))?;
    let constraint_system = Utils::get_constraint_system_from_artifact(&program_artifact, true);

    let recursive = true;
    let has_zk = ZeroKnowledge::No;

    let crs_size = co_noir::compute_circuit_size::<Bn254>(&constraint_system, recursive)?;
    let crs = CrsParser::<Bn254>::get_crs(
        dir.join("bn254_g1.dat"),
        dir.join("bn254_g2.dat"),
        crs_size,
        has_zk,
    )?;

    let app = Router::new().route(
        "/",
        get(move || async move {
            run_match(
                dir,
                parties,
                program_artifact,
                constraint_system,
                recursive,
                has_zk,
                crs,
            )
            .await
            .unwrap()
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn run_match(
    dir: PathBuf,
    parties: Vec<NetworkParty>,
    program_artifact: ProgramArtifact,
    constraint_system: AcirFormat<ark_bn254::Fr>,
    recursive: bool,
    has_zk: ZeroKnowledge,
    crs: Crs<Bn254>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let match_time = Instant::now();

    let [share0, share1, share2] = split_input(dir.join("Prover.toml"), program_artifact.clone())?;

    let data0 = Arc::new(DataForThread {
        id: PartyID::ID0,
        port: 10000,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            dir.join("key0.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share0,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        crs: crs.clone(),
    });
    let data1 = Arc::new(DataForThread {
        id: PartyID::ID1,
        port: 10001,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            dir.join("key1.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share1,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        crs: crs.clone(),
    });
    let data2 = Arc::new(DataForThread {
        id: PartyID::ID2,
        port: 10002,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            dir.join("key2.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share2,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        crs: crs.clone(),
    });

    let handles = vec![
        thread::spawn(move || spawn_party(data0)),
        thread::spawn(move || spawn_party(data1)),
        thread::spawn(move || spawn_party(data2)),
    ];

    for handle in handles {
        let _ = handle.join().unwrap()?;
    }

    println!("match time: {:?}", match_time.elapsed());

    Ok(())
}

type Share = BTreeMap<String, Rep3AcvmType<ark_bn254::Fr>>;
fn split_input(
    input_path: PathBuf,
    program_artifact: ProgramArtifact,
) -> Result<[Share; 3], Box<dyn std::error::Error + Send + Sync + 'static>> {
    let inputs = co_noir::parse_input(input_path, &program_artifact)?;

    let mut rng = rand::thread_rng();
    let [share0, share1, share2] =
        co_noir::split_input_rep3::<Bn254, Rep3MpcNet, _>(inputs, &mut rng);

    Ok([share0, share1, share2])
}

struct DataForThread {
    id: PartyID,
    port: u16,
    key: PrivateKeyDer<'static>,
    parties: Vec<NetworkParty>,
    share: Share,
    program_artifact: ProgramArtifact,
    constraint_system: AcirFormat<ark_bn254::Fr>,
    recursive: bool,
    has_zk: ZeroKnowledge,
    crs: Crs<Bn254>,
}

fn spawn_party(
    data: Arc<DataForThread>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    if let Ok(DataForThread {
        id,
        port,
        key,
        parties,
        share,
        program_artifact,
        constraint_system,
        recursive,
        has_zk,
        crs,
    }) = Arc::try_unwrap(data)
    {
        let (prover_crs, verifier_crs) = crs.split();

        let start_network = Instant::now();
        let network_config = NetworkConfig::new(
            id.into(),
            format!("0.0.0.0:{}", port).parse()?,
            key,
            parties,
            Some(Duration::from_secs(60)),
        );
        let net = Rep3MpcNet::new(network_config)?;
        println!("network setup time: {:?}", start_network.elapsed());

        let start_proof = Instant::now();

        // generate witness
        let (witness_share, net) = co_noir::generate_witness_rep3(share, program_artifact, net)?;

        // generate proving key and vk
        let (pk, net) =
            co_noir::generate_proving_key_rep3(net, &constraint_system, witness_share, recursive)?;
        let vk = pk.create_vk(&prover_crs, verifier_crs)?;

        // generate proof
        let (proof, _) =
            Rep3CoUltraHonk::<_, _, Poseidon2Sponge>::prove(net, pk, &prover_crs, has_zk)?;

        println!("proof time: {:?}", start_proof.elapsed());

        // verify proof
        assert!(UltraHonk::<_, Poseidon2Sponge>::verify(proof, &vk, has_zk)?);
        Ok(())
    } else {
        println!("error");
        Err("error".into())
    }
}
