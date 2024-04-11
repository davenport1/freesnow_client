use anyhow::{Error, Result};

mod client;
mod models;
mod utils;

use client::client::{initialize_client, post_forecasts, retrieve_forecasts};
use models::avalanche_request::AvalanchePostRequest;

// const URL: &str =
//     "https://api.avalanche.org/v2/public/product?type=forecast&center_id=COAA&zone_id=1619";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = initialize_client();
    let forecasts_json = retrieve_forecasts(&client).await.unwrap();
    let mut forecasts_request: Vec<AvalanchePostRequest> = vec![];
    for forecast in forecasts_json.iter() {
        forecasts_request.push(AvalanchePostRequest::from(forecast.clone()))
    }
    let result = post_forecasts(forecasts_request, &client);

    Ok(())
}
