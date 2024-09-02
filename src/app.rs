use axum::{routing::get, Router};

pub fn app() -> Router {
    let app: Router = Router::new().route("/", get(|| async { "Hello, world!" }));
    app
}
