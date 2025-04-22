use crate::models::image::{CreateImage, Image};
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ImageService {
    pool: SqlitePool,
}

impl ImageService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_image(&self, image: CreateImage) -> Result<Image> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO images (id, filename, content_type, size, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&image.filename)
        .bind(&image.content_type)
        .bind(image.size)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(Image {
            id,
            filename: image.filename,
            content_type: image.content_type,
            size: image.size,
            created_at: now,
        })
    }

    pub async fn get_image(&self, id: String) -> Result<Option<Image>> {
        let image = sqlx::query_as::<_, Image>(
            r#"
            SELECT id, filename, content_type, size, created_at
            FROM images
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(image)
    }

    pub async fn delete_image(&self, id: String) -> Result<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM images
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
