use std::mem;

pub fn find_first_palindrome(mut start: i32, mut end: i32) -> Option<i32> {
    if start > end {
        mem::swap(&mut start, &mut end);
    }

    for n in start..=end {
        let string = n.to_string();

        if string.chars().rev().collect::<String>() == string {
            return Some(n);
        }
    }

    None
}
