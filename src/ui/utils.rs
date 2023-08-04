pub fn concat_strings(strings: Vec<Vec<String>>, hfill: &str) -> String {
    if strings.len() == 0 {
        return String::new();
    }

    let mut result: Vec<String> = vec![String::new(); strings[0].len()];

    for i in 0..strings[0].len() {
        let mut row = String::new();
        row.push_str(&strings[0][i]);
        for k in 1..strings.len() {
            row.push_str(hfill);
            row.push_str(&strings[k][i]);
        }
        result[i].push_str(&row);
    }

    result.join("\n")
}

pub fn stylize_string(input_str: &str) -> String {
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
