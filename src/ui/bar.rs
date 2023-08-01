use crate::utils;

const FILLED_CHAR: char = '█';
const EMPTY_CHAR: char = ' ';

fn bare_bar(progress: f64, width: usize) -> String {
    let filled_width = (progress * width as f64) as usize;
    let remaining_width = width - filled_width;

    let mut bar = String::new();

    for _ in 0..filled_width {
        bar.push(FILLED_CHAR);
    }
    for _ in 0..remaining_width {
        bar.push(EMPTY_CHAR);
    }
    bar
}

fn render_box(width: usize, height: usize, title: String, message: Vec<Vec<char>>) -> String {
    if width < 2 || height < 2 {
        panic!("Width and height must be at least 2.");
    }
    let title = if width < title.len() + 2 {
        title.into_bytes()[0..width - 1].to_vec()
    } else {
        title.into_bytes()
    };

    let mut result = String::new();
    let title_start_index = width / 2 - utils::div_up(title.len(), 2);

    // Top border
    result.push('╭');
    for i in 1..width - 1 {
        if i >= title_start_index && i < title_start_index + title.len() {
            result.push(title[i - title_start_index] as char);
        } else {
            result.push('─');
        }
    }
    result.push('╮');
    result.push('\n');

    // Middle rows
    for i in 0..height - 2 {
        result.push('│');

        for j in 0..width - 2 {
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

pub fn boxed_bar(width: usize, height: usize, progress: f64, title: String) -> String {
    let height = height + 2;
    let width = width + 2;
    let progress_bar = bare_bar(progress, width - 2);

    let mut message: Vec<Vec<char>> = vec![];
    for i in 0..height - 2 {
        message.push(vec![]);
        for c in progress_bar.chars() {
            message[i].push(c);
        }
    }

    let progress_box = render_box(width, height, title, message);
    progress_box
}
