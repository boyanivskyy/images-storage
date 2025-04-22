use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

use crate::{
    models::image::{CreateImage, Image},
    services::image_service::ImageService,
};

pub fn create_routes(image_service: ImageService) -> Router {
    Router::new()
        .route("/images", post(create_image))
        .route("/images/:id", get(get_image))
        .route("/images/:id", delete(delete_image))
        .with_state(image_service)
}

#[axum::debug_handler]
async fn create_image(
    State(image_service): State<ImageService>,
    Json(image): Json<CreateImage>,
) -> Result<Json<Image>, StatusCode> {
    image_service
        .create_image(image)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[axum::debug_handler]
async fn get_image(
    State(image_service): State<ImageService>,
    Path(id): Path<String>,
) -> Result<Json<Image>, StatusCode> {
    image_service
        .get_image(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[axum::debug_handler]
async fn delete_image(
    State(image_service): State<ImageService>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match image_service.delete_image(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
