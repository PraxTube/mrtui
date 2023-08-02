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
