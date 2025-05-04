use axum::{
    Json, Router,
    extract::Multipart,
    http::Method,
    routing::{get, post},
};
use co_noir::{Address, Bn254, CrsParser, NetworkParty, PartyID, Utils};
use co_ultrahonk::prelude::ZeroKnowledge;
use rand::{Rng, distributions::Alphanumeric};
use rustls::pki_types::CertificateDer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
};

mod matching;
use matching::{DIR, run_match};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE | FmtSpan::ENTER);
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");

    // connect to network
    let parties = vec![
        NetworkParty::new(
            PartyID::ID0.into(),
            Address::new("localhost".to_string(), 10000),
            CertificateDer::from(std::fs::read(DIR.join("cert0.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID1.into(),
            Address::new("localhost".to_string(), 10001),
            CertificateDer::from(std::fs::read(DIR.join("cert1.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID2.into(),
            Address::new("localhost".to_string(), 10002),
            CertificateDer::from(std::fs::read(DIR.join("cert2.der"))?).into_owned(),
        ),
    ];

    let program_artifact = Utils::get_program_artifact_from_file(DIR.join("circuit.json"))?;
    let constraint_system = Utils::get_constraint_system_from_artifact(&program_artifact, true);

    let recursive = true;
    let has_zk = ZeroKnowledge::No;

    let crs_size = co_noir::compute_circuit_size::<Bn254>(&constraint_system, recursive)?;
    let crs = CrsParser::<Bn254>::get_crs(
        DIR.join("bn254_g1.dat"),
        DIR.join("bn254_g2.dat"),
        crs_size,
        has_zk,
    )?;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route(
            "/",
            get(move || async move {
                run_match(
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
        )
        .route("/upload", post(upload))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn upload(mut multipart: Multipart) -> Json<String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("{name:?} {data:?}");

        let file_name = random_id();
        let dir = DIR.join("tmp");
        tokio::fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join(file_name);
        tokio::fs::write(file_path, data).await.unwrap();
    }

    Json("ok".to_string())
}

fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
