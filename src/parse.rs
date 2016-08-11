

pub fn parse_line<S: Into<String>>(line: S) -> Vec<String> {
    let quotes = ['\'', '\"'];

    let mut parsed: Vec<String> = Vec::new();

    for c in line.into().trim().chars() {
        let mut word_accumulator = String::new();
        word_accumulator.push(c);
        if c == ' ' {
            parsed.push(word_accumulator);
        }
    }
    return parsed;
}
