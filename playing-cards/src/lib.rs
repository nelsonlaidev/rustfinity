pub enum Card {
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    match card {
        Card::King => String::from("King"),
        Card::Queen => String::from("Queen"),
        Card::Jack => String::from("Jack"),
        Card::Numbered(val, suit) => format!("{val} of {suit}"),
    }
}
