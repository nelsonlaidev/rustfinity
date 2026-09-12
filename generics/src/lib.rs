pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    pub fn get_item(&self) -> &T {
        &self.item
    }
}

pub fn main() {
    let item_1 = ItemContainer { item: 42 };
    assert_eq!(*item_1.get_item(), 42);

    let item_2 = ItemContainer {
        item: String::from("Hello"),
    };

    assert_eq!(item_2.get_item(), "Hello");
}
