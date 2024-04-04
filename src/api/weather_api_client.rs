use crate::api::weather_api_client_builder::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherApiResponse {
    pub temperature: f32,  // in Celsius or Fahrenheit
    pub humidity: f32,     // percentage
    pub wind_speed: f32,   // in meters per second or miles per hour
    pub precipitation: f32,// amount of precipitation in mm or inches
    pub cloud_cover: f32,  // percentage of sky covered by clouds
    pub pressure: f32,     // atmospheric pressure in hPa or inHg
}


pub trait WeatherProvider {
    async fn get_weather_async(&self, 
        lat: f32,
        lon: f32,) -> Result<WeatherApiResponse, reqwest::Error>;
}

pub struct WeatherApiClient{
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
}
impl WeatherApiClient {
    fn builder() -> WeatherApiClientBuilder {
        WeatherApiClientBuilder::new()
    }
}
impl WeatherProvider for WeatherApiClient {
    
    async fn get_weather_async(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<WeatherApiResponse, reqwest::Error> {
        let query_params: &[(&str, &str)] = &[
            ("latitude", &lat.to_string()),
            ("longitude",&lon.to_string()),
            ("current", "temperature_2m,wind_speed_10m"),
            ("hourly", "temperature_2m,relative_humidity_2m,wind_speed_10m"),
        ];

        let res = self
            .client
            .get(&self.base_url)
            .query(query_params)
            .send()
            .await?
            .json::<WeatherApiResponse>()
            .await?;
        Ok(res)
    }
}