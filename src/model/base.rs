use serde::Serialize;

#[derive(Serialize)]
pub struct Task{
    pub id: i8,
    pub title: String,
    pub content: String,
}