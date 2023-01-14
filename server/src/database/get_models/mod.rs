use shared::{BrainfartFilter, BrainfartInformation};
use sqlx::{Pool, Postgres};

pub async fn _get_brainfarts_från_filter(
    pool: Pool<Postgres>,
    _filter: BrainfartFilter,
) -> Option<Vec<BrainfartInformation>> {
    let select_query = sqlx::query!(
        "select id, title, content, birthdate, (select brainname from brains where id=mastermind LIMIT 1) as mastermind_name FROM
            brainfarts ORDER BY birthdate DESC"
    )
    .fetch_all(&pool)
    .await;

    if let Ok(result) = select_query {
        Some(
            result
                .iter()
                .map(|a| BrainfartInformation {
                    id: a.id.to_string(),
                    title: a.title.clone(),
                    content: a.content.clone(),
                    birthdate: a.birthdate.into(),
                    mastermind_name: a.mastermind_name.clone().unwrap(),
                })
                .collect(),
        )
    } else {
        None
    }
}
