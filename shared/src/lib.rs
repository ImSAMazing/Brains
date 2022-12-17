use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Brainfart {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub date: DateTime<Local>,
    pub author: Author,
}

impl Brainfart {
    pub fn create_from_request(id: u32, request: CreateBrainfart) -> Brainfart {
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

#[derive(Deserialize, Serialize)]
pub struct Author {
    username: String,
}
