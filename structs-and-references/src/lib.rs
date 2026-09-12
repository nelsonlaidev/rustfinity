pub struct TextFinder<'a> {
    pub s: &'a str,
}

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

pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first);

    let matches = finder.find_many("Rust");
    println!("{:?}", matches);
}
