use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    dtos::{CreateUrlRequest, UpdateUrlRequest},
    services::url_service::{UrlService, UrlServiceImpl},
};

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

pub async fn retrieve_url(
    State(data): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.retrieve(code).await {
        Ok(url) => Ok((StatusCode::OK, Json(url))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to fetch url" })),
        )),
    }
}

pub async fn update_url(
    State(data): State<Arc<AppState>>,
    Path(code): Path<String>,
    Json(body): Json<UpdateUrlRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.update(code, body).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Url updated" })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to update url" })),
        )),
    }
}

pub async fn get_stats_url(
    State(data): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.statistics(code).await {
        Ok(url) => Ok((StatusCode::OK, Json(url))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to fetch url" })),
        )),
    }
}

pub async fn delete_url(
    State(data): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_service = UrlService::new(data.pool.clone());

    match url_service.delete(code).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Url deleted" })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": "Failed to delete url" })),
        )),
    }
}