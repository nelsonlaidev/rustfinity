pub fn to_owned_string(s: &str) -> String {
    s.to_string()
}

pub fn count_chars(s: &str) -> usize {
    s.chars().count()
}

pub fn count_bytes(s: &str) -> usize {
    s.bytes().count()
}

pub fn is_ascii_only(s: &str) -> bool {
    s.is_ascii()
}

pub fn first_char(s: &str) -> Option<char> {
    s.chars().nth(0)
}

pub fn main() {
    let greeting = "Hello, world!";

    println!("Original: {}", greeting);
    println!("As owned String: {}", to_owned_string(greeting));
    println!("Character count: {}", count_chars(greeting));
    println!("Byte count: {}", count_bytes(greeting));
    println!("Is ASCII only: {}", is_ascii_only(greeting));
    println!("First character: {:?}", first_char(greeting));
}
