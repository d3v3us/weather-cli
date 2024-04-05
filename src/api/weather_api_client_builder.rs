use crate::api::weather_api_client::*;

pub struct WeatherApiClientBuilder {
    base_url: Option<String>,
}

impl WeatherApiClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
        }
    }
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn build(self) -> Result<WeatherApiClient, String> {
        let base_url = self.base_url.ok_or("Base URL is required")?;

        Ok(WeatherApiClient {
            client: reqwest::Client::new(),
            base_url,
        })
    }
}
