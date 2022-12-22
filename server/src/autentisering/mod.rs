use chrono::{Local, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::Hjärna;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iat: usize,
    iss: String,
    exp: usize,
}

pub struct Jwt_Information {
    hjärnannamn: String,
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
    fn producera(information: Jwt_Information) -> Claims {
        let now = Utc::now().timestamp();
        Claims {
            sub: information.hjärnannamn,
            aud: Claims::skaffa_mig_audience(),
            iss: Claims::skaffa_mig_issuer(),
            iat: now as usize,
            exp: (now + Claims::skaffa_mig_tidsspan()) as usize,
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
    DecodingKey::from_secret("secret".as_ref())
}

pub fn producera_jwt(hjärnannamn: String) -> String {
    producera_jwt_från_information(Jwt_Information { hjärnannamn })
}

fn producera_jwt_från_information(information: Jwt_Information) -> String {
    let claims = Claims::producera(information);
    let token = encode(&Header::default(), &claims, &skaffa_mig_din_hemlighet());
    token.unwrap()
}

pub fn konvertera_jwt(raw_token: &str) -> Jwt_Information {
    let mut validator = Validation::default();
    validator.set_audience(Claims::skaffa_mig_audience().as_bytes());
    validator.set_issuer(Claims::skaffa_mig_issuer().as_bytes());
    let token =
        decode::<Claims>(raw_token, &skaffa_mig_din_hemlighet_decoding(), &validator).unwrap();
    Jwt_Information {
        hjärnannamn: token.claims.sub,
    }
}
