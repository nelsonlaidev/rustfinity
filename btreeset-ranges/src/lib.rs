use std::collections::BTreeSet;
use std::ops::Bound;

pub fn create_number_set(numbers: &[i32]) -> BTreeSet<i32> {
    BTreeSet::from_iter(numbers.iter().copied())
}

pub fn get_range(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    set.range(start..end).copied().collect()
}

pub fn get_range_inclusive(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    set.range(start..=end).copied().collect()
}

pub fn get_elements_before(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    set.range(..threshold).copied().collect()
}

pub fn get_elements_from(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    set.range(threshold..).copied().collect()
}

pub fn count_in_range(set: &BTreeSet<i32>, start: i32, end: i32) -> usize {
    set.range(start..=end).count()
}

pub fn find_closest_less_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    set.range(..value).next_back().copied()
}

pub fn find_closest_greater_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    set.range((Bound::Excluded(value), Bound::Unbounded))
        .next()
        .copied()
}

pub fn main() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11, 13, 15]);
    println!("Set: {:?}", set);

    let range = get_range(&set, 5, 12);
    println!("Range [5, 12): {:?}", range);

    let range_inclusive = get_range_inclusive(&set, 5, 11);
    println!("Range [5, 11]: {:?}", range_inclusive);

    let before = get_elements_before(&set, 7);
    println!("Elements < 7: {:?}", before);

    let from = get_elements_from(&set, 9);
    println!("Elements >= 9: {:?}", from);

    let count = count_in_range(&set, 3, 11);
    println!("Count in [3, 11]: {}", count);

    let closest_less = find_closest_less_than(&set, 10);
    println!("Closest < 10: {:?}", closest_less);

    let closest_greater = find_closest_greater_than(&set, 10);
    println!("Closest > 10: {:?}", closest_greater);
}
