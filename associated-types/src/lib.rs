use std::{collections::HashMap, hash::Hash};

pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, k: Self::Key, v: Self::Value) -> ();
    fn get(&self, k: &Self::Key) -> Option<&Self::Value>;
}

pub struct InMemoryStore<K, V> {
    pub storage: HashMap<K, V>,
}

impl<K: Eq + Hash, V> KeyValueStore for InMemoryStore<K, V> {
    type Key = K;
    type Value = V;

    fn set(&mut self, k: Self::Key, v: Self::Value) {
        self.storage.insert(k, v);
    }
    fn get(&self, k: &Self::Key) -> Option<&Self::Value> {
        self.storage.get(k)
    }
}

pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));

    store.set("language".to_string(), "Rust".to_string());
    assert_eq!(
        store.get(&"language".to_string()),
        Some(&"Rust".to_string())
    );

    assert_eq!(store.get(&"non_existent".to_string()), None);
}
