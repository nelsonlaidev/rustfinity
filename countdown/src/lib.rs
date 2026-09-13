pub fn countdown(mut n: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while n > 0 {
        result.push(n);

        n -= 1;
    }

    result.push(n);

    result
}
