use crate::api::model::*;
use crate::api::weather_api_client_builder::*;
use crate::location;

pub trait WeatherProvider {
    fn get_weather_async(&self, 
        loc: &location::model::Location,
    ) -> impl std::future::Future<Output = Result<WeatherApiResponse, reqwest::Error>> + Send;
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
       loc: &location::model::Location,
    ) -> Result<WeatherApiResponse, reqwest::Error> {
        let query_params: &[(&str, &str)] = &[
            ("latitude", &loc.lat),
            ("longitude",&loc.lat),
            ("current", "temperature_2m,wind_speed_10m"),
            ("hourly", "temperature_2m,relative_humidity_2m,wind_speed_10m"),
        ];

        let mut res = self
            .client
            .get(&self.base_url)
            .query(query_params)
            .send()
            .await?
            .json::<WeatherApiResponse>()
            .await?;
        res.weather.city=Some(loc.name.clone());
        Ok(res)
    }
}