use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateImage {
    pub filename: String,
    pub content_type: String,
    pub size: i64,
}
