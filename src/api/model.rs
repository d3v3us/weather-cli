
#[derive(Debug, serde::Deserialize)]
pub struct WeatherApiResponse {
    pub current: Current,
}


#[derive(Debug, serde::Deserialize)]
pub struct Current {
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
}