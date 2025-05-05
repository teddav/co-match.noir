use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{Error, StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

// pub fn decode_token(token: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
//     let key = DecodingKey::from_secret(SECRET.as_ref());
//     let token = decode::<Token>(&token, &key, &Validation::default())?;
//     Ok(token.claims.user_id)
// }

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body: Json<_> = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Self>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
