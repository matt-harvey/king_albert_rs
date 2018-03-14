extern crate rand;

use rand::Rng;

use card::{Card, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 1..14 {
            for suit in Suit::iterator() {
                cards.push(Card::new(*suit, rank))
            }
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
