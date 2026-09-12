pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32]) {
    vec.extend_from_slice(elements);
}

pub fn remove_element(vec: &mut Vec<i32>, index: usize) {
    if index < vec.len() {
        vec.remove(index);
    }
}

pub fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()
}

pub fn main() {
    let mut vec = vec![1, 2, 3];
    add_elements(&mut vec, &[4, 5]);
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);

    remove_element(&mut vec, 2);
    assert_eq!(vec, vec![1, 2, 4, 5]);

    assert_eq!(get_element(&vec, 1), Some(2));
    assert_eq!(get_element(&vec, 10), None);
}
