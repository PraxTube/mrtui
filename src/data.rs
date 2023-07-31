use std::sync::mpsc;
use std::thread;

use reqwest;
use serde;

use crate::ui::inout;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FeeRecommendation {
    #[serde(rename = "fastestFee")]
    pub fastest_fee: u32,
    #[serde(rename = "halfHourFee")]
    pub half_hour_fee: u32,
    #[serde(rename = "hourFee")]
    pub hour_fee: u32,
    #[serde(rename = "economyFee")]
    pub economy_fee: u32,
    #[serde(rename = "minimumFee")]
    pub minimum_fee: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BlockData {
    pub id: String,
    pub height: u32,
    pub version: u32,
    pub timestamp: u64,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u32,
    pub merkle_root: String,
    pub previousblockhash: String,
    pub mediantime: u64,
    pub nonce: u32,
    pub bits: u32,
    pub difficulty: u64,
}

async fn fetch_response(endpoint_url: &str) -> reqwest::Response {
    let (tx, rx) = mpsc::channel::<bool>();
    let loading_thread = thread::Builder::new()
        .name("loading-animation".into())
        .spawn(move || inout::loading_animation(rx))
        .unwrap();

    let response = match reqwest::get(endpoint_url).await {
        Ok(result) => result,
        Err(err) => {
            tx.send(true).expect("Couldn't send tx.");
            loading_thread.join().unwrap();
            panic!("Couldn't fetch data from url {}", err);
        }
    };

    tx.send(true).expect("Couldn't send tx.");
    loading_thread.join().unwrap();
    response
}

async fn fetch_data<T>(endpoint_url: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let response = fetch_response(endpoint_url).await;
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<T>().await {
            Ok(parsed) => parsed,
            Err(err) => panic!("MISMATCH, shapes don't match, {}", err),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized...");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            panic!("Exceeded rate limits, too many requests.");
        }
        _ => {
            panic!("Not known error happend.");
        }
    }
}

pub async fn fetch_fee() -> FeeRecommendation {
    let endpoint_url = "https://mempool.space/api/v1/fees/recommended";
    fetch_data::<FeeRecommendation>(endpoint_url).await
}

async fn fetch_hash() -> String {
    let endpoint_url = "https://mempool.space/api/blocks/tip/hash";
    let response = fetch_response(endpoint_url).await;

    match response.status() {
        reqwest::StatusCode::OK => match response.text().await {
            Ok(hash) => hash,
            Err(err) => panic!("Couldn't get text from hash {}", err),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized...");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            panic!("Exceeded rate limits, too many requests.");
        }
        _ => {
            panic!("Not known error happend.");
        }
    }
}

pub async fn fetch_block() -> BlockData {
    let hash = fetch_hash().await;
    let endpoint_url = "https://mempool.space/api/block/".to_string() + &hash;
    fetch_data::<BlockData>(&endpoint_url).await
}
