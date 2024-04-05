use clap::*;

#[derive(Debug, Parser)] // requires `derive` feature
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name ="weather")]
    GetWeather {
        city: String,
    },
}