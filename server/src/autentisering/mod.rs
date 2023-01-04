use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use shared::JwtInformation;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtDataHolder {
    pub information: JwtInformation,
}

#[async_trait]
impl<B> FromRequestParts<B> for JwtDataHolder
where
    B: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        {
            if let Some(information) = konvertera_jwt(bearer.token()) {
                Ok(JwtDataHolder { information })
            } else {
                Err(StatusCode::UNPROCESSABLE_ENTITY)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

impl JwtDataHolder {
    fn skaffa_mig_audience() -> String {
        "HjärnorFörening".to_string()
    }
    fn skaffa_mig_issuer() -> String {
        "Hjärnan".to_string()
    }
}

fn skaffa_mig_din_hemlighet() -> RS384KeyPair {
    RS384KeyPair::from_pem(&std::fs::read_to_string("private.pem").unwrap()).unwrap()
}

fn skaffa_mig_din_hemlighet_public() -> RS384PublicKey {
    RS384PublicKey::from_pem(&std::fs::read_to_string("public.pem").unwrap()).unwrap()
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
    let claims = Claims::with_custom_claims(
        information,
        Duration::from_hours(
            std::env::var("TOKEN_DURATION_DAYS")
                .expect("TOKEN_DURATION_DAYS environmental variable not set")
                .parse()
                .expect("TOKEN_DURATION_DAYS is not an integer (i64)"),
        ),
    )
    .with_audience(JwtDataHolder::skaffa_mig_audience())
    .with_issuer(JwtDataHolder::skaffa_mig_issuer())
    .with_subject(id);
    let token = skaffa_mig_din_hemlighet().sign(claims);
    token.unwrap()
}

pub fn konvertera_jwt(raw_token: &str) -> Option<JwtInformation> {
    let mut options = VerificationOptions::default();
    // reject tokens if they don't include an issuer from that list
    options.allowed_issuers = Some(HashSet::from_strings(&[JwtDataHolder::skaffa_mig_issuer()]));
    options.allowed_audiences = Some(HashSet::from_strings(&[
        JwtDataHolder::skaffa_mig_audience(),
    ]));
    // see the documentation for the full list of available options

    if let Ok(claims) =
        skaffa_mig_din_hemlighet_public().verify_token::<JwtInformation>(raw_token, Some(options))
    {
        Some(claims.custom)
    } else {
        None
    }
}
