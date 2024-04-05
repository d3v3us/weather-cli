use std::fmt;
use comfy_table::Table;

#[derive(Debug, serde::Deserialize)]
pub struct WeatherApiResponse {
    #[serde(rename = "current")]
    pub weather: Weather,
}


#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    pub city: Option<String>,
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table

        .set_header(vec!["City", "Temperature", "Wind Speed"])
        .add_row(vec![
            self.city.clone().unwrap_or("".to_string()).as_str(),
            self.temperature_2m.to_string().as_str(),
            self.wind_speed_10m.to_string().as_str(),
        ]);
        write!(f, "({})",table)
    }
}
