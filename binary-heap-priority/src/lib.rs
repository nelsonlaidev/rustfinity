use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn create_max_heap(items: &[i32]) -> BinaryHeap<i32> {
    BinaryHeap::from_iter(items.iter().cloned())
}

pub fn create_min_heap(items: &[i32]) -> BinaryHeap<Reverse<i32>> {
    let mut result = BinaryHeap::new();

    for item in items {
        result.push(Reverse(item.clone()));
    }

    result
}

pub fn pop_max(heap: &mut BinaryHeap<i32>) -> Option<i32> {
    heap.pop()
}

pub fn peek_max(heap: &BinaryHeap<i32>) -> Option<&i32> {
    heap.peek()
}

pub fn top_k_largest(items: &[i32], k: usize) -> Vec<i32> {
    let mut heap = create_max_heap(items);
    let mut result = Vec::new();

    for _ in 0..k {
        match heap.pop() {
            Some(val) => result.push(val),
            None => break,
        }
    }

    result
}

pub fn top_k_smallest(items: &[i32], k: usize) -> Vec<i32> {
    let mut heap = create_min_heap(items);
    let mut result = Vec::new();

    for _ in 0..k {
        match heap.pop() {
            Some(val) => result.push(val.0),
            None => break,
        }
    }

    result
}

pub fn merge_heaps(mut heap1: BinaryHeap<i32>, heap2: BinaryHeap<i32>) -> BinaryHeap<i32> {
    heap1.extend(heap2);
    heap1
}

pub fn heap_sort_descending(items: &[i32]) -> Vec<i32> {
    let mut heap = create_max_heap(items);
    let mut result = Vec::new();

    while let Some(val) = heap.pop() {
        result.push(val);
    }

    result
}

pub fn main() {
    let max_heap = create_max_heap(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Max element: {:?}", max_heap.peek());

    let min_heap = create_min_heap(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Min element: {:?}", min_heap.peek().map(|r| r.0));

    let top3 = top_k_largest(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
    println!("Top 3 largest: {:?}", top3);

    let bottom3 = top_k_smallest(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
    println!("Top 3 smallest: {:?}", bottom3);

    let sorted = heap_sort_descending(&[3, 1, 4, 1, 5]);
    println!("Sorted descending: {:?}", sorted);
}
