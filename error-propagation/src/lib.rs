use std::io::BufRead;
use std::{fs::File, io};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let mut sum = 0;

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let number = line
            .parse::<i32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;

        sum += number;
    }

    Ok(sum)
}

pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
