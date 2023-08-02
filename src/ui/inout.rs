use crate::data::BlockData;
use crate::utils;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::ui::bar;
use crate::ui::block;
use crate::ui::utils::concat_strings;

const WIDTH: usize = 20;
const HEIGHT: usize = 10;
const HFILL: &str = "  ";

const DIFFICULTY_ADJUSTMENT: u32 = 2016;
const HALVING: u32 = 210_000;

pub fn loading_animation(rx: Receiver<bool>) {
    let char_length = 15;
    let frames = vec!["|", "/", "─", "\\"];
    let mut index = 0;

    loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => {}
        };
        print!("\rFetching data {}", frames[index]);
        io::stdout().flush().unwrap();
        index = (index + 1) % frames.len();
        std::thread::sleep(Duration::from_millis(100));
    }
    print!("\r{}{}", " ".repeat(char_length), "\r");
    io::stdout().flush().unwrap();
}

pub fn print_blocks(blocks: &Vec<BlockData>) {
    let blocks_str: Vec<Vec<String>> = blocks
        .iter()
        .map(|b| block::render_block(WIDTH, HEIGHT, b))
        .collect();

    println!("{}", concat_strings(blocks_str[0..4].to_vec(), HFILL));
    println!("{}\n", concat_strings(blocks_str[4..8].to_vec(), HFILL));
}

pub fn print_halving(block: &BlockData) {
    let width = 40;
    let height = 1;

    let seconds_till_halving = (HALVING - block.height % HALVING) as u64 * 10 * 60;
    let progress = (block.height % HALVING) as f64 / HALVING as f64;
    let title = format!(
        "Halving in: {}, {:.0}%",
        utils::format_time(seconds_till_halving),
        progress * 100.0
    );

    let box_halving =
        utils::split_strings_by_newline(&bar::boxed_bar(width, height, progress, title));

    let seconds_till_adjustment =
        (DIFFICULTY_ADJUSTMENT - block.height % DIFFICULTY_ADJUSTMENT) as u64 * 10 * 60;
    let progress = (block.height % DIFFICULTY_ADJUSTMENT) as f64 / DIFFICULTY_ADJUSTMENT as f64;
    let title = format!(
        "Difficulty Adj. in: {}, {:.0}%",
        utils::format_time(seconds_till_adjustment),
        progress * 100.0
    );

    let box_difficulty =
        utils::split_strings_by_newline(&bar::boxed_bar(width, height, progress, title));

    println!(
        "{}",
        concat_strings(vec![box_halving, box_difficulty], HFILL)
    );
}
