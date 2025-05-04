use serde_json::json;
use std::path::PathBuf;
use vercel_runtime::{
    Body, Error, Request, RequestPayloadExt, Response, StatusCode, http::bad_request, run,
};

use co_match::{Prover, save_prover_data, split_input};

const CIRCUIT_PATH: &str = "data/circuit.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<Prover>();
    println!("payload: {:#?}", payload);

    let payload = match payload {
        Err(..) => return bad_request("Invalid payload"),
        Ok(None) => return bad_request("No payload"),
        Ok(Some(payload)) => payload,
    };

    let prover_path1 = save_prover_data(&payload, false)?;
    let prover_path2 = save_prover_data(&payload, true)?;
    let circuit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(CIRCUIT_PATH);

    let shares1 = split_input(&circuit_path, &prover_path1)?;
    let shares2 = split_input(&circuit_path, &prover_path2)?;

    let mut out = shares1
        .iter()
        .map(|d| hex::encode(d))
        .collect::<Vec<String>>();

    let out2 = shares2
        .iter()
        .map(|d| hex::encode(d))
        .collect::<Vec<String>>();

    out.extend(out2);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "success",
              "shares": out
            })
            .to_string()
            .into(),
        )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::request::Builder;

    #[tokio::test]
    async fn test_split_handler() {
        // let req = Request::new(Body::from("salut!!"));
        let payload = r#"
        {
            "user1": {
                "age": 30,
                "gender": 0,
                "id": "0x1fed07ad686a727dfc33b91206d526e61f519dca9c5054ae729231c201717633",
                "id_nullifier": 12345,
                "interests": [5, 8, 10],
                "region": 1,
                "preferences": {
                    "age_max": 35,
                    "age_min": 25,
                    "gender": 1
                }
            },
            "user2": {
                "age": 32,
                "gender": 1,
                "id": "0x16e31ced6c74696a601f45f1bb2b9833380d51348fe89644360d0e5abeaf244a",
                "id_nullifier": 67890,
                "interests": [10, 20, 30],
                "region": 1,
                "preferences": {
                    "age_max": 35,
                    "age_min": 25,
                    "gender": 1
                }
            }
        }
        "#;
        let req = Builder::new()
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let res = handler(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_split_invalid() {
        let payload = r#"
        { "user1": { "age": 30 } }
        "#;
        let req = Builder::new()
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(payload))
            .unwrap();

        let res = handler(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(res.body(), &Body::from("\"Invalid payload\""));
    }
}
