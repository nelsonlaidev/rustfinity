pub fn calculate_area() -> u32 {
    let width = 10;
    let height = 20;
    prints_values(width, height);

    width * height
}

pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}
