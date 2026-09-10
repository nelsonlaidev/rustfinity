pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    let number = input.parse::<u8>();

    match number {
        Err(_) => Err(String::from("Invalid input")),
        Ok(val) => {
            if val > 100 {
                Err(String::from("Percentage out of range"))
            } else {
                Ok(val)
            }
        }
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
