use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> Result<SqlitePool> {
    let pool = SqlitePool::connect("sqlite:images.db").await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS images (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            created_at DATETIME NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
