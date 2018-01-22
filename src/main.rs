extern crate rand;

use std::fmt;
use std::io;
use std::io::Write;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Black,
    Red,
}

#[derive(PartialEq, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
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

type Rank = u8;

#[derive(Copy, Clone)]
struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    fn color(&self) -> Color {
        self.suit.color()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.rank {
            1       => write!(f, " A{}", self.suit),
            2 ... 9 => write!(f, " {}{}", self.rank, self.suit),
            10      => write!(f, "10{}", self.suit),
            11      => write!(f, " J{}", self.suit),
            12      => write!(f, " Q{}", self.suit),
            13      => write!(f, " K{}", self.suit),
            _       => panic!(),
        }
    }
}

trait Location {
    fn can_move_to(&self, card: &Card) -> bool;
    fn move_to(&mut self, card: Card);
    fn can_move_from(&self) -> bool;
    fn move_from(&mut self) -> Card;
    fn active_card(&self) -> Option<Card>;
}

struct Foundation {
    suit:     Suit,
    top_rank: Option<Rank>,
}

impl fmt::Display for Foundation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.top_rank {
          None       => write!(f,"  {}", self.suit),
          Some(rank) => write!(f, "{}", Card { rank: rank, suit: self.suit }),
        }
    }
}

impl Location for Foundation {
    fn can_move_to(&self, card: &Card) -> bool {
        if card.suit != self.suit {
            return false;
        }
        match self.top_rank {
            None       => card.rank == 1,
            Some(rank) => card.rank == rank + 1,
        }
    }
    fn move_to(&mut self, card: Card) {
        self.top_rank = Some(card.rank);
    }
    fn can_move_from(&self) -> bool {
        false
    }
    fn move_from(&mut self) -> Card {
        match self.top_rank {
            None       => panic!(),
            Some(rank) => {
                self.top_rank = Some(rank - 1);
                Card { suit: self.suit, rank: rank }
            },
        }
    }
    fn active_card(&self) -> Option<Card> {
        match self.top_rank {
            None       => None,
            Some(rank) => Some(Card { suit: self.suit, rank: rank }),
        }
    }
}

struct Column {
    cards: Vec<Card>,
}

impl Column {
    fn printable_card_at(&self, i: usize) -> String {
        match self.cards.get(i) {
            Some(card) => card.to_string(),
            None       => String::from("   "),  // TODO bleugh
        }
    }
}

impl Location for Column {
    fn can_move_from(&self) -> bool {
        !self.cards.is_empty()
    }
    fn move_from(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
    fn can_move_to(&self, card: &Card) -> bool {
        match self.active_card() {
            Some(active_card) =>
                (active_card.color() != card.color()) && (active_card.rank == card.rank + 1),
            None => true
        }
    }
    fn move_to(&mut self, card: Card) {
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

impl fmt::Display for SpotInHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.card {
            None       => write!(f, "   "),
            Some(card) => write!(f, "{}", card),
        }
    }
}

impl Location for SpotInHand {
    fn can_move_from(&self) -> bool {
        match self.card {
            Some(_) => true,
            None    => false,
        }
    }
    fn move_from(&mut self) -> Card {
        match self.card {
            Some(c) => {
                let ret = c.clone();
                self.card = None;
                ret
            },
            None => panic!(),
        }
    }
    fn can_move_to(&self, _card: &Card) -> bool {
        false
    }
    fn move_to(&mut self, card: Card) {
        self.card = Some(card);
    }
    fn active_card(&self) -> Option<Card> {
        self.card
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 1..14 {
            cards.push(Card { rank: rank, suit: Suit::Spades });
            cards.push(Card { rank: rank, suit: Suit::Hearts });
            cards.push(Card { rank: rank, suit: Suit::Diamonds });
            cards.push(Card { rank: rank, suit: Suit::Clubs });
        }
        Deck { cards: cards }
    }
    fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.cards);
    }
    fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

struct Board {
    foundations: [Foundation; 4],
    columns: [Column; 9],
    hand: [SpotInHand; 7],
}

impl Board {
    fn new() -> Board {
        let mut deck = Deck::new();
        deck.shuffle();
        let foundations = [
            Foundation { suit: Suit::Spades, top_rank: None },
            Foundation { suit: Suit::Hearts, top_rank: None },
            Foundation { suit: Suit::Diamonds, top_rank: None },
            Foundation { suit: Suit::Clubs, top_rank: None },
        ];
        let mut columns = [
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
            Column { cards: Vec::new() },
        ];
        let hand = [
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
            SpotInHand { card: Some(deck.deal()) },
        ];

        for (i, column) in columns.iter_mut().enumerate() {
            for _ in 1..(i + 2) {
                column.move_to(deck.deal());
            }
        }
        Board { foundations: foundations, columns: columns, hand: hand }
    }

    fn mut_location_at(&mut self, label: char) -> &mut Location {
        match label {
            'a' ... 'd' => &mut self.foundations[label as usize - 'a' as usize],
            'e' ... 'm' => &mut self.columns[label as usize - 'e' as usize],
            'n' ... 't' => &mut self.hand[label as usize - 'n' as usize],
            _           => panic!("Label outside range"),
        }
    }

    fn location_at(&self, label: char) -> &Location {
        match label {
            'a' ... 'd' => &self.foundations[label as usize - 'a' as usize],
            'e' ... 'm' => &self.columns[label as usize - 'e' as usize],
            'n' ... 't' => &self.hand[label as usize - 'n' as usize],
            _           => panic!("Label outside range"),
        }
    }

    fn execute(&mut self, m: Move) {
        if !self.permits(m) {
            panic!("Illegal move");
        }
        let card = self.mut_location_at(m.origin).move_from();
        self.mut_location_at(m.destination).move_to(card);
    }

    fn permits(&self, m: Move) -> bool {
        let origin = self.location_at(m.origin);
        let destination = self.location_at(m.destination);
        match origin.active_card() {
            Some(card) => origin.can_move_from() && destination.can_move_to(&card),
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

#[derive(Copy, Clone)]
struct Move {
    origin: char,
    destination: char,
}

fn get_char(prompt: &str) -> char {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Did not enter a correct string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        if let Ok(c) = input.parse::<char>() {
            return c;
        }
    }
}

fn main() {
    let mut board = Board::new();
    println!("{}[2J{}", 27 as char, board);

    loop {
        let mut m = Move { origin: 'a', destination: 'a' }; // dummy

        loop {
            let c = get_char("\nEnter position to move FROM (labelled e-t): ");
            if c >= 'e' && c <= 't' {
                m.origin = c;
                break;
            }
            println!("You must enter a letter from e to t");
        }

        loop {
            let c = get_char("\nEnter position to move TO (labelled a-m): ");
            if c >= 'a' && c <= 'm' {
                m.destination = c;
                break;
            }
            println!("You must enter a letter from a to m");
        }

        if board.permits(m) {
            board.execute(m);
            println!("{}[2J{}", 27 as char, board);
        } else {
            println!("That move is not permitted, try again!");
        }
    }
}
