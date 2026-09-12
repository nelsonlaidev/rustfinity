pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    Box::new(Animal {
        name: name.to_string(),
        age,
    })
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    (animal.name, animal.age)
}

pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}
