use crate::data::BlockData;
use crate::utils;

const BLOCK_FILL_COLOR_INDEX: u8 = 69;

fn write_row(message: &mut Vec<Vec<char>>, row: i32, content: &str) {
    let width = message[0].len();
    if width - 2 < content.chars().count() {
        panic!("Width is too small.");
    }

    let row: usize = match row < 0 {
        true => message.len() - row.wrapping_abs() as usize,
        false => row.wrapping_abs() as usize,
    };

    let mut index = utils::div_up(width, 2) - utils::div_up(content.chars().count(), 2);

    for char in content.chars() {
        message[row][index] = char;
        index += 1;
    }
}

pub fn render(width: usize, height: usize, block: &BlockData) -> String {
    if width < 2 || height < 2 {
        panic!("Width and height must be at least 2.");
    }
    let bheight = block.height.to_string().into_bytes();
    if width < bheight.len() + 2 {
        panic!("Width must be bigger then the number of digits in block height");
    }

    let title_start_index = width / 2 - utils::div_up(bheight.len(), 2);
    let mut result = String::new();
    // TODO: Fix this size, which is the reason
    // we get bad positioning (should be smaller then width and height)
    let mut message = vec![vec![' '; width]; height];
    let block_fill = block.weight as f32 / 4_000_000 as f32;

    write_row(
        &mut message,
        3,
        &format!("{}B", utils::format_number_unit(block.size)),
    );
    write_row(&mut message, 4, &format!("{} txs", block.tx_count));
    write_row(
        &mut message,
        5,
        &format!("{}WU", utils::format_number_unit(block.weight)),
    );
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
