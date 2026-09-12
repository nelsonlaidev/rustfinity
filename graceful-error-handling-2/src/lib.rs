#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

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

pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result);

    let result = parse_percentage("101");
    println!("{:?}", result);

    let result = parse_percentage("abc");
    println!("{:?}", result);
}
