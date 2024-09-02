use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::url_handlers::{
    create_url, delete_url, get_stats_url, retrieve_url, update_url,
};

pub struct AppState {
    pub pool: sqlx::Pool<sqlx::MySql>,
}

pub fn app(pool: sqlx::Pool<sqlx::MySql>) -> Router {
    let app: Router = Router::new()
        .route("/shorten", post(create_url))
        .route(
            "/shorten/:code",
            get(retrieve_url).put(update_url).delete(delete_url),
        )
        .route("/shorten/:code/stats", get(get_stats_url))
        .with_state(Arc::new(AppState { pool: pool.clone() }));
    app
}
