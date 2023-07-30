use std::time::{SystemTime, UNIX_EPOCH};

pub fn div_up(number: usize, divider: usize) -> usize {
    let result = number / divider;

    if number % divider == 0 {
        return result;
    } else {
        return result + 1;
    }
}

pub fn format_number_bytes(number: u32) -> String {
    let kilo = 1_000;
    let meg = 1_000_000;
    let gig = 1_000_000_000;

    if number > gig {
        return format!("{:.2}GB", number as f32 / gig as f32);
    } else if number > meg {
        return format!("{:.2}MB", number as f32 / meg as f32);
    } else if number > kilo {
        return format!("{:.2}kB", number as f32 / kilo as f32);
    }
    return format!("{}B", number as f32);
}

pub fn minute_difference(block_timestamp: u64) -> String {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_secs();
    let sec_difference = current_timestamp - block_timestamp;
    let minutes = sec_difference / 60;
    let days = minutes / 1440;

    if days >= 1 {
        return format!("{} days ago", days);
    }

    format!("{} minutes ago", minutes)
}
