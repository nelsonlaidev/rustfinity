// 1. Define the struct
pub struct TextFinder<'a> {
    pub s: &'a str,
}

// 2. Implement the struct and define the methods
impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        TextFinder { s: text }
    }

    pub fn find_first(&self, text: &str) -> Option<&str> {
        self.s.lines().find(|l| l.contains(text))
    }

    pub fn find_many(&self, text: &str) -> Vec<&str> {
        self.s.lines().filter(|l| l.contains(text)).collect()
    }
}

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
