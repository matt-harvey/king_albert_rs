extern crate rand;

use card::{Card, Suit};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 1..=13 {
            for suit in Suit::iterator() {
                cards.push(Card::new(*suit, rank))
            }
        }
        Deck { cards: cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = &mut thread_rng();
        self.cards.shuffle(& mut rng);
    }
    pub fn deal(&self, index: usize) -> Card {
        self.cards[index]
    }
}
