pub fn create_typed_closures() -> (
    impl Fn(f64, f64) -> f64,
    impl FnMut(&mut f64, f64),
    impl FnOnce(String) -> String,
) {
    let calculate_total = |a, b| a + a * b;

    let apply_discount = |a: &mut f64, b: f64| {
        *a -= b;
    };

    let checkout_cart = |s| format!("Checkout complete: {}", s);

    (calculate_total, apply_discount, checkout_cart)
}

pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
