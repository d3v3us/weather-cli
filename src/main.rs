pub mod api;
pub mod location;
pub mod cli;

use crate::api::weather_api_client::*;
use crate::location::model::Location;

use crate::cli::Cli;
use clap::Parser;
use cli::Commands;
#[tokio::main]
async fn main() {    
    let cli = Cli::parse();
    match cli.command {
        Commands::GetWeather { city } => {
           println!("Who")
        }
    }
}

async fn get_weather() {
    let loc= Location::resolve_location("london").await.unwrap();
    let weather_provider = WeatherApiClient::builder()
        .base_url("https://api.open-meteo.com/v1/forecast")
        .build()
        .unwrap();

     let weather= weather_provider.get_weather_async(&loc.lat, &loc.lon).await.unwrap();
     println!("{:?}", weather);
}