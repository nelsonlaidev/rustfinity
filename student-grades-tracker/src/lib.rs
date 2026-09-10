use std::collections::HashMap;

pub struct Student {
    // 1. Define the fields
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    // 2. Define the fields
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        // Implement here
        self.students.insert(
            name.to_string(),
            Student {
                name: name.to_string(),
                grades: Vec::new(),
            },
        );
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // Implement here
        if let Some(student) = self.students.get_mut(name) {
            student.grades.push(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
        if let Some(student) = self.students.get(&name.to_string()) {
            &student.grades
        } else {
            &[]
        }
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
