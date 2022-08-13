pub fn word_wrap(text: &str, width: i32) -> String {
    let mut out = String::new();
    let mut line = String::new();
    let mut words = text.split_whitespace();
    while let Some(word) = words.next() {
        if line.len() + word.len() > width as usize {
            out.push_str(&line);
            out.push_str("\n");
            line = String::new();
        }
        line.push_str(word);
        line.push(' ');
    }
    out.push_str(&line);
    out
}