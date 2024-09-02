use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub short_code: String,
}

#[derive(Deserialize)]
pub struct UpdateUrlRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub url: String,
    pub short_code: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}