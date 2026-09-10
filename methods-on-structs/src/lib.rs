// 1. Define the struct
pub struct Counter {
    count: i32,
}

// 2. Implement the associated function and methods
impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) -> () {
        self.count += 1
    }

    pub fn decrement(&mut self) -> () {
        self.count -= 1
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }
}

// Example use case
pub fn main() {
    let mut counter = Counter::new();

    counter.increment();
    assert_eq!(counter.get_count(), 1);

    counter.increment();
    counter.increment();
    assert_eq!(counter.get_count(), 3);

    counter.decrement();
    assert_eq!(counter.get_count(), 2);

    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get_count(), 0);
}
