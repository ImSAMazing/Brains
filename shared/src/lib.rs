use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Brainfart {
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub date: DateTime<Local>,
    pub author: Author,
}

impl Brainfart {
    pub fn create_from_request(id: sqlx::types::Uuid, request: CreateBrainfart) -> Brainfart {
        Brainfart {
            id,
            title: request.title,
            text: request.text,
            date: Local::now(),
            author: request.author,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateBrainfart {
    title: String,
    text: String,
    author: Author,
}

impl CreateBrainfart {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn get_author_name(&self) -> &str {
        &self.author.get_name()
    }
}
#[derive(Deserialize, Serialize)]
pub struct Author {
    username: String,
}

impl Author {
    pub fn get_name(&self) -> &str {
        &self.username
    }
}
