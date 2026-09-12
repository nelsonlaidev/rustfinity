pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for i in indices {
        let x = slice.get_mut(*i);

        match x {
            Some(val) => *val = value,
            None => continue,
        }
    }
}
