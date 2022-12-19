use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use chrono::{DateTime, Local};
use shared::{ProduceraFantasiforsterFörfrågan, RegistreraHjärnaFörfrågan};
use sqlx::{types::Uuid, Pool, Postgres};

pub struct ProduceraReaktion {
    pub uuid: Uuid,
    pub födelsedag: DateTime<Local>,
}
#[async_trait]
pub trait ProduceraFrånFörfrågan {
    async fn producera(
        &self, pool: Pool<Postgres>, utländsk_id: Uuid
    ) -> Option<ProduceraReaktion>;
}
#[async_trait]
impl ProduceraFrånFörfrågan for ProduceraFantasiforsterFörfrågan {
    async fn producera(
        &self,
        pool: Pool<Postgres>,
        utländsk_id: Uuid,
    ) -> Option<ProduceraReaktion> {
        let create_query = sqlx::query!(
            "INSERT INTO
                fantasifoster
                (titel,innehåll,födelsedag,uppfinnare)
                VALUES(
                $1,
                $2,
                NOW(),
                $3)
                RETURNING id, födelsedag",
            &self.skaffa_mig_din_titel(),
            &self.skaffa_mig_ditt_innehåll(),
            utländsk_id
        )
        .fetch_one(&pool)
        .await;
        match create_query {
            Ok(result) => Some(ProduceraReaktion {
                uuid: result.id,
                födelsedag: result.födelsedag.unwrap().into(),
            }),
            Err(_) => None,
        }
    }
}

#[async_trait]
impl ProduceraFrånFörfrågan for RegistreraHjärnaFörfrågan {
    async fn producera(
        &self,
        pool: Pool<Postgres>,
        _utländsk_id: Uuid,
    ) -> Option<ProduceraReaktion> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let Ok(password_hash) =
            argon2.hash_password(&self.skaffa_mig_ditt_lösenord().as_bytes(), &salt) else{

            }

        let create_query = sqlx::query!(
            "INSERT INTO
                hjärnor
                (hjärnannamn)
                VALUES(
                $1)
                RETURNING id, födelsedag",
            &self.skaffa_mig_ditt_namn()
        )
        .fetch_one(&pool)
        .await;
        match create_query {
            Ok(result) => Some(ProduceraReaktion {
                uuid: result.id,
                födelsedag: result.födelsedag.unwrap().into(),
            }),
            Err(_) => None,
        }
    }
}
