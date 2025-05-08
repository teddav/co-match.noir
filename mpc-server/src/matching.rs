use co_noir::{
    AcirFormat, Address, Bn254, NetworkConfig, NetworkParty, PartyID, Poseidon2Sponge,
    Rep3CoUltraHonk, Rep3MpcNet, UltraHonk, merge_input_shares,
};
use co_ultrahonk::prelude::{ProverCrs, ZeroKnowledge};
use noirc_artifacts::program::ProgramArtifact;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};
use std::{
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::db::{
    connect_db, get_all_users, get_user, insert_matches, update_checked, update_checked_many,
};
use crate::shares::{Share, get_shares};

pub const DATA_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"));
pub const CONFIG_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config"));
pub const SHARES_DIR_1: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("user1"));
pub const SHARES_DIR_2: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("user2"));

pub async fn run_matches(
    user_id: String,
    parties_certs: [CertificateDer<'static>; 3],
    program_artifact: &ProgramArtifact,
    constraint_system: Arc<AcirFormat<ark_bn254::Fr>>,
    recursive: bool,
    has_zk: ZeroKnowledge,
    prover_crs: Arc<ProverCrs<Bn254>>,
    verifier_crs: Arc<ark_bn254::G2Affine>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let conn = connect_db()?;

    let user1 = get_user(&conn, &user_id)?;
    let all_users = get_all_users(&conn)?
        .into_iter()
        .filter(|u| !user1.checked.contains(&u.id))
        .collect::<Vec<_>>();

    update_checked(
        &conn,
        &user1.id,
        all_users
            .clone()
            .into_iter()
            .map(|u| u.id)
            .collect::<Vec<String>>(),
    )?;

    let users2 = all_users
        .iter()
        .map(|u| u.id.clone())
        .collect::<Vec<String>>();

    println!("ALL USERS: {:?}", all_users.len());

    let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build()?;

    let verified_matches = pool.install(|| -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        all_users
            .into_par_iter()
            .enumerate()
            .map(|(thread_id, user2)| -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
                            let shares_user1 = get_shares(&user1.id, true)?;
                            let shares_user2 = get_shares(&user2.id, false)?;

                            let share0 = merge_shares(shares_user1[0].clone(), shares_user2[0].clone())?;
                            let share1 = merge_shares(shares_user1[1].clone(), shares_user2[1].clone())?;
                            let share2 = merge_shares(shares_user1[2].clone(), shares_user2[2].clone())?;

                            match run_match(
                                thread_id,
                                [share0, share1, share2],
                                parties_certs.clone(),
                                program_artifact,
                                constraint_system.clone(),
                                recursive,
                                has_zk,
                                prover_crs.clone(),
                                verifier_crs.clone(),
                            ) {
                                Ok(_) => Ok(user2.id),
                                Err(e) => Err(e),
                            }
                        },
                    )
                    .filter(|m| m.is_ok())
                    .collect::<Result<Vec<String>, _>>()
    })?;

    println!("verified matches: {verified_matches:?}");

    update_checked_many(&conn, users2, vec![user_id.clone()])?;

    insert_matches(
        &conn,
        verified_matches
            .iter()
            .map(|m| (user_id.clone(), m.clone()))
            .collect(),
    )?;
    Ok(())
}

