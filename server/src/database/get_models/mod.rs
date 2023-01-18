use std::future::Future;

use shared::{Brain, BrainInformation, BrainfartFilter, BrainfartInformation};
use sqlx::{types::Uuid, Pool, Postgres};

async fn get_brain_information(pool: Pool<Postgres>, brain_id: Uuid) -> Option<BrainInformation> {
    let query = sqlx::query!(
        "select brainname, birthdate from brains where id=$1 LIMIT 1",
        brain_id
    )
    .fetch_one(&pool)
    .await;
    if let Ok(result) = query {
        Some(BrainInformation::create(
            brain_id.to_string(),
            result.brainname,
            result.birthdate.into(),
        ))
    } else {
        None
    }
}
pub async fn get_brainfarts_using_filter(
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
        let mut final_result = vec![];
        let result_iter:  =  result
            .iter()
            .map(|a| async {
                let minds_blown_query = sqlx::query!(
                    "select explosion, brainid from mindsblownbyfarts where brainfartid=$1",
                    a.id
                )
                .fetch_all(&pool)
                .await;
                if let Ok(minds_blown_result) = minds_blown_query {
                    let mut minds_blown = vec![];
                    let mut minds_imploded = vec![];
                    for record in minds_blown_result.iter() {
                        if let Some(brain_info) =
                            get_brain_information(pool, record.brainid.unwrap()).await
                        {
                            if record.explosion.unwrap() {
                                minds_blown.push(brain_info);
                            } else {
                                minds_imploded.push(brain_info);
                            }
                        }
                    }
                    BrainfartInformation {
                        id: a.id.to_string(),
                        title: a.title.clone(),
                        content: a.content.clone(),
                        birthdate: a.birthdate.into(),
                        mastermind_name: a.mastermind_name.clone().unwrap(),
                        blew_minds: minds_blown,
                        imploded_minds: minds_imploded,
                    }
                } else {
                    BrainfartInformation::empty()
                }
            })
            .collect()
        {
            final_result.push(info_future.await);
        }
        Some(final_result)
    } else {
        None
    }
}
