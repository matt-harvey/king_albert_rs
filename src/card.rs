use std::fmt;
use std::slice::Iter;

pub type Rank = u8;

#[derive(Copy, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit: suit, rank: rank }
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
    pub fn rank(&self) -> Rank {
        self.rank
    }
    pub fn color(&self) -> Color {
        self.suit().color()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.rank() {
            1       => write!(f, " A{}", self.suit()),
            2 ... 9 => write!(f, " {}{}", self.rank(), self.suit()),
            10      => write!(f, "10{}", self.suit()),
            11      => write!(f, " J{}", self.suit()),
            12      => write!(f, " Q{}", self.suit()),
            13      => write!(f, " K{}", self.suit()),
            _       => panic!(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    Red,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn iterator() -> Iter<'static, Self> {
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        SUITS.into_iter()
    }
    fn color(self) -> Color {
        match self {
            Suit::Spades | Suit::Clubs    => Color::Black,
            Suit::Hearts | Suit::Diamonds => Color::Red,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spades   => write!(f, "\u{2660}"),
            Suit::Hearts   => write!(f, "\u{2661}"),
            Suit::Diamonds => write!(f, "\u{2662}"),
            Suit::Clubs    => write!(f, "\u{2663}"),
        }
    }
}
