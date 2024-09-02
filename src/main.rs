mod db;

use db::connect_database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    connect_database().await;
}
