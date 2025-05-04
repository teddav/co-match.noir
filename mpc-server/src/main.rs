use axum::{
    Json, Router,
    extract::{Multipart, Query},
    http::{Method, StatusCode},
    routing::{get, post},
};
use co_noir::{Address, Bn254, CrsParser, NetworkParty, PartyID, Utils};
use co_ultrahonk::prelude::ZeroKnowledge;
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

mod db;
mod matching;
mod shares;

use db::{connect_db, setup_db};
use matching::{DIR, run_matches};
use shares::upload;

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

    let conn = connect_db()?;
    setup_db(&conn)?;

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
            get(move |Query(token): Query<String>| async move {
                match run_matches(
                    token,
                    parties,
                    program_artifact,
                    constraint_system,
                    recursive,
                    has_zk,
                    crs,
                )
                .await
                {
                    Ok(_) => (StatusCode::OK, Json("ok")),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("error")),
                }
            }),
        )
        .route(
            "/upload",
            post(|multipart: Multipart| async {
                match upload(multipart).await {
                    Ok(id) => (StatusCode::OK, Json(id)),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("error".to_string())),
                }
            }),
        )
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
