use crate::api::weather_api_client::*;
pub struct WeatherApiClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
}

impl WeatherApiClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
        }
    }
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    pub fn build(self) -> Result<WeatherApiClient, String> {
        let base_url = self.base_url.ok_or("Base URL is required")?;
        let api_key = self.api_key.ok_or("API key is required")?;

        Ok(WeatherApiClient {
            client: reqwest::Client::new(),
            base_url,
        })
    }
}
