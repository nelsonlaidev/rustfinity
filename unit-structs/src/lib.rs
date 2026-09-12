pub struct Logger;

impl Logger {
    pub fn log_message(message: &str) -> () {
        println!("[LOG]: {message}.");
    }
}

pub fn main() {
    Logger::log_message("Hello, World!");
}
