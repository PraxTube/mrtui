use crate::data::{BlockData, FeeRecommendation};
use crate::utils;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::ui::bar;
use crate::ui::block;
use crate::ui::utils::stylize_string;

const WIDTH: usize = 20;
const HEIGHT: usize = 10;
const HFILL: &str = "     ";

const DIFFICULTY_ADJUSTMENT: u32 = 2016;
const HALVING: u32 = 210_000;

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
        utils::format_number_kilo(fees.fastest_fee * 140 * 3, "_"),
        utils::format_number_kilo(fees.minimum_fee * 140, "_"),
    ));
    println!("{}", expected_fee);
}

pub fn print_blocks(blocks: &Vec<BlockData>) {
    let blocks_str: Vec<Vec<String>> = blocks
        .iter()
        .map(|b| block::render(WIDTH, HEIGHT, b))
        .collect();

    let mut first_result: Vec<String> = vec![String::new(); blocks_str[0].len()];

    for i in 0..blocks_str[0].len() {
        let mut row = String::new();
        for k in 0..4 {
            row.push_str(&blocks_str[k][i]);
            row.push_str(HFILL);
        }
        first_result[i].push_str(&row);
    }

    let mut second_result: Vec<String> = vec![String::new(); blocks_str[0].len()];

    for i in 0..blocks_str[0].len() {
        let mut row = String::new();
        for k in 4..8 {
            row.push_str(&blocks_str[k][i]);
            row.push_str(HFILL);
        }
        second_result[i].push_str(&row);
    }

    println!();
    for line in first_result {
        println!("{}", line);
    }
    println!();
    for line in second_result {
        println!("{}", line);
    }
    println!();
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
