// 1. Finish the trait definition
pub trait Person {
    fn name(&self) -> String;
}

// 2. Finish the trait definition
pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

// 3. Finish the struct definition
pub struct Undergraduate {
    // Define fields for id, name, and field_of_study here...
    pub id: u32,
    pub name: String,
    pub field_of_study: String,
}

// 4. Implement the necessary traits for the Undergraduate struct
impl Person for Undergraduate {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for Undergraduate {
    fn id(&self) -> u32 {
        self.id
    }

    fn field_of_study(&self) -> String {
        self.field_of_study.clone()
    }
}

// Example usage
pub fn main() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.name(), "John Doe");
    assert_eq!(student.id(), 101);
    assert_eq!(student.field_of_study(), "Computer Science");
}
