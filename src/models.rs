use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}
