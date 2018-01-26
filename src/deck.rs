extern crate rand;

use rand::Rng;

use card::Card;
use card::Suit;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 1..14 {
            cards.push(Card { rank: rank, suit: Suit::Spades });
            cards.push(Card { rank: rank, suit: Suit::Hearts });
            cards.push(Card { rank: rank, suit: Suit::Diamonds });
            cards.push(Card { rank: rank, suit: Suit::Clubs });
        }
        Deck { cards: cards }
    }
    pub fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.cards);
    }
    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
