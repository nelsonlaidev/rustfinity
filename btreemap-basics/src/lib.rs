use std::collections::BTreeMap;

pub fn create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32> {
    pairs.iter().cloned().collect()
}

pub fn get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()
}

pub fn get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String> {
    map.keys().cloned().collect()
}

pub fn get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32> {
    map.values().copied().collect()
}

pub fn get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)> {
    map.range(start.to_string()..end.to_string())
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

pub fn get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    map.first_key_value().map(|(k, v)| (k.clone(), *v))
}

pub fn get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    map.last_key_value().map(|(k, v)| (k.clone(), *v))
}

pub fn main() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];

    let map = create_sorted_map(&pairs);
    println!("Map: {:?}", map);

    println!("Value for 'apple': {:?}", get_value(&map, "apple"));
    println!("Keys in order: {:?}", get_keys_in_order(&map));
    println!("Values in key order: {:?}", get_values_in_key_order(&map));

    let range = get_range(&map, "apple", "cherry");
    println!("Range [apple, cherry): {:?}", range);

    println!("First entry: {:?}", get_first(&map));
    println!("Last entry: {:?}", get_last(&map));
}
