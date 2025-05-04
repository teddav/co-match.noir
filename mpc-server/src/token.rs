use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

// TODO: move to env
const SECRET: &str = "secret_secret_secret_secret_secret_secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub user_id: String,
    pub exp: u64,
}

pub fn encode_token(user_id: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let key = EncodingKey::from_secret(SECRET.as_ref());
    let token = encode(
        &Header::default(),
        &Token {
            user_id,
            exp: 100000000000,
        },
        &key,
    )?;
    Ok(token)
}

pub fn decode_token(token: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let key = DecodingKey::from_secret(SECRET.as_ref());
    let token = decode::<Token>(&token, &key, &Validation::default())?;
    Ok(token.claims.user_id)
}
