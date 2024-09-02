mod app;
mod db;
mod handlers;
mod models;
mod services;

use app::app;
use db::connect_database;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = connect_database().await;

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap_or("0.0.0.0".to_owned()),
        env::var("PORT").unwrap_or("3000".to_owned())
    );

    let app = app(pool);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server is running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
