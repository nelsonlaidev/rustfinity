use std::collections::HashSet;

pub fn unique_elements(items: &[i32]) -> HashSet<i32> {
    HashSet::from_iter(items.iter().copied())
}

pub fn count_unique(items: &[i32]) -> usize {
    unique_elements(items).iter().count()
}

pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).copied().collect()
}

pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.union(set2).copied().collect()
}

pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.difference(set2).copied().collect()
}

pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.symmetric_difference(set2).copied().collect()
}

pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) -> bool {
    potential_subset.is_subset(potential_superset)
}

pub fn main() {
    let items = vec![1, 2, 3, 2, 1, 4, 3];
    let unique = unique_elements(&items);
    println!("Unique elements: {:?}", unique);

    let count = count_unique(&[1, 2, 2, 3, 3, 3]);
    println!("Count of unique elements: {}", count);

    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    println!("Intersection: {:?}", find_common(&set1, &set2));
    println!("Union: {:?}", find_all(&set1, &set2));
    println!(
        "Difference (set1 - set2): {:?}",
        find_difference(&set1, &set2)
    );
    println!(
        "Symmetric difference: {:?}",
        find_symmetric_difference(&set1, &set2)
    );

    let small: HashSet<i32> = [2, 3].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    println!(
        "Is {:?} subset of {:?}? {}",
        small,
        large,
        is_subset(&small, &large)
    );
}
