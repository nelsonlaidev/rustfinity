pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    let num1 = numbers.get(index1)?;
    let num2 = numbers.get(index2)?;

    Some(num1 * num2)
}

pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result = find_and_multiply(numbers.clone(), 1, 3);
    println!("{:?}", result);

    let result = find_and_multiply(numbers.clone(), 1, 10);
    println!("{:?}", result);
}
