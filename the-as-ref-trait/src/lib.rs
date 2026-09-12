pub fn print_message<S: AsRef<str>>(s: S) -> () {
    let s = s.as_ref();

    println!("{}", s);
}

pub fn main() {
    print_message("Hello, world!");

    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
