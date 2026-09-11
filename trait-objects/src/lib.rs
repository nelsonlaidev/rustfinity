// 1. Create the Renderable trait
pub trait Renderable {
    fn render(&self) -> String;
}

// 2. Create the Circle and Rectangle structs
pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// 3. Implement the trait for Circle and Rectangle
impl Renderable for Circle {
    fn render(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}
impl Renderable for Rectangle {
    fn render(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

// 4. Create the Canvas struct
pub struct Canvas {
    shapes: Vec<Box<dyn Renderable>>,
}

// 5. Implement the Canvas struct
impl Canvas {
    pub fn new() -> Canvas {
        Canvas { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        self.shapes.push(shape);
    }

    pub fn render_all(&self) -> Vec<String> {
        self.shapes.iter().map(|x| x.render()).collect()
    }
}

// Example usage
pub fn main() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    for shape in rendered_shapes {
        println!("{}", shape);
    }
}
