// 1. Complete the definitions of the structs Person, Point, and Rectangle.
// have a look at the function below `debug_example` to get the fields of the structs.
//
// 2. Make sure you implement the Debug trait for each struct. using the derive attribute.
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// Example function
pub fn debug_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let point = Point { x: 5.0, y: -3.2 };
    println!("{:?}", point);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rectangle);
}
