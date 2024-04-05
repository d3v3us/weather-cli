pub mod api;
pub mod location;
pub mod cli;

use crate::api::weather_api_client::*;
use crate::location::model::Location;

use crate::cli::Cli;
use api::model::WeatherApiResponse;
use clap::Parser;
use cli::Commands;
#[tokio::main]
async fn main() {    
    let cli = Cli::parse();
   

    match cli.command {
        Commands::GetWeather { city } => {
            let resp = get_weather_handler(&city).await;
            println!("{}",resp.weather)
        }
    }
}

async fn get_weather_handler(city:&str) -> WeatherApiResponse {
    let loc= Location::resolve_location(city).await.unwrap();
    let weather_provider = WeatherApiClient::builder()
        .base_url("https://api.open-meteo.com/v1/forecast")
        .build()
        .unwrap();

     let weather= weather_provider.get_weather_async(&loc).await.unwrap();
    return  weather;
}