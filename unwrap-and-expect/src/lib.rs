use std::{env, fs};

pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    let contents = fs::read_to_string(path).expect(&format!("Failed to read file: {path}"));

    contents
}

pub fn get_env_variable(key: &str) -> String {
    // 2. Implement the function
    env::var(key).unwrap()
}

/// Example usage
pub fn main() {
    // Example 1: Using read_file_to_string
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    // Example 2: Using get_env_variable
    std::env::set_var("EXAMPLE_KEY", "example_value");
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    // Must panic
    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}
