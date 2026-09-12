pub fn check_number_sign(number: i32) -> String {

    if number > 0 {
        String::from("positive")
    }
    else if number < 0 {
        String::from("negative")
    }
    else {
        String::from("zero")
    }
}
