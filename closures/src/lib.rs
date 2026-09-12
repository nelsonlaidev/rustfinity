pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        a + b
    };

    let subtract_closure = |a, b| a - b;

    let multiply_closure = |a, b| a * b;

    (add_closure, subtract_closure, multiply_closure)
}

pub fn main() {
    let (add, subtract, multiply) = create_closures();

    assert_eq!(add(3, 4), 7);
    assert_eq!(subtract(10, 4), 6);
    assert_eq!(multiply(3, 5), 15);
}
