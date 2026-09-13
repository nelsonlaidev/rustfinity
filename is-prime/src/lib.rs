pub fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    if n < 2 {
        return false;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i <= n / i {
        if n % i == 0 {
            return false;
        };

        i += 2;
    }

    true
}
