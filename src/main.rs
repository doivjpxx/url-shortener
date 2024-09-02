mod app;
mod db;
mod models;

use app::app;
use db::connect_database;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    connect_database().await;

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap_or("0.0.0.0".to_owned()),
        env::var("PORT").unwrap_or("3000".to_owned())
    );

    let app = app();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server is running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
