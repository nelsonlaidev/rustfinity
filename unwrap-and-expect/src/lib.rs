use std::{env, fs};

pub fn read_file_to_string(path: &str) -> String {
    let contents = fs::read_to_string(path).expect(&format!("Failed to read file: {path}"));

    contents
}

pub fn get_env_variable(key: &str) -> String {
    env::var(key).unwrap()
}

pub fn main() {
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    std::env::set_var("EXAMPLE_KEY", "example_value");
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}
