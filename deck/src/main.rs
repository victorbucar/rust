#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
fn main() {
    // list of suit
    // list of value

    let suits = ["Hears", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];

    for suit in suits{
        for value in values{
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck{cards: vec![]};

    println!("Here's your deck: {:?}", deck);
}
