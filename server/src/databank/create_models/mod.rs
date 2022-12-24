use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::async_trait;
use chrono::{DateTime, Local};
use shared::{ProduceraFantasiforsterFörfrågan, RegistreraHjärnaFörfrågan};
use sqlx::{types::Uuid, Pool, Postgres};

pub struct ProduceraReaktion {
    pub uuid: Uuid,
    pub födelsedag: DateTime<Local>,
    pub tillägen_information: Option<String>,
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
                tillägen_information: None,
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
        if let Ok(password_hash) =
            argon2.hash_password(&self.skaffa_mig_ditt_lösenord().as_bytes(), &salt)
        {
            let create_query = sqlx::query!(
                "INSERT INTO
                hjärnor
                (hjärnannamn, lösenord)
                VALUES(
                $1,
                $2)
                RETURNING id, födelsedag",
                &self.skaffa_mig_ditt_namn(),
                password_hash.to_string()
            )
            .fetch_one(&pool)
            .await;
            match create_query {
                Ok(result) => Some(ProduceraReaktion {
                    uuid: result.id,
                    födelsedag: result.födelsedag.unwrap().into(),
                    tillägen_information: Some(password_hash.to_string()),
                }),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
