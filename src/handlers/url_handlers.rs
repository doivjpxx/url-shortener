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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect_database;

    #[tokio::test]
    async fn test_create_url() {
        dotenvy::dotenv().ok();

        let pool = connect_database().await;
        let app_state = Arc::new(AppState { pool });

        let response = create_url(
            State::from(axum::extract::State(Arc::clone(&app_state))),
            Json(CreateUrlRequest {
                url: "https://example.com".to_string(),
                short_code: "example".to_string(),
            }),
        )
        .await;

        assert_eq!(response.is_ok(), true);
    }

    #[tokio::test]
    async fn test_retrieve_url() {
        dotenvy::dotenv().ok();

        let pool = connect_database().await;
        let app_state = Arc::new(AppState { pool });

        let response = retrieve_url(
            State::from(axum::extract::State(Arc::clone(&app_state))),
            Path("example".to_string()),
        )
        .await;

        assert_eq!(response.is_ok(), true);
    }

    #[tokio::test]
    async fn test_update_url() {
        dotenvy::dotenv().ok();

        let pool = connect_database().await;
        let app_state = Arc::new(AppState { pool });

        let response = update_url(
            State::from(axum::extract::State(Arc::clone(&app_state))),
            Path("example".to_string()),
            Json(UpdateUrlRequest {
                url: "https://example.com".to_string(),
            }),
        )
        .await;

        assert_eq!(response.is_ok(), true);
    }

    #[tokio::test]
    async fn test_get_stats_url() {
        dotenvy::dotenv().ok();

        let pool = connect_database().await;
        let app_state = Arc::new(AppState { pool });

        let response = get_stats_url(
            State::from(axum::extract::State(Arc::clone(&app_state))),
            Path("example".to_string()),
        )
        .await;

        assert_eq!(response.is_ok(), true);
    }

    #[tokio::test]
    async fn test_delete_url() {
        dotenvy::dotenv().ok();

        let pool = connect_database().await;
        let app_state = Arc::new(AppState { pool });

        let response = delete_url(
            State::from(axum::extract::State(Arc::clone(&app_state))),
            Path("example".to_string()),
        )
        .await;

        assert_eq!(response.is_ok(), true);
    }
}
