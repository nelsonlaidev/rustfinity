use std::collections::HashMap;

pub fn count_words(text: &str) -> HashMap<String, u32> {
    let words = text
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>();
    let mut word_counts = HashMap::new();

    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    word_counts
}

pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut word_groups: HashMap<usize, Vec<String>> = HashMap::new();

    for word in words {
        word_groups
            .entry(word.len())
            .or_default()
            .push(word.to_string());
    }

    word_groups
}

pub fn get_or_compute<F>(cache: &mut HashMap<String, i32>, key: &str, compute: F) -> i32
where
    F: FnOnce() -> i32,
{
    *cache.entry(key.to_string()).or_insert_with(compute)
}

pub fn increment_or_init(map: &mut HashMap<String, i32>, key: &str, init: i32) {
    map.entry(key.to_string())
        .and_modify(|v| *v += 1)
        .or_insert(init);
}

pub fn merge_maps(
    mut map1: HashMap<String, i32>,
    map2: HashMap<String, i32>,
) -> HashMap<String, i32> {
    map2.iter().for_each(|(k, v)| {
        *map1.entry(k.clone()).or_insert(0) += *v;
    });

    map1
}

pub fn first_occurrence(items: &[&str]) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    for (index, &item) in items.iter().enumerate() {
        result.entry(item.to_string()).or_insert(index);
    }

    result
}

pub fn update_or_default(map: &mut HashMap<String, Vec<i32>>, key: &str, value: i32) {
    map.entry(key.to_string()).or_default().push(value);
}

pub fn main() {
    let counts = count_words("the quick brown fox jumps over the lazy dog");
    println!("Word counts: {:?}", counts);

    let grouped = group_by_length(&["hi", "hello", "hey", "world"]);
    println!("Grouped by length: {:?}", grouped);

    let mut cache: HashMap<String, i32> = HashMap::new();
    let value = get_or_compute(&mut cache, "answer", || {
        println!("Computing expensive value...");
        42
    });
    println!("Cached value: {}", value);

    let mut scores: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut scores, "alice", 100);
    increment_or_init(&mut scores, "alice", 100);
    println!("Alice's score: {}", scores["alice"]);

    let map1: HashMap<String, i32> = [("a".to_string(), 1), ("b".to_string(), 2)].into();
    let map2: HashMap<String, i32> = [("b".to_string(), 3), ("c".to_string(), 4)].into();
    let merged = merge_maps(map1, map2);
    println!("Merged: {:?}", merged);

    let firsts = first_occurrence(&["a", "b", "a", "c", "b"]);
    println!("First occurrences: {:?}", firsts);

    let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut groups, "evens", 2);
    update_or_default(&mut groups, "evens", 4);
    println!("Groups: {:?}", groups);
}
