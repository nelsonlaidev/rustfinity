pub fn filter_starts_with<'a>(
    input: &'a [String],
    s: &'a str,
) -> impl Iterator<Item = &'a String> + 'a {
    input.iter().filter(move |x| x.starts_with(s))
}

pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered);
}
