use async_trait::async_trait;
use reqwest::Client;
use crate::domain::{
    ports::external_api::CountryApiPort,
    errors::DomainError,
};

pub struct RestCountriesAdapter {
    client: Client,
    base_url: String,
}

impl RestCountriesAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://restcountries.com/v3.1".to_string(),
        }
    }
}

#[async_trait]
impl CountryApiPort for RestCountriesAdapter {
    async fn fetch_country_info(&self, country: &str) -> Result<serde_json::Value, DomainError> {
        let url = format!("{}/name/{}", self.base_url, country);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::ExternalApiError(e.to_string()))?;

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| DomainError::ExternalApiError(e.to_string()))?;

        Ok(data)
    }
}