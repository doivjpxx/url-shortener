use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: i32,
    pub url: String,
    pub short_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub access_count: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub short_code: String,
}
