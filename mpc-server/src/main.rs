use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Multipart, Query},
    http::{HeaderMap, Method, StatusCode},
    routing::{get, post},
};
use co_noir::{Address, Bn254, CrsParser, NetworkParty, PartyID, Utils};
use co_ultrahonk::prelude::ZeroKnowledge;
use rusqlite::Connection;
use rustls::pki_types::CertificateDer;
use serde::Deserialize;
use serde_json::json;
use token::{Token, encode_token};
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

use db::{connect_db, get_all_users, setup_db};
use matching::{DIR, run_matches};
use shares::upload;

#[derive(Debug, Deserialize)]
pub struct TokenQuery {
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    twitter_handle: String,
}

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
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // things(&conn)?;

    let app = Router::new()
        .route(
            "/",
            get(move |token: Token| async move {
                match run_matches(
                    token.user_id,
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
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, Json("error"))
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn things(conn: &Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = encode_token("V2IBcEl7dx".to_string())?;
    println!("token: {}", token);

    // insert_user(&conn, "abc", "@abc", "h123")?;
    // insert_user(&conn, "def", "@def", "h456")?;

    // let user = get_user(&conn, "abc")?;
    // println!("abc: {:?}", user);

    // update_checked(&conn, "abc", vec!["abc".to_string(), "def".to_string()])?;

    let users = get_all_users(&conn)?;
    println!("users: {:#?}", users);

    Ok(())
}
