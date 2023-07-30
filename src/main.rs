use std::fs::File;
use std::io::Read;
use std::panic;

use reqwest::{self};
use serde;

mod utils;

use utils::div_up;

const MOCK_DATA: bool = true;
const BLOCK_FILL_COLOR_INDEX: u8 = 69;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct FeeRecommendation {
    #[serde(rename = "fastestFee")]
    fastest_fee: u32,
    #[serde(rename = "halfHourFee")]
    half_hour_fee: u32,
    #[serde(rename = "hourFee")]
    hour_fee: u32,
    #[serde(rename = "economyFee")]
    economy_fee: u32,
    #[serde(rename = "minimumFee")]
    minimum_fee: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BlockData {
    id: String,
    height: u32,
    version: u32,
    timestamp: u64,
    tx_count: u32,
    size: u32,
    weight: u32,
    merkle_root: String,
    previousblockhash: String,
    mediantime: u64,
    nonce: u32,
    bits: u32,
    difficulty: u64,
}

fn stylize_string(input_str: &str) -> String {
    fn hex_to_asni_str(hex_str: &str) -> String {
        if let Ok(color) = u32::from_str_radix(&hex_str.replace("#", ""), 16) {
            let red = (color >> 16) & 0xFF;
            let green = (color >> 8) & 0xFF;
            let blue = color & 0xFF;

            return format!("{};{};{}", red, green, blue);
        }
        "FORMAT ERROR".to_string()
    }

    let mut output_str: String = input_str
        .replace("[bold]", "\x1b[1m")
        .replace("[/bold]", "\x1b[22m")
        .replace("[italic]", "\x1b[3m")
        .replace("[/italic]", "\x1b[23m")
        .replace("[underline]", "\x1b[4m")
        .replace("[/underline]", "\x1b[24m")
        .replace("[blink]", "\x1b[5m")
        .replace("[/blink]", "\x1b[25m")
        .replace("[invert]", "\x1b[7m")
        .replace("[/invert]", "\x1b[27m")
        .replace("[crossout]", "\x1b[9m")
        .replace("[crossout]", "\x1b[29m")
        .replace("[strike]", "\x1b[9m")
        .replace("[/strike]", "\x1b[29m")
        .replace("[/color]", "\x1b[39m");

    let start_tag = "[color=";
    let end_tag = "]";
    let mut current_index = 0;

    while let Some(start_index) = output_str[current_index..].find(start_tag) {
        let adjusted_start_index = current_index + start_index + start_tag.len();

        if let Some(end_index) = output_str[adjusted_start_index..].find(end_tag) {
            let hex_str = &output_str[adjusted_start_index..(adjusted_start_index + end_index)];
            output_str = output_str.replace(
                &format!("{}{}{}", start_tag, hex_str, end_tag),
                &format!("\x1b[38;2;{}m", hex_to_asni_str(hex_str)),
            );
            current_index = adjusted_start_index + end_index;
        } else {
            break;
        }
    }
    output_str
}

fn print_fee(fees: FeeRecommendation) {
    let fee_str = stylize_string(&format!(
        "Fees in sats/vB\n\
            [color=#BF0000]\u{2191} High:[/color] {}, \
            \u{2248} Medium: {}, \
            [color=#009F4F]\u{2193} Low:[/color] {}",
        fees.fastest_fee, fees.hour_fee, fees.minimum_fee
    ));
    println!("{}", fee_str);
    let expected_fee = stylize_string(&format!(
        "\nExpected fees in sats:\n\
            [color=#BF0000]High end:[/color] {}, [color=#009F4F]Low end:[/color] {}",
        fees.fastest_fee * 600,
        fees.minimum_fee * 200
    ));
    println!("{}", expected_fee);
}

fn print_block(block: BlockData) {
    println!("{}", render_box(20, 10, block));
}

async fn fetch_data<T>(endpoint_url: &str, mock_url: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    if MOCK_DATA {
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

    println!("Fetching data...");
    let response = match reqwest::get(endpoint_url).await {
        Ok(result) => result,
        Err(err) => panic!("Couldn't fetch data from url {}", err),
    };

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

async fn do_fee() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = "https://mempool.space/api/v1/fees/recommended";
    let mock_url = "mock_data/fee.json";
    print_fee(fetch_data::<FeeRecommendation>(endpoint_url, mock_url).await);
    Ok(())
}

async fn get_hash() -> Result<String, Box<dyn std::error::Error>> {
    if MOCK_DATA {
        let mut file = File::open("mock_data/hash.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(contents);
    }

    let endpoint_url = "https://mempool.space/api/blocks/tip/hash";
    println!("Fetching data...");
    let response = match reqwest::get(endpoint_url).await {
        Ok(result) => result,
        Err(err) => panic!("Couldn't fetch data from url {}", err),
    };

    match response.status() {
        reqwest::StatusCode::OK => match response.text().await {
            Ok(hash) => Ok(hash),
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

async fn do_block() -> Result<(), Box<dyn std::error::Error>> {
    let hash = get_hash().await?;
    let endpoint_url = "https://mempool.space/api/block/".to_string() + &hash;
    let mock_url = "mock_data/block.json";
    print_block(fetch_data::<BlockData>(&endpoint_url, mock_url).await);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_fee().await?;
    do_block().await?;
    Ok(())
}

fn write_row(message: &mut Vec<Vec<char>>, row: i32, content: &str) {
    let width = message[0].len();
    if width - 2 < content.chars().count() {
        panic!("Width is too small.");
    }

    let row: usize = match row < 0 {
        true => message.len() - row.wrapping_abs() as usize,
        false => row.wrapping_abs() as usize,
    };

    let mut index = div_up(width, 2) - div_up(content.chars().count(), 2);

    for char in content.chars() {
        message[row][index] = char;
        index += 1;
    }
}

fn render_box(width: usize, height: usize, block: BlockData) -> String {
    if width < 2 || height < 2 {
        panic!("Width and height must be at least 2.");
    }
    let bheight = block.height.to_string().into_bytes();
    if width < bheight.len() + 2 {
        panic!("Width must be bigger then the number of digits in block height");
    }

    let title_start_index = width / 2 - div_up(bheight.len(), 2);
    let mut result = String::new();
    let mut message = vec![vec![' '; width]; height];
    let block_fill = block.weight as f32 / 4_000_000 as f32;

    write_row(
        &mut message,
        3,
        &format!("{}", utils::format_number_bytes(block.size)),
    );
    write_row(&mut message, 4, &format!("{} txs", block.tx_count));
    write_row(&mut message, -2, &utils::minute_difference(block.timestamp));

    // Top border
    result.push('╭');
    for i in 1..width - 1 {
        if i >= title_start_index && i < title_start_index + bheight.len() {
            result.push(bheight[i - title_start_index] as char);
        } else {
            result.push('─');
        }
    }
    result.push('╮');
    result.push('\n');

    // Middle rows
    for i in 1..height - 1 {
        result.push('│');

        for j in 1..width - 1 {
            let percentage = 1.0 - i as f32 / height as f32;
            if block_fill > percentage {
                let content = format!(
                    "\x1b[48;5;{}m{}\x1b[49m",
                    BLOCK_FILL_COLOR_INDEX, message[i][j]
                );
                result += &content;
            } else {
                result.push(message[i][j]);
            }
        }
        result.push('│');
        result.push('\n');
    }

    // Bottom border
    result.push('╰');
    for _ in 1..width - 1 {
        result.push('─');
    }
    result.push('╯');

    result
}
