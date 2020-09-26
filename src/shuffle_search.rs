use board::{Board};
use deck::Deck;
use rand::{seq::IteratorRandom, thread_rng};
use victory_state::VictoryState;

const MAX_MOVES: u32 = 700;
const MAX_TRIALS: u32 = 25_600;

#[derive(PartialEq, Clone, Copy)]
enum TrialResult {
    Won,
    Lost,
    MaxMovesReached,
}

fn shuffled_deck() -> Deck {
    let mut deck = Deck::new();
    deck.shuffle();
    deck
}

pub fn find_winnable_deck() -> Deck {
    let mut i = 0;
    loop {
        println!("\n{} decks tried", i);
        let deck = shuffled_deck();
        if can_find_win(&deck) {
            return deck;
        }
        i += 1;
    }
}

fn make_random_move(board: &mut Board) -> bool {
    let permitted_moves = board.permitted_moves();
    if permitted_moves.is_empty() {
        false
    } else {
        let mut rng = thread_rng();
        let movement = permitted_moves.iter().choose(&mut rng).unwrap();
        board.execute(movement);
        true
    }
}

fn can_find_win(deck: &Deck) -> bool {
    for i in 0..MAX_TRIALS {
        print!("\rtrial no. {}", i); // DEBUG
        if try_deck(&deck) == TrialResult::Won {
            println!("\nFound a win.");
            return true;
        }
    }
    false
}

fn try_deck(deck: &Deck) -> TrialResult {
    let mut board = Board::new(&deck);
    for _ in 0..MAX_MOVES {
        let result = make_random_move(&mut board);
        if result {
            match board.victory_state() {
                VictoryState::Won => return TrialResult::Won,
                VictoryState::Ongoing => {},
            }
        } else {
            return TrialResult::Lost;
        }
    }
    TrialResult::MaxMovesReached
}
