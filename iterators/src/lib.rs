pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    iter.filter(|x| x % 2 != 0).collect()
}

pub fn uppercase_strings<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<String> {
    iter.map(|x| x.to_uppercase()).collect()
}

pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let odd_numbers = filter_even_numbers(numbers.into_iter());
    println!("{:?}", odd_numbers);

    let words = vec!["hello", "world"];
    let uppercase_words = uppercase_strings(words.into_iter());
    println!("{:?}", uppercase_words);
}
