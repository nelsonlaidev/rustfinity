pub fn mutating_variables() -> String {
    let mut text = String::from("hello");
    mutates_value(&mut text);
    text
}

pub fn mutates_value(value: &mut String) {
    *value = String::from("bye")
}
