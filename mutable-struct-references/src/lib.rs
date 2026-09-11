// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    pub s: &'a mut String,
}

// 2. Implement the methods for the struct
impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        MutableTextFinder { s: text }
    }

    pub fn find_first(&self, text: &str) -> Option<&str> {
        self.s.lines().find(|l| l.contains(text))
    }

    pub fn replace_lines(&mut self, text: &str, replacement: &str) -> () {
        let modified = self
            .s
            .lines()
            .map(|x| if x.contains(text) { replacement } else { x })
            .collect::<Vec<_>>()
            .join("\n");

        *self.s = modified;
    }

    pub fn get_text(&self) -> &str {
        self.s
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
