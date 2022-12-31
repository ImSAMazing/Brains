use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use shared::DemonstreraBesittarHjärnaFörfrågon;
use sqlx::{types::Uuid, Pool, Postgres};

pub async fn verifiera_lösenord(
    pool: Pool<Postgres>,
    förfrågon: &DemonstreraBesittarHjärnaFörfrågon,
) -> Option<Uuid> {
    let hjärna_query = sqlx::query!(
        "select id, lösenord from hjärnor where hjärnannamn=$1 LIMIT 1",
        &förfrågon.skaffa_mig_ditt_namn(),
    )
    .fetch_one(&pool)
    .await;
    if let Ok(result) = hjärna_query {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&result.lösenord).unwrap();
        //tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        if argon2
            .verify_password(
                förfrågon.skaffa_mig_ditt_lösenord().as_bytes(),
                &parsed_hash,
            )
            .is_ok()
        {
            return Some(result.id);
        }
    }
    None
}
