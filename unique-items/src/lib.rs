use std::collections::HashSet;

// 1. Finish the function
pub fn unique_items<I, T>(items: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut set = HashSet::new();

    for item in items {
        let s = item.as_ref().trim();
        if !s.is_empty() {
            set.insert(s.to_string());
        }
    }

    let mut result: Vec<String> = set.into_iter().collect();

    result.sort();
    result
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
