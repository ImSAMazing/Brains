use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use shared::DemonstreraBesittarHjärnaFörfrågon;
use sqlx::{Pool, Postgres};

pub async fn verifiera_lösenord(
    pool: Pool<Postgres>,
    förfrågon: DemonstreraBesittarHjärnaFörfrågon,
) -> Option<bool> {
    let hjärna_query = sqlx::query!(
        "select lösenord from hjärnor where hjärnannamn=$1 LIMIT 1",
        &förfrågon.skaffa_mig_ditt_namn(),
    )
    .fetch_one(&pool)
    .await;
    match hjärna_query {
        Ok(result) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&result.lösenord).unwrap();
            Some(
                argon2
                    .verify_password(
                        förfrågon.skaffa_mig_ditt_lösenord().as_bytes(),
                        &parsed_hash,
                    )
                    .is_ok(),
            )
        }
        Err(_) => None,
    }
}
