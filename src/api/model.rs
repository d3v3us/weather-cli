
#[derive(Debug, serde::Deserialize)]
pub struct WeatherApiResponse {
    current: Current,
}


#[derive(Debug, serde::Deserialize)]
struct Current {
    time: String,
    interval: i32,
    temperature_2m: f32,
    wind_speed_10m: f32,
}