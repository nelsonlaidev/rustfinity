use std::collections::HashMap;

pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    map.insert(key, value);
}

pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    map.get(&key).cloned()
}

pub fn main() {
    let mut store = HashMap::new();

    insert_or_update(&mut store, "name".to_string(), "Alice".to_string());

    insert_or_update(&mut store, "name".to_string(), "Bob".to_string());

    let value = get_value(&store, "name".to_string());
    assert_eq!(value, Some("Bob".to_string()));

    let missing = get_value(&store, "age".to_string());
    assert_eq!(missing, None);
}
