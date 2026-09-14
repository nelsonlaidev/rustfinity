pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

pub fn byte_count(s: &str) -> usize {
    s.bytes().count()
}

pub fn safe_substring(s: &str, start: usize, end: usize) -> Option<String> {
    if start > end {
        return None;
    }

    let total_chars = s.chars().count();

    if start > total_chars || end > total_chars {
        return None;
    }

    Some(s.chars().skip(start).take(end - start).collect::<String>())
}

pub fn char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

pub fn is_single_char(s: &str) -> bool {
    s.chars().count() == 1
}

pub fn main() {
    let examples = ["Hello", "Привет", "你好", "🎉"];

    for text in &examples {
        println!(
            "\"{}\" - chars: {}, bytes: {}",
            text,
            char_count(text),
            byte_count(text)
        );
    }

    let text = "Hello, 世界!";
    println!("\nSubstring [0..5]: {:?}", safe_substring(text, 0, 5));
    println!("Char at index 7: {:?}", char_at(text, 7));
    println!("Is '好' single char? {}", is_single_char("好"));
}
