use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn connect_database() -> Pool<MySql> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}
