use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use shared::ProveOwnsBrainRequest;
use sqlx::{types::Uuid, Pool, Postgres};

pub async fn verify_password(
    pool: Pool<Postgres>,
    förfrågon: &ProveOwnsBrainRequest,
) -> Option<Uuid> {
    let brain_query = sqlx::query!(
        "select id, password from brains where brainname=$1 LIMIT 1",
        &förfrågon.get_name(),
    )
    .fetch_one(&pool)
    .await;
    if let Ok(result) = brain_query {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&result.password).unwrap();
        //tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        if argon2
            .verify_password(förfrågon.get_password().as_bytes(), &parsed_hash)
            .is_ok()
        {
            return Some(result.id);
        }
    }
    None
}
