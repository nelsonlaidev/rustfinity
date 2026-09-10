pub struct Person {
    // Define fields here
    // Read the description
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    person.age >= 18
} // Finish the function
