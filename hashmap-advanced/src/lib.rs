use std::collections::HashMap;

pub fn create_with_capacity(capacity: usize) -> HashMap<String, i32> {
    HashMap::with_capacity(capacity)
}

pub fn reserve_additional(map: &mut HashMap<String, i32>, additional: usize) {
    map.reserve(additional);
}

pub fn shrink_map(map: &mut HashMap<String, i32>) {
    map.shrink_to_fit();
}

pub fn bulk_insert(items: &[(&str, i32)]) -> HashMap<String, i32> {
    let mut map = create_with_capacity(items.len());

    for (k, v) in items {
        map.insert(k.to_string(), *v);
    }

    map
}

pub fn get_capacity_stats(map: &HashMap<String, i32>) -> (usize, usize) {
    (map.len(), map.capacity())
}

pub fn clear_and_shrink(map: &mut HashMap<String, i32>) {
    map.clear();
    map.shrink_to_fit();
}

pub fn group_by_key<F>(items: &[(String, i32)], key_fn: F) -> HashMap<String, Vec<i32>>
where
    F: Fn(&str) -> String,
{
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    for (k, v) in items {
        map.entry(key_fn(k)).or_default().push(*v);
    }

    map
}

pub fn merge_with_capacity(maps: Vec<HashMap<String, i32>>) -> HashMap<String, i32> {
    let total_len = maps.iter().map(|h| h.len()).sum();

    let mut merged = HashMap::with_capacity(total_len);

    for map in maps {
        for (k, v) in map {
            *merged.entry(k).or_default() += v;
        }
    }

    merged
}

pub fn main() {
    let map = create_with_capacity(100);
    println!("Created map with capacity: {}", map.capacity());

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    reserve_additional(&mut map, 100);
    println!("After reserve: capacity = {}", map.capacity());

    let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
    map.insert("a".to_string(), 1);
    println!("Before shrink: capacity = {}", map.capacity());
    shrink_map(&mut map);
    println!("After shrink: capacity = {}", map.capacity());

    let items = [("a", 1), ("b", 2), ("c", 3)];
    let map = bulk_insert(&items);
    println!("Bulk inserted {} items", map.len());

    let map = create_with_capacity(50);
    let (len, cap) = get_capacity_stats(&map);
    println!("Stats: len = {}, capacity = {}", len, cap);

    let mut map = bulk_insert(&[("a", 1), ("b", 2), ("c", 3)]);
    println!("Before clear: len = {}", map.len());
    clear_and_shrink(&mut map);
    println!("After clear: len = {}", map.len());

    let items = vec![
        ("apple".to_string(), 1),
        ("apricot".to_string(), 2),
        ("banana".to_string(), 3),
    ];
    let grouped = group_by_key(&items, |s| s.chars().next().unwrap().to_string());
    println!("Grouped: {:?}", grouped);

    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = [("a".to_string(), 2), ("b".to_string(), 3)].into();
    let merged = merge_with_capacity(vec![map1, map2]);
    println!("Merged: {:?}", merged);
}
