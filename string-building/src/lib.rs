use std::fmt::{self, Display, Formatter, Write};

pub fn build_greeting(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

pub fn build_list(items: &[&str]) -> String {
    let mut output = String::new();

    for (index, item) in items.iter().enumerate() {
        write!(output, "{}. {}", index + 1, item).unwrap();

        if index != items.len() - 1 {
            write!(output, "\n").unwrap();
        }
    }

    output
}

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut col_widths = vec![0; headers.len()];
    let mut output = String::new();

    for i in 0..headers.len() {
        if headers[i].len() > col_widths[i] {
            col_widths[i] = headers[i].len()
        }

        for j in 0..rows.len() {
            if rows[j][i].len() > col_widths[i] {
                col_widths[i] = rows[j][i].len();
            }
        }
    }

    for (index, &header) in headers.iter().enumerate() {
        write!(output, "| {:<width$} ", header, width = col_widths[index]).unwrap();
        if index == headers.len() - 1 {
            write!(output, "|\n").unwrap();
        }
    }

    for i in 0..headers.len() {
        write!(output, "|{}", "-".repeat(col_widths[i] + 2)).unwrap();
        if i == headers.len() - 1 {
            write!(output, "|\n").unwrap();
        }
    }

    for row in rows {
        for (index, cell) in row.iter().enumerate() {
            write!(output, "| {:<width$} ", cell, width = col_widths[index]).unwrap();
            if index == row.len() - 1 {
                write!(output, "|\n").unwrap();
            }
        }
    }

    output
}

pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    let mut output = String::new();

    for (index, part) in parts.iter().enumerate() {
        write!(output, "{}", part).unwrap();

        if index != parts.len() - 1 {
            write!(output, "{}", sep).unwrap();
        }
    }

    output
}

pub fn main() {
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));

    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    println!("\n=== Person Display ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    println!("{}", person);

    println!("\n=== build_table ===");
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("{}", build_table(headers, &rows));

    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
}
