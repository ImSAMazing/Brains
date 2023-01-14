use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::async_trait;
use chrono::{DateTime, Local};
use shared::{CreateBrainfartRequest, RegisterBrainRequest};
use sqlx::{types::Uuid, Pool, Postgres};

pub struct CreateResponse {
    pub uuid: Uuid,
    pub birthdate: DateTime<Local>,
    pub extra_information: Option<String>,
}
#[async_trait]
pub trait CreateFromRequest {
    async fn create(&self, pool: Pool<Postgres>, foreign_id: Uuid) -> Option<CreateResponse>;
}
#[async_trait]
impl CreateFromRequest for CreateBrainfartRequest {
    async fn create(&self, pool: Pool<Postgres>, foreign_id: Uuid) -> Option<CreateResponse> {
        let create_query = sqlx::query!(
            "INSERT INTO
                brainfarts
                (title,content,birthdate,mastermind)
                VALUES(
                $1,
                $2,
                NOW(),
                $3)
                RETURNING id, birthdate",
            &self.get_title(),
            &self.get_content(),
            foreign_id
        )
        .fetch_one(&pool)
        .await;
        match create_query {
            Ok(result) => Some(CreateResponse {
                uuid: result.id,
                birthdate: result.birthdate.into(),
                extra_information: None,
            }),
            Err(_) => None,
        }
    }
}

#[async_trait]
impl CreateFromRequest for RegisterBrainRequest {
    async fn create(&self, pool: Pool<Postgres>, _foreign_id: Uuid) -> Option<CreateResponse> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        if let Ok(password_hash) = argon2.hash_password(&self.get_password().as_bytes(), &salt) {
            let create_query = sqlx::query!(
                "INSERT INTO
                brains
                (brainname, password)
                VALUES(
                $1,
                $2)
                RETURNING id, birthdate",
                &self.get_name(),
                password_hash.to_string()
            )
            .fetch_one(&pool)
            .await;
            match create_query {
                Ok(result) => Some(CreateResponse {
                    uuid: result.id,
                    birthdate: result.birthdate.into(),
                    extra_information: Some(password_hash.to_string()),
                }),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
