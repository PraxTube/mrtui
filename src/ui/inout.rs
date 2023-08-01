use crate::data::{BlockData, FeeRecommendation};
use crate::utils;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::ui::bar;
use crate::ui::block;

const WIDTH: usize = 20;
const HEIGHT: usize = 10;
const DIFFICULTY_ADJUSTMENT: u32 = 2016;
const HALVING: u32 = 210_000;

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

pub fn loading_animation(rx: Receiver<bool>) {
    let frames = vec!["|", "/", "-", "\\"];
    let mut index = 0;

    loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => {}
        };
        print!("\rFetching data {} ", frames[index]);
        io::stdout().flush().unwrap();
        index = (index + 1) % frames.len();
        std::thread::sleep(Duration::from_millis(100));
    }
    print!("\r");
    io::stdout().flush().unwrap();
}

pub fn print_fee(fees: FeeRecommendation) {
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

pub fn print_block(block: &BlockData) {
    println!("{}", block::render(WIDTH, HEIGHT, block));
}

pub fn print_difficulty(block: &BlockData) {
    let width = 50;
    let height = 1;
    let seconds_till_adjustment =
        (DIFFICULTY_ADJUSTMENT - block.height % DIFFICULTY_ADJUSTMENT) as u64 * 10 * 60;
    let progress = (block.height % DIFFICULTY_ADJUSTMENT) as f64 / DIFFICULTY_ADJUSTMENT as f64;
    let title = format!(
        "Difficulty Adjustment in: {}, {:.0}%",
        utils::format_time(seconds_till_adjustment),
        progress * 100.0
    );

    let progress_box = bar::boxed_bar(width, height, progress, title);
    println!("{}", progress_box);
}

pub fn print_halving(block: &BlockData) {
    let seconds_till_halving = (HALVING - block.height % HALVING) as u64 * 10 * 60;
    let width = 50;
    let height = 1;
    let progress = (block.height % HALVING) as f64 / HALVING as f64;
    let title = format!(
        "Halving in: {}, {:.0}%",
        utils::format_time(seconds_till_halving),
        progress * 100.0
    );

    let progress_box = bar::boxed_bar(width, height, progress, title);
    println!("{}", progress_box);
}
