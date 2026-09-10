pub fn data_types() -> (u8, f64, bool, char) {
    // 1. Define variable of type `u8` and value `42`
    let n1: u8 = 42;
    // 2. Define variable of type `f64` and value `3.14`
    let n2: f64 = 3.14;
    // 3. Define variable of type `bool` and value `false`
    let n3: bool = false;
    // 4. Define variable of type `char` and value `a`
    let n4: char = 'a';
    // 5. Return a tuple with the variables in the order they were defined
    (n1, n2, n3, n4)
}
