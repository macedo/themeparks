#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::all, clippy::pedantic)]

mod client;

mod prelude {
    pub const BASE_URL: &'static str = "https://api.themeparks.wiki/v1";
    pub use reqwest::header;
    pub use crate::client::*;
}

use clap::Parser;
use serde::Deserialize;
use prelude::*;


#[derive(clap::Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Destinations,
}

#[derive(Deserialize, Debug)]
struct DestinationsApiResponse {
    destinations: Vec<Destination>,
}

#[derive(Deserialize, Debug)]
struct Destination {
    id: String,
    name: String,
    slug: String,
    parks: Vec<Park>,
}

#[derive(Deserialize, Debug)]
struct Park {
    id: String,
    name: String,
}

async fn get_destinations(client: reqwest::Client) -> Result<Vec<Destination>, reqwest::Error> {
    let url = format!("{}/{}", BASE_URL, "destinations");
    println!("fetching {}", url);

    let response = client.get(&url).send().await?;
    let data = response.json::<DestinationsApiResponse>().await?;

    Ok(data.destinations)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Arguments::parse();
    match args.action {
        Action::Destinations => {
            let client = Client::new()?;

            get_destinations(client).await?;
        }
    }
    Ok(())
}
