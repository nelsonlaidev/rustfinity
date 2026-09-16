use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

pub fn vec_to_hashset<T: Eq + Hash>(vec: Vec<T>) -> HashSet<T> {
    HashSet::from_iter(vec)
}

pub fn vec_to_btreeset<T: Ord>(vec: Vec<T>) -> BTreeSet<T> {
    BTreeSet::from_iter(vec)
}

pub fn hashset_to_sorted_vec<T: Ord>(set: HashSet<T>) -> Vec<T> {
    let mut result = Vec::from_iter(set);

    result.sort();
    result
}

pub fn pairs_to_hashmap<K: Eq + Hash, V>(pairs: Vec<(K, V)>) -> HashMap<K, V> {
    HashMap::from_iter(pairs)
}

pub fn pairs_to_btreemap<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    BTreeMap::from_iter(pairs)
}

pub fn hashmap_to_pairs<K, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    Vec::from_iter(map)
}

pub fn merge_vecs<T>(vecs: Vec<Vec<T>>) -> Vec<T> {
    Vec::from_iter(vecs.into_iter().flatten())
}

pub fn chain_and_collect<T>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    first.into_iter().chain(second).collect()
}

pub fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 3];
    let unique = vec_to_hashset(numbers);
    println!("Unique numbers: {:?}", unique);

    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted_unique = vec_to_btreeset(numbers);
    println!("Sorted unique: {:?}", sorted_unique);

    let set: HashSet<i32> = [5, 2, 8, 1].into_iter().collect();
    let sorted = hashset_to_sorted_vec(set);
    println!("Sorted vec: {:?}", sorted);

    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map = pairs_to_hashmap(pairs);
    println!("HashMap: {:?}", map);

    let pairs = vec![("c", 3), ("a", 1), ("b", 2)];
    let map = pairs_to_btreemap(pairs);
    println!("BTreeMap (sorted): {:?}", map);

    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    let pairs = hashmap_to_pairs(map);
    println!("Pairs: {:?}", pairs);

    let vecs = vec![vec![1, 2], vec![3, 4], vec![5]];
    let merged = merge_vecs(vecs);
    println!("Merged: {:?}", merged);

    let chained = chain_and_collect(vec![1, 2], vec![3, 4]);
    println!("Chained: {:?}", chained);
}