pub fn run_match(
    thread_id: usize,
    [share0, share1, share2]: [Share; 3],
    parties_certs: [CertificateDer<'static>; 3],
    program_artifact: &ProgramArtifact,
    constraint_system: Arc<AcirFormat<ark_bn254::Fr>>,
    recursive: bool,
    has_zk: ZeroKnowledge,
    prover_crs: Arc<ProverCrs<Bn254>>,
    verifier_crs: Arc<ark_bn254::G2Affine>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let match_time = Instant::now();

    let party0_port = 10000 + thread_id as u16;
    let party1_port = 11000 + thread_id as u16;
    let party2_port = 12000 + thread_id as u16;

    let parties = vec![
        NetworkParty::new(
            PartyID::ID0.into(),
            Address::new("localhost".to_string(), party0_port),
            parties_certs[0].clone(),
        ),
        NetworkParty::new(
            PartyID::ID1.into(),
            Address::new("localhost".to_string(), party1_port),
            parties_certs[1].clone(),
        ),
        NetworkParty::new(
            PartyID::ID2.into(),
            Address::new("localhost".to_string(), party2_port),
            parties_certs[2].clone(),
        ),
    ];

    let data0 = DataForThread {
        id: PartyID::ID0,
        port: party0_port,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            CONFIG_DIR.join("key0.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share0,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        prover_crs: prover_crs.clone(),
        verifier_crs: verifier_crs.clone(),
    };
    let data1 = DataForThread {
        id: PartyID::ID1,
        port: party1_port,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            CONFIG_DIR.join("key1.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share1,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        prover_crs: prover_crs.clone(),
        verifier_crs: verifier_crs.clone(),
    };
    let data2 = DataForThread {
        id: PartyID::ID2,
        port: party2_port,
        key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(std::fs::read(
            CONFIG_DIR.join("key2.der"),
        )?))
        .clone_key(),
        parties: parties.clone(),
        share: share2,
        program_artifact: program_artifact.clone(),
        constraint_system: constraint_system.clone(),
        recursive,
        has_zk,
        prover_crs: prover_crs.clone(),
        verifier_crs: verifier_crs.clone(),
    };

    let handles = vec![
        thread::spawn(move || spawn_party(data0)),
        thread::spawn(move || spawn_party(data1)),
        thread::spawn(move || spawn_party(data2)),
    ];

    for handle in handles {
        let verified = handle.join().unwrap()?;
        if !verified {
            return Err("Proof verification failed".into());
        }
    }

    println!("match time: {:?}", match_time.elapsed());

    Ok(())
}

fn merge_shares(
    share_user1: Share,
    share_user2: Share,
) -> Result<Share, Box<dyn std::error::Error + Send + Sync>> {
    let merged = merge_input_shares::<Bn254>(vec![share_user1, share_user2])?;
    Ok(merged)
}

struct DataForThread {
    id: PartyID,
    port: u16,
    key: PrivateKeyDer<'static>,
    parties: Vec<NetworkParty>,
    share: Share,
    program_artifact: ProgramArtifact,
    constraint_system: Arc<AcirFormat<ark_bn254::Fr>>,
    recursive: bool,
    has_zk: ZeroKnowledge,
    prover_crs: Arc<ProverCrs<Bn254>>,
    verifier_crs: Arc<ark_bn254::G2Affine>,
}

fn spawn_party(
    data: DataForThread,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let DataForThread {
        id,
        port,
        key,
        parties,
        share,
        program_artifact,
        constraint_system,
        recursive,
        has_zk,
        prover_crs,
        verifier_crs,
    } = data;

    let start_network = Instant::now();
    let network_config = NetworkConfig::new(
        id.into(),
        format!("[::]:{}", port).parse()?,
        key,
        parties,
        Some(Duration::from_secs(60)),
    );
    let net = Rep3MpcNet::new(network_config)?;
    println!("network setup time: {:?}", start_network.elapsed());

    let total_time = Instant::now();

    let witness_time = Instant::now();
    let (witness_share, net) = co_noir::generate_witness_rep3(share, program_artifact, net)?;
    println!("witness time: {:?}", witness_time.elapsed());

    let pk_time = Instant::now();
    let (pk, net) =
        co_noir::generate_proving_key_rep3(net, &constraint_system, witness_share, recursive)?;
    let vk = pk.create_vk(&prover_crs, *verifier_crs)?;
    println!("pk time: {:?}", pk_time.elapsed());

    let proof_time = Instant::now();
    let (proof, _) = Rep3CoUltraHonk::<_, _, Poseidon2Sponge>::prove(net, pk, &prover_crs, has_zk)?;
    println!("proof time: {:?}", proof_time.elapsed());

    println!("TOTAL time: {:?}", total_time.elapsed());

    let verified = UltraHonk::<_, Poseidon2Sponge>::verify(proof, &vk, has_zk)?;

    Ok(verified)
}
