pub fn transform_even_odd(slice: &mut [i32]) {
    slice.iter_mut().for_each(|x| {
        if *x % 2 == 0 {
            *x *= 2;
        } else {
            *x -= 1;
        }
    })
}
