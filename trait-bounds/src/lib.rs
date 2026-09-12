use std::fmt::Display;

pub fn compare_and_display<T>(a: T, b: T) -> T
where
    T: Display + PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

pub fn main() {
    let greater = compare_and_display(10, 20);
    println!("Greater value: {}", greater);

    let greater = compare_and_display("Apple", "Orange");
    println!("Greater value: {}", greater);
}
