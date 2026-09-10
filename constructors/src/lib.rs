pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    // 2. Define the `new` associated function
    pub fn new(title: &str, author: &str, year: i32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}
