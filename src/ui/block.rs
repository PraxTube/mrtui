use crate::data::BlockData;
use crate::utils;

const BLOCK_FILL_COLOR_INDEX: u8 = 69;

fn write_row(message: &mut Vec<Vec<char>>, row: i32, content: &str) {
    let width = message[0].len();
    let content = if width - 2 < content.chars().count() {
        &content[0..width - 1]
    } else {
        content
    };

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

pub fn render(width: usize, height: usize, block: &BlockData) -> Vec<String> {
    if width < 2 || height < 2 {
        panic!("Width and height must be at least 2.");
    }
    let title = utils::format_number_kilo(block.height, "_").into_bytes();
    let title = if width < title.len() + 2 {
        title[0..width - 1].to_vec()
    } else {
        title
    };

    let title_start_index = width / 2 - utils::div_up(title.len(), 2);
    let mut result: Vec<String> = Vec::new();
    // TODO: Fix this size, which is the reason
    // we get bad positioning (should be smaller then width and height)
    let mut message = vec![vec![' '; width]; height];
    let block_fill = block.weight as f32 / 4_000_000 as f32;

    write_row(
        &mut message,
        1,
        &format!("\u{2248} {:.0} sat/vB", block.extras.median_fee),
    );
    write_row(
        &mut message,
        2,
        &format!(
            "{:.0} - {:.0} sat/vB",
            block.extras.fee_range[0],
            block.extras.fee_range[block.extras.fee_range.len() - 1]
        ),
    );
    write_row(
        &mut message,
        4,
        &format!("{}B", utils::format_number_unit(block.size)),
    );
    write_row(&mut message, 5, &format!("{} txs", block.tx_count));
    write_row(
        &mut message,
        -2,
        &utils::minute_difference(block.timestamp.into()),
    );

    // Top border
    result.push(String::new());
    result[0].push('╭');
    for i in 1..width - 1 {
        if i >= title_start_index && i < title_start_index + title.len() {
            result[0].push(title[i - title_start_index] as char);
        } else {
            result[0].push('─');
        }
    }
    result[0].push('╮');

    // Middle rows
    for i in 1..height - 1 {
        result.push(String::new());
        result[i].push('│');

        for j in 1..width - 1 {
            let percentage = 1.0 - i as f32 / height as f32;
            if block_fill > percentage {
                let content = format!(
                    "\x1b[48;5;{}m{}\x1b[49m",
                    BLOCK_FILL_COLOR_INDEX, message[i][j]
                );
                result[i] += &content;
            } else {
                result[i].push(message[i][j]);
            }
        }
        result[i].push('│');
    }

    // Bottom border
    result.push(String::new());
    result[height - 1].push('╰');
    for _ in 1..width - 1 {
        result[height - 1].push('─');
    }
    result[height - 1].push('╯');

    result
}
