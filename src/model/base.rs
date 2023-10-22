use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task{
    pub id: i8,
    pub title: String,
    pub content: String,
}