use std::collections::VecDeque;

pub fn create_queue<T: Clone>(items: &[T]) -> VecDeque<T> {
    items.iter().cloned().collect()
}

pub fn enqueue<T>(queue: &mut VecDeque<T>, item: T) {
    queue.push_back(item);
}

pub fn dequeue<T>(queue: &mut VecDeque<T>) -> Option<T> {
    queue.pop_front()
}

pub fn peek_front<T>(queue: &VecDeque<T>) -> Option<&T> {
    queue.front()
}

pub fn peek_back<T>(queue: &VecDeque<T>) -> Option<&T> {
    queue.back()
}

pub fn rotate_left<T>(queue: &mut VecDeque<T>, n: usize) {
    if n == 0 || queue.len() == 0 {
        return;
    }

    if n > queue.len() {
        queue.rotate_left(n % queue.len());
        return;
    }

    queue.rotate_left(n);
}

pub fn rotate_right<T>(queue: &mut VecDeque<T>, n: usize) {
    if n == 0 || queue.len() == 0 {
        return;
    }

    if n > queue.len() {
        queue.rotate_right(n % queue.len());
        return;
    }

    queue.rotate_right(n);
}

// Example usage
pub fn main() {
    // Create a task queue
    let mut tasks = create_queue(&["task1", "task2", "task3"]);
    println!("Initial queue: {:?}", tasks);

    // Add a new task
    enqueue(&mut tasks, "task4");
    println!("After enqueue: {:?}", tasks);

    // Process tasks (FIFO order)
    while let Some(task) = dequeue(&mut tasks) {
        println!("Processing: {}", task);
    }

    // Demonstrate rotation
    let mut numbers = create_queue(&[1, 2, 3, 4, 5]);
    println!("\nBefore rotation: {:?}", numbers);

    rotate_left(&mut numbers, 2);
    println!("After rotate_left(2): {:?}", numbers);

    rotate_right(&mut numbers, 2);
    println!("After rotate_right(2): {:?}", numbers);
}
