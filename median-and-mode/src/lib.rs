use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();

    let len = numbers.len();
    let mid = len / 2;

    if len % 2 == 0 {
        (numbers[mid - 1] as f32 + numbers[mid] as f32) / 2.0
    } else {
        numbers[mid] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for number in numbers {
        *map.entry(*number).or_insert(0) += 1;
    }

    let max_val = map.values().copied().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, val)| val == max_val)
        .map(|(num, _)| num)
        .collect()
}
