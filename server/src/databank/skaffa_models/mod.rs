use axum::async_trait;
use chrono::{DateTime, Local};
use shared::{Fantasiforster, FantasiforsterFilter, FantasiforsterInformation};
use sqlx::{types::Uuid, Pool, Postgres};

pub async fn skaffa_mig_fantasiforster_från_filter(
    pool: Pool<Postgres>,
    filter: FantasiforsterFilter,
) -> Option<Vec<FantasiforsterInformation>> {
    let select_query = sqlx::query!(
        "select id, titel, innehåll, födelsedag, (select hjärnannamn from hjärnor where id=uppfinnare LIMIT 1) as uppfinnare_namn FROM
            fantasifoster ORDER BY födelsedag DESC"
    )
    .fetch_all(&pool)
    .await;

    if let Ok(result) = select_query {
        Some(
            result
                .iter()
                .map(|a| FantasiforsterInformation {
                    id: a.id.to_string(),
                    titel: a.titel.clone(),
                    innehåll: a.innehåll.clone(),
                    födelsedag: a.födelsedag.into(),
                    uppfinnare_namn: a.uppfinnare_namn.clone().unwrap(),
                })
                .collect(),
        )
    } else {
        None
    }
}
