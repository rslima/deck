use std::vec;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}



fn main() {

    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];    

    let mut cards = vec![];

    for suit in suits {
        for rank in ranks {
            let card = format!("{} of {}", rank, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };

    println!("Here's your deck: {:#?}", deck);
}
