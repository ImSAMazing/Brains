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
    fn producera(subject: String) -> Claims {
        let now = Utc::now().timestamp();
        Claims {
            sub: subject,
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
pub fn producera_jwt(hjärnannamn: String) -> String {
    let claims = Claims::producera(hjärnannamn);
    let token = encode(&Header::default(), &claims, &skaffa_mig_din_hemlighet());
    token.unwrap()
}
