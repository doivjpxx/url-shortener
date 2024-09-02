use crate::models::url::{CreateUrlRequest, Url};

pub struct UrlService {
    pub pool: sqlx::Pool<sqlx::MySql>,
}

pub trait UrlServiceImpl {
    fn new(pool: sqlx::Pool<sqlx::MySql>) -> Self;
    async fn add(&self, data: CreateUrlRequest) -> Result<(), String>;
    async fn get(&self) -> Result<Vec<Url>, String>;
}

impl UrlServiceImpl for UrlService {
    fn new(pool: sqlx::Pool<sqlx::MySql>) -> Self {
        Self { pool }
    }

    async fn get(&self) -> Result<Vec<Url>, String> {
        let urls = sqlx::query_as!(Url, "SELECT * FROM url")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| format!("Failed to fetch urls: {:?}", err))
            .unwrap();

        Ok(urls)
    }

    async fn add(&self, data: CreateUrlRequest) -> Result<(), String> {
        sqlx::query_as!(
            Url,
            "INSERT INTO url (url, short_code) VALUES (?, ?)",
            data.url,
            data.short_code
        )
        .execute(&self.pool)
        .await
        .map_err(|err| format!("Failed to create url: {:?}", err))
        .unwrap();

        Ok(())
    }
}
