pub fn chars_to_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub fn words_to_vec(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

pub fn lines_to_vec(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}

pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .map(String::from)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut chars = w.chars();

            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn main() {
    let text = "hello world";
    println!("Original: '{}'", text);
    println!("chars_to_vec: {:?}", chars_to_vec(text));
    println!("words_to_vec: {:?}", words_to_vec(text));
    println!("count_words: {}", count_words(text));
    println!("reverse_words: '{}'", reverse_words(text));
    println!("capitalize_words: '{}'", capitalize_words(text));

    let multiline = "line one\nline two\nline three";
    println!("\nMultiline text:");
    println!("{}", multiline);
    println!("lines_to_vec: {:?}", lines_to_vec(multiline));
}
