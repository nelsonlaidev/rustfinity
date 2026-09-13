pub fn fizz_buzz(num: u32) -> String {
    match num {
        x if x % 3 == 0 && x % 5 == 0 => String::from("FizzBuzz"),
        x if x % 3 == 0 => String::from("Fizz"),
        x if x % 5 == 0 => String::from("Buzz"),
        _ => num.to_string(),
    }
}
