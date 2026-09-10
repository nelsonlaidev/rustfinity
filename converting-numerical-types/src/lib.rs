pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: TrafficLight) -> String {
    match light {
        TrafficLight::Red => String::from("Stop"),
        TrafficLight::Yellow => String::from("Caution"),
        TrafficLight::Green => String::from("Go"),
    }
}

pub fn numerical_type_conversion(n: i32) -> u32 {
    n as u32
}
