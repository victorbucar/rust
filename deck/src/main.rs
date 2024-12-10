
use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck{
    fn new() -> Self{

        let suits = ["Hears", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits{
            for value in values{
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck{cards}
    }
    fn shuffle(&mut self) {
       let mut rng = thread_rng();
       self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num_cards: usize) -> Vec<String>{
        self.cards.split_off(self.cards.len() - num_cards)
    }
}
fn main() {
    let mut deck = Deck::new();
    
    println!("Here's your deck: {:#?}", deck);
    deck.shuffle();
    println!("Here's your shuffled deck: {:#?}", deck);
    let hand = deck.deal(4);
    println!("Here's your hand: {:#?}", hand);
}
