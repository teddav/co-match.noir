use axum::{
    Json, Router,
    extract::{Multipart, Query},
    http::StatusCode,
    routing::{get, post},
};
use axum_server::tls_rustls::RustlsConfig;
use co_noir::{Address, Bn254, CrsParser, NetworkParty, PartyID, Utils};
use co_ultrahonk::prelude::{ProverCrs, ZeroKnowledge};
use rustls::pki_types::CertificateDer;
use serde::Deserialize;
use serde_json::json;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use token::Token;
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
mod token;

use db::{get_matches, setup_db};
use matching::{CONFIG_DIR, DATA_DIR, run_matches};
use shares::{ProverData, split_handler, upload};

#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    twitter_handle: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

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

    let ssl_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ssl-cert");
    let config =
        RustlsConfig::from_pem_file(ssl_dir.join("cert.pem"), ssl_dir.join("key.pem")).await?;

    setup_db()?;

    let parties = vec![
        NetworkParty::new(
            PartyID::ID0.into(),
            Address::new("localhost".to_string(), 10000),
            CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert0.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID1.into(),
            Address::new("localhost".to_string(), 10001),
            CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert1.der"))?).into_owned(),
        ),
        NetworkParty::new(
            PartyID::ID2.into(),
            Address::new("localhost".to_string(), 10002),
            CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert2.der"))?).into_owned(),
        ),
    ];

    let program_artifact = Utils::get_program_artifact_from_file(DATA_DIR.join("circuit.json"))?;
    let constraint_system = Arc::new(Utils::get_constraint_system_from_artifact(
        &program_artifact,
        true,
    ));

    let recursive = true;
    let has_zk = ZeroKnowledge::No;

    let crs_size = co_noir::compute_circuit_size::<Bn254>(&constraint_system, recursive)?;
    let crs: (ProverCrs<Bn254>, ark_bn254::G2Affine) = CrsParser::<Bn254>::get_crs(
        CONFIG_DIR.join("bn254_g1.dat"),
        CONFIG_DIR.join("bn254_g2.dat"),
        crs_size,
        has_zk,
    )?
    .split();
    let prover_crs = Arc::new(crs.0);
    let verifier_crs = Arc::new(crs.1);

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let p = program_artifact.clone();

    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .route(
            "/split",
            post(|payload: Json<ProverData>| async move {
                match split_handler(payload.0, &p).await {
                    Ok(shares) => (StatusCode::OK, Json(json!({"shares": shares}))),
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": "error"})),
                        )
                    }
                }
            }),
        )
        .route(
            "/matches",
            post(move |token: Token| async move {
                match run_matches(
                    token.user_id,
                    parties,
                    &program_artifact,
                    constraint_system,
                    recursive,
                    has_zk,
                    prover_crs,
                    verifier_crs,
                )
                .await
                {
                    Ok(_) => (StatusCode::OK, Json("ok")),
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, Json("error"))
                    }
                }
            }),
        )
        .route(
            "/matches",
            get(move |token: Token| async move {
                match get_matches(token.user_id) {
                    Ok(matches) => (StatusCode::OK, Json(json!({"matches": matches}))),
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": "error"})),
                        )
                    }
                }
            }),
        )
        .route(
            "/upload",
            post(
                |Query(query): Query<UploadQuery>, multipart: Multipart| async {
                    println!("query: {:?}", query);
                    match upload(query.twitter_handle, multipart).await {
                        Ok(token) => (StatusCode::OK, Json(json!({"token": token}))),
                        Err(e) => {
                            println!("ERROR: {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": "error"})),
                            )
                        }
                    }
                },
            ),
        )
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    // axum::serve(listener, app).await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matching::run_match, shares::split_input};

    #[tokio::test]
    async fn test_match() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let prover_toml = r#"[user1]
age = 30
region = 1
gender = 0
id = "0x1fed07ad686a727dfc33b91206d526e61f519dca9c5054ae729231c201717633"
interests = [2, 4, 6]

[user1.preferences]
age_max = 35
age_min = 25
gender = 1

[user2]
age = 32
region = 1
gender = 1
id = "0x16e31ced6c74696a601f45f1bb2b9833380d51348fe89644360d0e5abeaf244a"
interests = [1, 2, 3]

[user2.preferences]
age_max = 35
age_min = 25
gender = 0"#;

        std::fs::write("Prover.toml", prover_toml).unwrap();

        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        let parties = vec![
            NetworkParty::new(
                PartyID::ID0.into(),
                Address::new("localhost".to_string(), 10000),
                CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert0.der"))?).into_owned(),
            ),
            NetworkParty::new(
                PartyID::ID1.into(),
                Address::new("localhost".to_string(), 10001),
                CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert1.der"))?).into_owned(),
            ),
            NetworkParty::new(
                PartyID::ID2.into(),
                Address::new("localhost".to_string(), 10002),
                CertificateDer::from(std::fs::read(CONFIG_DIR.join("cert2.der"))?).into_owned(),
            ),
        ];

        let program_artifact =
            Utils::get_program_artifact_from_file(DATA_DIR.join("circuit.json"))?;
        let constraint_system = Arc::new(Utils::get_constraint_system_from_artifact(
            &program_artifact,
            true,
        ));

        let recursive = true;
        let has_zk = ZeroKnowledge::No;

        let crs_size = co_noir::compute_circuit_size::<Bn254>(&constraint_system, recursive)?;
        let crs: (ProverCrs<Bn254>, ark_bn254::G2Affine) = CrsParser::<Bn254>::get_crs(
            CONFIG_DIR.join("bn254_g1.dat"),
            CONFIG_DIR.join("bn254_g2.dat"),
            crs_size,
            has_zk,
        )?
        .split();
        let prover_crs = Arc::new(crs.0);
        let verifier_crs = Arc::new(crs.1);

        let shares = split_input(PathBuf::from("Prover.toml"), &program_artifact)?;

        let result = run_match(
            shares,
            parties.clone(),
            &program_artifact,
            constraint_system.clone(),
            recursive,
            has_zk,
            prover_crs.clone(),
            verifier_crs.clone(),
        )
        .await;

        println!("result: {:?}", result);

        std::fs::remove_file("Prover.toml").unwrap();

        Ok(())
    }
}
