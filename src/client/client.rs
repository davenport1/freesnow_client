// Client
// Handles requests and responses to the avalanche center api and freesnow_server
//

use anyhow::Error;

use crate::models::{
    avalanche_json::AvalancheForecastJson, avalanche_request::AvalanchePostRequest,
};

use super::super::models::{avalanche_json, avalanche_request};

const URLS: [&str; 4] = [
    "https://api.avalanche.org/v2/public/product?type=forecast&center_id=COAA&zone_id=1619",
    "https://api.avalanche.org/v2/public/product?type=forecast&center_id=BAC&zone_id=1350",
    "https://api.avalanche.org/v2/public/product?type=forecast&center_id=MSAC&zone_id=1432",
    "https://api.avalanche.org/v2/public/product?type=forecast&center_id=SAC&zone_id=1605",
];

const POSTURL: &str = "http://localhost:8000/avalanche/forecast";

pub enum AvalancheZoneIds {
    CentralOregon = 1619,
    Bridgeport = 1350,
    Shasta = 1432,
    Sierra = 1605,
}

impl AvalancheZoneIds {
    pub fn get_identifier_st(&self) -> &str {
        match self {
            Self::CentralOregon => return "COAA",
            Self::Bridgeport => return "BAC",
            Self::Shasta => return "MSAC",
            Self::Sierra => return "SAC",
        }
    }
}

pub async fn retrieve_forecasts(
    client: &reqwest::Client,
) -> Result<Vec<avalanche_json::AvalancheForecastJson>, reqwest::Error> {
    let mut forecasts: Vec<AvalancheForecastJson> = vec![];

    for url in URLS.iter() {
        let response = client.get(url.to_string()).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                match response
                    .json::<avalanche_json::AvalancheForecastJson>()
                    .await
                {
                    Ok(parsed) => forecasts.push(parsed.clone()),
                    Err(_) => println!("response didnt match"),
                };
            }
            _other => {
                panic!("Internal Server Error");
            }
        }
    }

    return Ok(forecasts);
}

pub async fn post_forecasts(
    forecasts: Vec<AvalanchePostRequest>,
    client: &reqwest::Client,
) -> Result<reqwest::StatusCode, reqwest::Error> {
    let json_body = serde_json::to_string(&forecasts).unwrap();

    let response = client
        .post(POSTURL)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_body)
        .send()
        .await?;

    Ok(response.status())
}

pub fn initialize_client() -> reqwest::Client {
    reqwest::Client::new()
}
