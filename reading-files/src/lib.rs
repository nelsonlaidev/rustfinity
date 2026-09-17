use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub fn read_entire_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn count_lines(path: &str) -> Result<usize, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}

pub fn count_words(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;

    Ok(contents.split_whitespace().count())
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().take(n).collect()
}

pub fn main() {
    let sample_content = "Hello World\nThis is a test\nRust is great\n";
    let path = "sample.txt";

    if fs::write(path, sample_content).is_ok() {
        println!("Sample file created.");

        if let Ok(contents) = read_entire_file(path) {
            println!("File contents:\n{}", contents);
        }

        if let Ok(count) = count_lines(path) {
            println!("Line count: {}", count);
        }

        if let Ok(count) = count_words(path) {
            println!("Word count: {}", count);
        }

        if let Ok(lines) = read_lines(path) {
            println!("Lines: {:?}", lines);
        }

        if let Ok(lines) = first_n_lines(path, 2) {
            println!("First 2 lines: {:?}", lines);
        }

        let _ = fs::remove_file(path);
    }
}
