use std::time::{SystemTime, UNIX_EPOCH};

pub fn div_up(number: usize, divider: usize) -> usize {
    let result = number / divider;

    if number % divider == 0 {
        return result;
    } else {
        return result + 1;
    }
}

pub fn format_number_unit(number: u32) -> String {
    let kilo = 1_000;
    let meg = 1_000_000;
    let gig = 1_000_000_000;

    if number > gig {
        return format!("{:.2} G", number as f32 / gig as f32);
    } else if number > meg {
        return format!("{:.2} M", number as f32 / meg as f32);
    } else if number > kilo {
        return format!("{:.2} k", number as f32 / kilo as f32);
    }
    return format!("{}", number as f32);
}

pub fn format_time(seconds: u64) -> String {
    let year_div = 365 * 24 * 60 * 60;
    let month_div = 30 * 24 * 60 * 60;
    let day_div = 24 * 60 * 60;
    let hour_div = 60 * 60;
    let minute_div = 60;

    if seconds > year_div {
        return format!("{} years", seconds / year_div);
    } else if seconds > month_div {
        return format!("{} months", seconds / month_div);
    } else if seconds > day_div {
        return format!("{} days", seconds / day_div);
    } else if seconds > hour_div {
        return format!("{} hours", seconds / hour_div);
    } else if seconds > minute_div {
        return format!("{} minutes", seconds / minute_div);
    } else {
        format!("{} seconds", seconds)
    }
}

pub fn minute_difference(block_timestamp: u64) -> String {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_secs();
    let sec_difference = current_timestamp - block_timestamp;
    let time = format_time(sec_difference);

    format!("{} ago", time)
}
