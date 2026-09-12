use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub fn push<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    data.borrow_mut().push(element);
}

pub fn iterate_and_print_shared_data<T: Display>(data: Rc<RefCell<Vec<T>>>) {
    for item in data.borrow().iter() {
        println!("{}", item);
    }
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    *data.borrow_mut() += 1;
}

pub fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    push(Rc::clone(&shared_data), 4);
    push(Rc::clone(&shared_data), 5);

    println!("Shared Data:");
    iterate_and_print_shared_data(Rc::clone(&shared_data));

    let my_num = Rc::new(RefCell::new(5));
    plus_one(Rc::clone(&my_num));
    assert_eq!(*my_num.borrow(), 6, "Value was not incremented correctly.");
}
