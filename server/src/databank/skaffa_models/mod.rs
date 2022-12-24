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
                    id: a.id,
                    titel: a.titel.clone().unwrap(),
                    innehåll: a.innehåll.clone().unwrap(),
                    födelsedag: a.födelsedag.unwrap().into(),
                    uppfinnare_namn: a.uppfinnare_namn.clone().unwrap(),
                })
                .collect(),
        )
    } else {
        None
    }
}
