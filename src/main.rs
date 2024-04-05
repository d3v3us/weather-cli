pub mod api;
pub mod location;
pub mod cli;

use crate::api::weather_api_client::*;
use crate::location::model::Location;
use comfy_table::Table;

use crate::cli::Cli;
use api::model::WeatherApiResponse;
use clap::Parser;
use cli::Commands;
#[tokio::main]
async fn main() {    
    let cli = Cli::parse();
    let mut table = Table::new();

    match cli.command {
        Commands::GetWeather { city } => {
            let weather = get_weather(&city).await;
            table

            .set_header(vec!["City", "Temperature", "Wind Speed"])
            .add_row(vec![
                city.as_str(),
                weather.current.temperature_2m.to_string().as_str(),
                weather.current.wind_speed_10m.to_string().as_str(),
            ]);
    
        println!("\n\n{table}");
        }
    }
}

async fn get_weather(city:&str) -> WeatherApiResponse {
    let loc= Location::resolve_location(city).await.unwrap();
    let weather_provider = WeatherApiClient::builder()
        .base_url("https://api.open-meteo.com/v1/forecast")
        .build()
        .unwrap();

     let weather= weather_provider.get_weather_async(&loc.lat, &loc.lon).await.unwrap();
    return  weather;
}