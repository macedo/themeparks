#![warn(clippy::all, clippy::pedantic)]

mod themparks_client;

use clap::Parser;
use serde_json::Value;
use std::collections::HashMap;

mod prelude {
    pub const BASE_URL: &str = "https://api.themeparks.wiki/v1";

    pub use crate::themparks_client::*;
}

use prelude::*;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Arguments {
    #[clap(long, value_parser)]
    filter: Option<String>,
}

async fn get_destinations(
    client: &ThemeparksClient,
    filter: &Option<String>,
) -> Result<Vec<HashMap<String, Value>>, reqwest::Error> {
    let response = client.get("destinations").await?;
    let data: HashMap<String, Value> =
        serde_json::from_str(&response.text().await?).unwrap_or_default();

    let mut destinations: Vec<HashMap<String, Value>> =
        serde_json::from_value(data["destinations"].clone()).unwrap_or_default();

    if let Some(f) = filter {
        let slugs: Vec<&str> = f.split(',').collect();
        destinations = destinations
            .into_iter()
            .filter(|d| slugs.contains(&d["slug"].as_str().unwrap_or_default()))
            .collect();
    }

    Ok(destinations)
}

async fn get_live_data(
    client: &ThemeparksClient,
    entity_id: &String,
) -> Result<Vec<HashMap<String, Value>>, reqwest::Error> {
    let path = format!("entity/{}/live", entity_id);
    let response = client.get(&path).await?;
    let data: HashMap<String, Value> =
        serde_json::from_str(&response.text().await?).unwrap_or_default();

    let live_data: Vec<HashMap<String, Value>> =
        serde_json::from_value(data["liveData"].clone()).unwrap_or_default();

    Ok(live_data)
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let client = ThemeparksClient::new();

    let destinations = get_destinations(&client, &args.filter)
        .await
        .unwrap_or_default();

    let parks: Vec<HashMap<String, Value>> = destinations
        .into_iter()
        .map(|d| {
            let parks: Vec<HashMap<String, Value>> =
                serde_json::from_value(d["parks"].clone()).unwrap();
            parks
        })
        .flatten()
        .collect();

    for park in parks {
        let park_id: String = serde_json::from_value(park["id"].clone()).unwrap_or_default();
        let response = get_live_data(&client, &park_id).await.unwrap();
        println!("{:#?}", response);
    }
}
