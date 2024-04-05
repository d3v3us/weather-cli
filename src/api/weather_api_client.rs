use crate::api::model::*;
use crate::api::weather_api_client_builder::*;

pub trait WeatherProvider {
    fn get_weather_async(&self, 
        lat: &str,
        lon: &str,) -> impl std::future::Future<Output = Result<WeatherApiResponse, reqwest::Error>> + Send;
}

pub struct WeatherApiClient{
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
}
impl WeatherApiClient {
   pub fn builder() -> WeatherApiClientBuilder {
        WeatherApiClientBuilder::new()
    }
}
impl WeatherProvider for WeatherApiClient {
    
    async fn get_weather_async(
        &self,
        lat: &str,
        lon: &str,
    ) -> Result<WeatherApiResponse, reqwest::Error> {
        let query_params: &[(&str, &str)] = &[
            ("latitude", &lat),
            ("longitude",&lon),
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