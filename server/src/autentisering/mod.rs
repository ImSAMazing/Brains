use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::JwtInformation;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    iat: usize,
    iss: String,
    exp: usize,
    pub information: JwtInformation,
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        {
            if let Some(data) = konvertera_jwt(bearer.token()) {
                Ok(data)
            } else {
                Err(StatusCode::UNPROCESSABLE_ENTITY)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

impl Claims {
    fn skaffa_mig_audience() -> String {
        "HjärnorFörening".to_string()
    }
    fn skaffa_mig_issuer() -> String {
        "Hjärnan".to_string()
    }
    fn skaffa_mig_tidsspan() -> i64 {
        7 * 24 * 60 * 60
    }
    fn producera(id: Uuid, information: JwtInformation) -> Claims {
        let now = Utc::now().timestamp();
        Claims {
            sub: id.to_string(),
            aud: Claims::skaffa_mig_audience(),
            iss: Claims::skaffa_mig_issuer(),
            iat: now as usize,
            exp: (now + Claims::skaffa_mig_tidsspan()) as usize,
            information,
        }
    }
}

fn skaffa_mig_din_hemlighet() -> EncodingKey {
    EncodingKey::from_secret(
        std::env::var("JWT_KEY")
            .expect("JWT_KEY environmental variable not set")
            .as_bytes(),
    )
}

fn skaffa_mig_din_hemlighet_decoding() -> DecodingKey {
    DecodingKey::from_secret(
        std::env::var("JWT_KEY")
            .expect("JWT_KEY environmental variable not set")
            .as_bytes(),
    )
}

pub fn producera_jwt(id: Uuid, hjärnannamn: String) -> String {
    producera_jwt_från_information(
        id,
        JwtInformation {
            id: id.to_string(),
            hjärnannamn: hjärnannamn,
        },
    )
}

fn producera_jwt_från_information(id: Uuid, information: JwtInformation) -> String {
    let claims = Claims::producera(id, information);
    let token = encode(&Header::default(), &claims, &skaffa_mig_din_hemlighet());
    token.unwrap()
}

pub fn konvertera_jwt(raw_token: &str) -> Option<Claims> {
    let mut validator = Validation::default();
    validator.set_audience(&[Claims::skaffa_mig_audience()]);
    validator.set_issuer(&[Claims::skaffa_mig_issuer()]);
    if let Ok(token) = decode::<Claims>(raw_token, &skaffa_mig_din_hemlighet_decoding(), &validator)
    {
        Some(token.claims)
    } else {
        None
    }
}
