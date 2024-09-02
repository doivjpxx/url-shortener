use crate::{
    dtos::{CreateUrlRequest, UpdateUrlRequest, UrlResponse},
    models::url::Url,
};

pub struct UrlService {
    pub pool: sqlx::Pool<sqlx::MySql>,
}

pub trait UrlServiceImpl {
    fn new(pool: sqlx::Pool<sqlx::MySql>) -> Self;
    async fn add(&self, data: CreateUrlRequest) -> Result<(), String>;
    async fn retrieve(&self, short_code: String) -> Result<UrlResponse, String>;
    async fn update(&self, short_code: String, data: UpdateUrlRequest) -> Result<(), String>;
    async fn delete(&self, short_code: String) -> Result<(), String>;
    async fn statistics(&self, short_code: String) -> Result<Url, String>;
}

impl UrlServiceImpl for UrlService {
    fn new(pool: sqlx::Pool<sqlx::MySql>) -> Self {
        Self { pool }
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

    async fn retrieve(&self, short_code: String) -> Result<UrlResponse, String> {
        let update_result = sqlx::query!(
            "UPDATE url SET access_count = access_count + 1 WHERE short_code = ?",
            short_code
        )
        .execute(&self.pool)
        .await
        .map_err(|err| format!("Failed to update access count: {:?}", err));

        if let Err(err) = update_result {
            return Err(err);
        }

        let url = sqlx::query_as!(Url, "SELECT * FROM url WHERE short_code = ?", short_code)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| format!("Failed to fetch url: {:?}", err));

        match url {
            Ok(url) => Ok(UrlResponse {
                url: url.url,
                short_code: url.short_code,
                created_at: url.created_at.map(|date| date.to_string()),
                updated_at: url.updated_at.map(|date| date.to_string()),
            }),
            Err(err) => Err(err),
        }
    }

    async fn update(&self, short_code: String, data: UpdateUrlRequest) -> Result<(), String> {
        sqlx::query!(
            "UPDATE url SET url = ?, updated_at = ? WHERE short_code = ?",
            data.url,
            chrono::Utc::now(),
            short_code,
        )
        .execute(&self.pool)
        .await
        .map_err(|err| format!("Failed to update url: {:?}", err))
        .unwrap();

        Ok(())
    }

    async fn delete(&self, short_code: String) -> Result<(), String> {
        sqlx::query!("DELETE FROM url WHERE short_code = ?", short_code)
            .execute(&self.pool)
            .await
            .map_err(|err| format!("Failed to delete url: {:?}", err))
            .unwrap();

        Ok(())
    }

    async fn statistics(&self, short_code: String) -> Result<Url, String> {
        let url = sqlx::query_as!(Url, "SELECT * FROM url WHERE short_code = ?", short_code)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| format!("Failed to fetch url: {:?}", err));

        match url {
            Ok(url) => Ok(url),
            Err(err) => Err(err),
        }
    }
}
