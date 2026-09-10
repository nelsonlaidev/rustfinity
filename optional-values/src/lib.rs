pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here...
    let filtered: Vec<&i32> = numbers.iter().filter(|&x| *x % 2 == 0).collect();

    match filtered.get(0) {
        Some(val) => Some(**val),
        None => None,
    }
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
}
