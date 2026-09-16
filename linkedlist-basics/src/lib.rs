use std::collections::LinkedList;

pub fn create_list(items: &[i32]) -> LinkedList<i32> {
    LinkedList::from_iter(items.iter().cloned())
}

pub fn add_front<T>(list: &mut LinkedList<T>, item: T) {
    list.push_front(item);
}

pub fn add_back<T>(list: &mut LinkedList<T>, item: T) {
    list.push_back(item);
}

pub fn remove_front<T>(list: &mut LinkedList<T>) -> Option<T> {
    list.pop_front()
}

pub fn remove_back<T>(list: &mut LinkedList<T>) -> Option<T> {
    list.pop_back()
}

pub fn peek_front<T>(list: &LinkedList<T>) -> Option<&T> {
    list.front()
}

pub fn peek_back<T>(list: &LinkedList<T>) -> Option<&T> {
    list.back()
}

pub fn move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool {
    let mut found = false;
    let mut new_list = LinkedList::new();
    let mut item_to_move = None;

    while let Some(item) = list.pop_front() {
        if !found && &item == value {
            found = true;
            item_to_move = Some(item);
        } else {
            new_list.push_back(item);
        }
    }

    if let Some(item) = item_to_move {
        new_list.push_front(item);
    }

    *list = new_list;

    found
}

pub fn concat_lists<T>(mut list1: LinkedList<T>, list2: LinkedList<T>) -> LinkedList<T> {
    list1.extend(list2);
    list1
}

pub fn main() {
    let list = create_list(&[1, 2, 3, 4, 5]);
    println!("Created list with {} elements", list.len());
    println!("Front: {:?}, Back: {:?}", list.front(), list.back());

    let mut list = LinkedList::new();
    add_back(&mut list, 2);
    add_front(&mut list, 1);
    add_back(&mut list, 3);
    println!(
        "After adding 1 at front, 2 and 3 at back: {:?}",
        list.iter().collect::<Vec<_>>()
    );

    let mut list = create_list(&[10, 20, 30]);
    println!("Removed from front: {:?}", remove_front(&mut list));
    println!("Removed from back: {:?}", remove_back(&mut list));
    println!("Remaining: {:?}", list.iter().collect::<Vec<_>>());

    let mut list = create_list(&[1, 2, 3, 4, 5]);
    println!(
        "Before move_to_front(&3): {:?}",
        list.iter().collect::<Vec<_>>()
    );
    move_to_front(&mut list, &3);
    println!(
        "After move_to_front(&3): {:?}",
        list.iter().collect::<Vec<_>>()
    );

    let list1 = create_list(&[1, 2]);
    let list2 = create_list(&[3, 4]);
    let combined = concat_lists(list1, list2);
    println!("Combined list: {:?}", combined.iter().collect::<Vec<_>>());
}
