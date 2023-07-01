use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
}
