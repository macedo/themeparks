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

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Arguments {
    #[clap(long, value_parser)]
    filter: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DestinationsApiResponse {
    destinations: Vec<Destination>,
}

#[derive(Deserialize, Debug)]
struct Destination {
    slug: String,
    parks: Vec<Park>,
}

#[derive(Deserialize, Debug)]
struct Park {}

async fn get_destinations(
    client: reqwest::Client,
    filter: &Option<String>
) -> Result<Vec<Destination>, reqwest::Error> {
    let url = format!("{}/{}", BASE_URL, "destinations");
    println!("fetching {}", url);

    let response = client.get(&url).send().await?;
    let data = response.json::<DestinationsApiResponse>().await?;

    let mut destinations = data.destinations;

    if let Some(f) = filter {
        let slugs: Vec<&str> = f.split(",").collect();
        destinations = destinations
            .into_iter()
            .filter(|d| slugs.contains(&d.slug.as_str()))
            .collect::<Vec<Destination>>();
    }

    Ok(destinations)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Arguments::parse();
    let client = Client::new()?;
    let destinations = get_destinations(client, &args.filter).await?;
    println!("{:?}", destinations);
    Ok(())
}
