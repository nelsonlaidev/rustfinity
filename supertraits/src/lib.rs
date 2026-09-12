pub trait Person {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

pub struct Undergraduate {
    pub id: u32,
    pub name: String,
    pub field_of_study: String,
}

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
