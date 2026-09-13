pub fn clean_string(s: &str) -> String {
    s.trim().to_lowercase()
}

pub fn contains_word(text: &str, word: &str) -> bool {
    text.to_lowercase().contains(&word.to_lowercase())
}

pub fn replace_word(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    let parts = s.split(delimiter).collect::<Vec<&str>>();

    parts.iter().map(|x| x.trim().to_string()).collect()
}

pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn main() {
    let messy = "  Hello, World!  ";
    println!("Original: '{}'", messy);
    println!("Cleaned: '{}'", clean_string(messy));

    let text = "Rust is a systems programming language";
    println!("\nText: '{}'", text);
    println!("Contains 'SYSTEMS': {}", contains_word(text, "SYSTEMS"));
    println!("Contains 'Java': {}", contains_word(text, "Java"));

    let original = "hello world world";
    println!("\nOriginal: '{}'", original);
    println!(
        "Replace 'world' with 'Rust': '{}'",
        replace_word(original, "world", "Rust")
    );

    let csv = "  apple ,  banana  , cherry ";
    println!("\nCSV: '{}'", csv);
    println!("Split and trim: {:?}", split_and_trim(csv, ','));

    let spaced = "  too   many    spaces   here  ";
    println!("\nSpaced: '{}'", spaced);
    println!("Normalized: '{}'", normalize_whitespace(spaced));
}
