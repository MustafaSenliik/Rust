use rand::{thread_rng, seq::SliceRandom}; 

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = vec![];
        let suits = ["Hearts", "Diamonds", "Clubs"];
        let values = ["2", "3", "4"];

        for suit in suits {
            for value in values {
                let card = format!("{} {}", suit, value);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    // Shuffle function to randomize the deck
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng); // Using SliceRandom to shuffle
    }
}

fn main() {
    let mut deck = Deck::new(); // The deck must be mutable so it can be shuffled
    deck.shuffle(); // Shuffle the deck

    println!("Shuffled Deck: {:#?}", deck); // Print the shuffled deck
}
