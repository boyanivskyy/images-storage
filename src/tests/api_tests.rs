use crate::{
    api::routes::create_routes, db::init_db, models::image::CreateImage,
    services::image_service::ImageService,
};
use axum::http::StatusCode;
use reqwest::Client;
use sqlx::SqlitePool;
use tempfile::tempdir;
use uuid::Uuid;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

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
    .await
    .unwrap();

    pool
}

#[tokio::test]
async fn test_create_and_get_image() {
    let pool = setup_test_db().await;
    let image_service = ImageService::new(pool);
    let app = create_routes(image_service);

    let client = Client::new();
    let image = CreateImage {
        filename: "test.jpg".to_string(),
        content_type: "image/jpeg".to_string(),
        size: 1024,
    };

    let response = client
        .post("http://localhost:3000/images")
        .json(&image)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let created_image: serde_json::Value = response.json().await.unwrap();
    let image_id = created_image["id"].as_str().unwrap();

    let response = client
        .get(&format!("http://localhost:3000/images/{}", image_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let retrieved_image: serde_json::Value = response.json().await.unwrap();
    assert_eq!(retrieved_image["filename"], "test.jpg");
}

#[tokio::test]
async fn test_delete_image() {
    let pool = setup_test_db().await;
    let image_service = ImageService::new(pool);
    let app = create_routes(image_service);

    let client = Client::new();
    let image = CreateImage {
        filename: "test.jpg".to_string(),
        content_type: "image/jpeg".to_string(),
        size: 1024,
    };

    let response = client
        .post("http://localhost:3000/images")
        .json(&image)
        .send()
        .await
        .unwrap();

    let created_image: serde_json::Value = response.json().await.unwrap();
    let image_id = created_image["id"].as_str().unwrap();

    let response = client
        .delete(&format!("http://localhost:3000/images/{}", image_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client
        .get(&format!("http://localhost:3000/images/{}", image_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
