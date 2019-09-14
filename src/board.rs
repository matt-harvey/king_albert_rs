use std::fmt;

use card::{Card, Rank, Suit};
use deck::Deck;
use victory_state::VictoryState;

pub struct Board {
    foundations: Vec<Foundation>,
    columns: Vec<Column>,
    hand: Vec<SpotInHand>,
}

impl Board {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let foundations = Suit::iterator().map(|suit| Foundation::new(*suit)).collect();

        let columns = (0..9).map(|i| {
            let mut column = Column::new();
            for _ in 1..(i + 2) {
                column.receive(deck.deal());
            }
            column
        }).collect();

        let hand = (0..7).map(|_| SpotInHand::new(deck.deal())).collect();

        Self { foundations: foundations, columns: columns, hand: hand }
    }

    pub fn victory_state(&self) -> VictoryState {
        if self.foundations.iter().all(|f| f.top_rank.unwrap_or(0) == 13) {
            VictoryState::Won
        } else {
            VictoryState::Ongoing
        }
    }

    fn mut_location_at(&mut self, label: char) -> &mut Location {
        match label {
            'a' ..= 'd' => &mut self.foundations[label as usize - 'a' as usize],
            'e' ..= 'm' => &mut self.columns[label as usize - 'e' as usize],
            'n' ..= 't' => &mut self.hand[label as usize - 'n' as usize],
            _           => panic!("Label outside range"),
        }
    }

    fn location_at(&self, label: char) -> &Location {
        match label {
            'a' ..= 'd' => &self.foundations[label as usize - 'a' as usize],
            'e' ..= 'm' => &self.columns[label as usize - 'e' as usize],
            'n' ..= 't' => &self.hand[label as usize - 'n' as usize],
            _           => panic!("Label outside range"),
        }
    }

    pub fn execute(&mut self, movement: Movement) {
        if !self.permits(movement) {
            panic!("Illegal move");
        }
        let card = self.mut_location_at(movement.origin).give_card();
        self.mut_location_at(movement.destination).receive(card);
    }

    pub fn permits(&self, movement: Movement) -> bool {
        let origin = self.location_at(movement.origin);
        let destination = self.location_at(movement.destination);
        match origin.active_card() {
            Some(card) => origin.can_give_card() && destination.can_receive(&card),
            None       => false,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fds = &self.foundations;
        write!(f, "                           a    b    c    d\n")?;
        write!(f, "____________________________________________\n")?;
        write!(f, "                          {}  {}  {}  {}\n\n\n", fds[0], fds[1], fds[2], fds[3])?;
        write!(f, "  e    f    g    h    i    j    k    l    m\n")?;
        write!(f, "____________________________________________\n")?;

        let mut i = 0;
        while !self.columns.iter().all(|c| c.cards.len() < i) {
            write!(f, "{}  {}  {}  {}  {}  {}  {}  {}  {}\n",
                self.columns[0].printable_card_at(i),
                self.columns[1].printable_card_at(i),
                self.columns[2].printable_card_at(i),
                self.columns[3].printable_card_at(i),
                self.columns[4].printable_card_at(i),
                self.columns[5].printable_card_at(i),
                self.columns[6].printable_card_at(i),
                self.columns[7].printable_card_at(i),
                self.columns[8].printable_card_at(i))?;

            i += 1;
        }

        write!(f, "\n")?;
        write!(f, "  n    o    p    q    r    s    t\n")?;
        write!(f, "____________________________________________\n")?;
        let h = &self.hand;
        write!(f, "{}  {}  {}  {}  {}  {}  {}  \n", h[0], h[1], h[2], h[3], h[4], h[5], h[6])
    }
}

trait Location {
    fn can_receive(&self, card: &Card) -> bool;
    fn receive(&mut self, card: Card);
    fn can_give_card(&self) -> bool;
    fn give_card(&mut self) -> Card;
    fn active_card(&self) -> Option<Card>;
}

struct Foundation {
    suit:     Suit,
    top_rank: Option<Rank>,
}

impl Foundation {
    fn new(suit: Suit) -> Self {
        Self { suit: suit, top_rank: None }
    }
    fn next_rank(&self) -> Rank {
        self.top_rank.unwrap_or(0) + 1
    }
}

impl fmt::Display for Foundation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.top_rank {
          None       => write!(f, "  {}", self.suit),
          Some(rank) => write!(f, "{}", Card::new(self.suit, rank)),
        }
    }
}

impl Location for Foundation {
    fn can_receive(&self, card: &Card) -> bool {
        (card.suit() == self.suit) && (card.rank() == self.next_rank())
    }
    fn receive(&mut self, card: Card) {
        self.top_rank = Some(card.rank());
    }
    fn can_give_card(&self) -> bool {
        false
    }
    fn give_card(&mut self) -> Card {
        match self.top_rank {
            None       => panic!(),
            Some(rank) => {
                self.top_rank = Some(rank - 1);
                Card::new(self.suit, rank)
            },
        }
    }
    fn active_card(&self) -> Option<Card> {
        match self.top_rank {
            None       => None,
            Some(rank) => Some(Card::new(self.suit, rank)),
        }
    }
}

struct Column {
    cards: Vec<Card>,
}

impl Column {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }
    fn printable_card_at(&self, i: usize) -> String {
        match self.cards.get(i) {
            Some(card) => card.to_string(),
            None       => String::from("   "),  // TODO bleugh
        }
    }
}

impl Location for Column {
    fn can_give_card(&self) -> bool {
        !self.cards.is_empty()
    }
    fn give_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
    fn can_receive(&self, card: &Card) -> bool {
        match self.active_card() {
            Some(active_card) =>
                (active_card.color() != card.color()) && (active_card.rank() == card.rank() + 1),
            None => true
        }
    }
    fn receive(&mut self, card: Card) {
        self.cards.push(card);
    }
    fn active_card(&self) -> Option<Card> {
        match self.cards.last() {
            Some(card) => Some(*card),
            None       => None,
        }
    }
}

struct SpotInHand {
    card: Option<Card>,
}

impl SpotInHand {
    fn new(card: Card) -> Self {
        Self { card: Some(card) }
    }
}

impl fmt::Display for SpotInHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.card {
            None       => write!(f, "   "),
            Some(card) => write!(f, "{}", card),
        }
    }
}

impl Location for SpotInHand {
    fn can_give_card(&self) -> bool {
        self.card.is_some()
    }
    fn give_card(&mut self) -> Card {
        match self.card {
            Some(c) => {
                let ret = c.clone();
                self.card = None;
                ret
            },
            None => panic!(),
        }
    }
    fn can_receive(&self, _card: &Card) -> bool {
        false
    }
    fn receive(&mut self, card: Card) {
        self.card = Some(card);
    }
    fn active_card(&self) -> Option<Card> {
        self.card
    }
}

#[derive(Copy, Clone)]
pub struct Movement {
    pub origin: char,
    pub destination: char,
}
