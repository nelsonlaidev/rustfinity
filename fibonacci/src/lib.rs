pub fn fibonacci(n: u32) -> u32 {
    if (0..=1).contains(&n) {
        return n;
    }

    let mut prev2 = 0;
    let mut prev1 = 1;
    let mut current = 0;

    for _ in 2..=n {
        current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    current
}
