pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let celsius = to_celsius(value, from_unit)?;

    match to_unit {
        "K" => Ok(celsius + 273.15),
        "F" => Ok(celsius * (9.0 / 5.0) + 32.0),
        "C" => Ok(celsius),
        _ => Err(String::from("Invalid unit")),
    }
}

fn to_celsius(value: f64, from_unit: &str) -> Result<f64, String> {
    match from_unit {
        "F" => Ok((value - 32.0) * (5.0 / 9.0)),
        "K" => Ok(value - 273.15),
        "C" => Ok(value),
        _ => Err(String::from("Invalid unit")),
    }
}
