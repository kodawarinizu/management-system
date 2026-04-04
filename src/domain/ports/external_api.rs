use async_trait::async_trait;
use crate::domain::errors::DomainError;


#[async_trait]
pub trait CountryApiPort: Send + Sync {
    async fn fetch_country_info (&self, country: &str) -> Result<serde_json::Value, DomainError>;
}