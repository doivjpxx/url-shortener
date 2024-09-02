use std::sync::Arc;

use axum::{routing::get, Router};

use crate::handlers::url_handlers::{create_url, get_urls};

pub struct AppState {
    pub pool: sqlx::Pool<sqlx::MySql>,
}

pub fn app(pool: sqlx::Pool<sqlx::MySql>) -> Router {
    let app: Router = Router::new()
        .route("/urls", get(get_urls).post(create_url))
        .with_state(Arc::new(AppState { pool: pool.clone() }));
    app
}
