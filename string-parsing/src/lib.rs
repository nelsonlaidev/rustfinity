use std::str::FromStr;

pub fn parse_int(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| "Failed to parse integer".to_string())
}

pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s.trim().to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        "1" => Ok(true),
        "0" => Ok(false),
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err("Invalid input".to_string()),
    }
}

pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let parts = s
        .trim()
        .split_once('=')
        .ok_or_else(|| "Invalid format".to_string())?;

    Ok((parts.0.trim().to_string(), parts.1.trim().to_string()))
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',');

        let colors = parts
            .map(|s| {
                s.trim()
                    .parse::<u8>()
                    .map_err(|_| "Invalid format".to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;

        match colors.as_slice() {
            [r, g, b] => Ok(Color {
                r: *r,
                g: *g,
                b: *b,
            }),
            _ => Err("Invalid format".to_string()),
        }
    }
}

pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    s.split(delimiter)
        .map(|item| {
            item.trim()
                .parse::<T>()
                .map_err(|_| "Invalid input".to_string())
        })
        .collect()
}

pub fn main() {
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);

    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
}
