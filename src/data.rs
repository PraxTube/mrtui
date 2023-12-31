use std::sync::mpsc;
use std::thread;

use reqwest;
use serde;

use crate::ui::inout;

#[derive(Debug, serde::Deserialize)]
pub struct BlockData {
    pub id: String,
    pub height: u32,
    pub version: u32,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub difficulty: f64,
    pub merkle_root: String,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u32,
    pub previousblockhash: String,
    pub mediantime: u32,
    pub extras: Extras,
}

#[derive(Debug, serde::Deserialize)]
pub struct Extras {
    #[serde(rename = "totalFees")]
    pub total_fees: u32,
    #[serde(rename = "medianFee")]
    pub median_fee: f64,
    #[serde(rename = "feeRange")]
    pub fee_range: Vec<f64>,
    pub reward: u32,
    pub pool: Pool,
    #[serde(rename = "avgFee")]
    pub avg_fee: u32,
    #[serde(rename = "avgFeeRate")]
    pub avg_fee_rate: u32,
    #[serde(rename = "coinbaseRaw")]
    pub coinbase_raw: String,
    #[serde(rename = "coinbaseAddress")]
    pub coinbase_address: String,
    #[serde(rename = "coinbaseSignature")]
    pub coinbase_signature: String,
    #[serde(rename = "coinbaseSignatureAscii")]
    pub coinbase_signature_ascii: String,
    #[serde(rename = "avgTxSize")]
    pub avg_tx_size: f64,
    #[serde(rename = "totalInputs")]
    pub total_inputs: u32,
    #[serde(rename = "totalOutputs")]
    pub total_outputs: u32,
    #[serde(rename = "totalOutputAmt")]
    pub total_output_amt: u64,
    #[serde(rename = "feePercentiles")]
    pub fee_percentiles: Option<Vec<u32>>,
    #[serde(rename = "segwitTotalTxs")]
    pub segwit_total_txs: u32,
    #[serde(rename = "segwitTotalSize")]
    pub segwit_total_size: u32,
    #[serde(rename = "segwitTotalWeight")]
    pub segwit_total_weight: u32,
    pub header: String,
    #[serde(rename = "utxoSetChange")]
    pub utxo_set_change: i32,
    #[serde(rename = "utxoSetSize")]
    pub utxo_set_size: Option<u32>,
    #[serde(rename = "totalInputAmt")]
    pub total_input_amt: Option<u64>,
    #[serde(rename = "virtualSize")]
    pub virtual_size: f64,
    pub orphans: Vec<serde_json::Value>,
    #[serde(rename = "matchRate")]
    pub match_rate: Option<serde_json::Value>,
    #[serde(rename = "expectedFees")]
    pub expected_fees: Option<serde_json::Value>,
    #[serde(rename = "expectedWeight")]
    pub expected_weight: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pool {
    pub id: u32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct HashrateData {
    pub hashrates: Vec<HashrateEntry>,
    pub difficulty: Vec<DifficultyEntry>,
    #[serde(rename = "currentHashrate")]
    pub current_hashrate: u128,
    #[serde(rename = "currentDifficulty")]
    pub current_difficulty: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct HashrateEntry {
    pub timestamp: u64,
    #[serde(rename = "avgHashrate")]
    pub avg_hashrate: u128,
}

#[derive(Debug, serde::Deserialize)]
pub struct DifficultyEntry {
    pub time: u64,
    pub height: u64,
    pub difficulty: f64,
    pub adjustment: f64,
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
            panic!(
                "couldn't fetch data from url, most likely timed out\n{}",
                err
            );
        }
    };

    tx.send(true).expect("couldn't send tx to rx");
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

pub async fn fetch_blocks(block_height: Option<usize>) -> Vec<BlockData> {
    let endpoint_url = match block_height {
        Some(height) => format!(
            "https://mempool.space/api/v1/blocks/{}",
            &height.to_string()
        ),
        None => "https://mempool.space/api/v1/blocks".to_string(),
    };
    fetch_data::<Vec<BlockData>>(&endpoint_url).await
}

pub async fn fetch_hashrate() -> HashrateData {
    let endpoint_url = "https://mempool.space/api/v1/mining/hashrate/3m";
    fetch_data::<HashrateData>(&endpoint_url).await
}
