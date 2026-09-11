use core::str;

pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        String::from("Beep boop")
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => {
            // Return a Dog instance here
            Box::new(Dog {
                name: String::from("Rex"),
                breed: String::from("Labrador"),
            })
        }
        "robot" => {
            // Return a Robot instance here
            Box::new(Robot {
                model: String::from("T-1000"),
                purpose: String::from("Helper"),
            })
        }
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
