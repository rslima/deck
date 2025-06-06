use std::vec;
use rand::{seq::SliceRandom, rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut cards = vec![];

        for suit in suits {
            for rank in ranks {
                let card = format!("{} of {}", rank, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
    fn draw(&mut self, num_cards: usize) -> Vec<String> {
        // Need to handle case where num_cards is bigger than deck length
        self.cards.split_off(self.cards.len() - num_cards)
        
    }
}

fn main() {

    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.draw(5);
    println!("Here's your hand: {:#?}", hand);
}
