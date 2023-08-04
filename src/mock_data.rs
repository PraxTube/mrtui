use std::fs::File;
use std::io::Read;

use serde;

use crate::data::{BlockData, HashrateData};

fn fetch_data<T>(mock_url: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let mut file = match File::open(mock_url) {
        Ok(file) => file,
        Err(err) => panic!("Could not open file {}", err),
    };
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    match serde_json::from_str(&contents) {
        Ok(parsed) => return parsed,
        Err(err) => panic!("Failed to parse JSON: {}", err),
    }
}

pub fn fetch_blocks() -> Vec<BlockData> {
    let mock_url = "mock_data/blocks.json";
    fetch_data::<Vec<BlockData>>(mock_url)
}

pub fn fetch_hashrate() -> HashrateData {
    let mock_url = "mock_data/hashrate.json";
    fetch_data::<HashrateData>(mock_url)
}
