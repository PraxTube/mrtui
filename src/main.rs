use reqwest::{self, Response};
use serde;

mod utils;

use utils::div_up;

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

fn positive_response(response: &Response) -> bool {
    match response.status() {
        reqwest::StatusCode::OK => return true,
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Unauthorized...");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("Exceeded rate limits, too many requests.");
        }
        _ => {
            panic!("Not known error happend.");
        }
    }
    false
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
    println!("{}", render_box(15, 10, block));
}

async fn do_fee() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint_url = "https://mempool.space/api/v1/fees/recommended";
    println!("Fetching data...");
    let response = reqwest::get(endpoint_url).await?;

    if positive_response(&response) {
        match response.json::<FeeRecommendation>().await {
            Ok(parsed) => print_fee(parsed),
            Err(err) => println!("MISMATCH, didn't match expected shape.\n{}", err),
        };
    }
    Ok(())
}

async fn get_hash() -> Result<String, Box<dyn std::error::Error>> {
    let endpoint_hash = "https://mempool.space/api/blocks/tip/hash";
    println!("Fetching data...");
    let response = reqwest::get(endpoint_hash).await?;

    if positive_response(&response) {
        let hash = response.text().await?;
        return Ok(hash);
    }
    Ok(String::new())
}

async fn do_block() -> Result<(), Box<dyn std::error::Error>> {
    let hash = get_hash().await?;
    let endpoint_url = "https://mempool.space/api/block/".to_string() + &hash;
    println!("Fetching data...");
    let response = reqwest::get(endpoint_url).await?;

    if positive_response(&response) {
        match response.json::<BlockData>().await {
            Ok(parsed) => print_block(parsed),
            Err(err) => println!("MISTMATCH, didn't match the shape expected.\n{}", err),
        };
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_fee().await?;
    do_block().await?;
    Ok(())
}

fn write_row(message: &mut Vec<Vec<char>>, row: usize, content: &str) {
    let width = message[0].len();
    if width - 2 < content.chars().count() {
        panic!("Width is too small.");
    }

    let mut index = width / 2 - div_up(content.chars().count(), 2);

    for char in content.chars() {
        index += 1;
        message[row][index] = char;
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

    write_row(&mut message, 3, &format!("{}", block.size));

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
            result.push(message[i][j]);
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
