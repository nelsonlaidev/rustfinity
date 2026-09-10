// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

// 2. Implement the `Error` trait
impl std::error::Error for ParsePercentageError {}
impl std::fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid input"),
            Self::OutOfRange => write!(f, "Percentage out of range"),
        }
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    let number = input.parse::<u8>();

    match number {
        Err(_) => Err(ParsePercentageError::InvalidInput),
        Ok(val) => {
            if val > 100 {
                Err(ParsePercentageError::OutOfRange)
            } else {
                Ok(val)
            }
        }
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
