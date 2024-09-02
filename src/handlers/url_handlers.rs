use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    models::url::CreateUrlRequest,
    services::url_service::{UrlService, UrlServiceImpl},
};

pub async fn get_urls(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.get().await {
        Ok(urls) => Ok((StatusCode::OK, Json(urls))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to fetch urls" })),
        )),
    }
}

pub async fn create_url(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUrlRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.add(body).await {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Url created" })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to create url" })),
        )),
    }
}
