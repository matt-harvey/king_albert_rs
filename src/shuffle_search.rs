use board::{Board, Movement};
use deck::Deck;
use rand::{seq::IteratorRandom, thread_rng};
use victory_state::VictoryState;

const MAX_MOVES: u32 = 700;
const MAX_TRIALS: u32 = 25_600;

#[derive(Clone)]
enum TrialResult {
    Won(Vec<Movement>),
    Lost,
    MaxMovesReached,
}

fn shuffled_deck() -> Deck {
    let mut deck = Deck::new();
    deck.shuffle();
    deck
}

pub fn find_winnable_deck() -> (Deck, Vec<Movement>) {
    let mut i = 0;
    loop {
        println!("\n{} decks tried", i);
        let deck = shuffled_deck();
        if let Some(movements) = can_find_win(&deck) {
            return (deck, movements);
        } else {
            i += 1
        }
    }
}

fn make_random_move(board: &mut Board) -> Option<Movement> {
    let permitted_moves = board.permitted_moves();
    if permitted_moves.is_empty() {
        None
    } else {
        let mut rng = thread_rng();
        let movement = permitted_moves.iter().choose(&mut rng).unwrap();
        board.execute(movement);
        Some(*movement)
    }
}

fn can_find_win(deck: &Deck) -> Option<Vec<Movement>> {
    for i in 0..MAX_TRIALS {
        print!("\rtrial no. {}", i); // DEBUG
        if let TrialResult::Won(moves) = try_deck(&deck) {
            println!("\nFound a win.");
            return Some(moves);
        }
    }
    None
}

fn try_deck(deck: &Deck) -> TrialResult {
    let mut board = Board::new(&deck);
    let mut moves = Vec::new();
    for _ in 0..MAX_MOVES {
        if let Some(movement) = make_random_move(&mut board) {
            moves.push(movement);
            match board.victory_state() {
                VictoryState::Won => return TrialResult::Won(moves),
                VictoryState::Ongoing => {},
            }
        } else {
            return TrialResult::Lost;
        }
    }
    TrialResult::MaxMovesReached
}
