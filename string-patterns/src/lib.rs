pub fn has_prefix(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

pub fn has_suffix(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

pub fn find_first(s: &str, pattern: &str) -> Option<usize> {
    s.find(pattern)
}

pub fn find_last(s: &str, pattern: &str) -> Option<usize> {
    s.rfind(pattern)
}

pub fn count_occurrences(s: &str, pattern: &str) -> usize {
    s.matches(pattern).count()
}

pub fn find_all_indices(s: &str, pattern: &str) -> Vec<usize> {
    s.match_indices(pattern).map(|(i, _)| i).collect()
}

pub fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let start_idx = s.find(start)?;
    let content_start = start_idx + start.len();
    let end_idx = s[content_start..].find(end)? + content_start;

    s.get(content_start..end_idx).map(|sub| sub.to_string())
}

pub fn main() {
    let text = "hello world, hello universe";

    println!("Text: '{}'", text);
    println!();

    println!("has_prefix('hello'): {}", has_prefix(text, "hello"));
    println!("has_suffix('universe'): {}", has_suffix(text, "universe"));
    println!("find_first('hello'): {:?}", find_first(text, "hello"));
    println!("find_last('hello'): {:?}", find_last(text, "hello"));
    println!(
        "count_occurrences('hello'): {}",
        count_occurrences(text, "hello")
    );
    println!(
        "find_all_indices('hello'): {:?}",
        find_all_indices(text, "hello")
    );

    let html = "<title>My Page</title>";
    println!();
    println!("HTML: '{}'", html);
    println!(
        "extract_between('<title>', '</title>'): {:?}",
        extract_between(html, "<title>", "</title>")
    );
}
