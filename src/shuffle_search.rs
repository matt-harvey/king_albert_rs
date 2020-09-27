use board::{Board, Movement};
use deck::Deck;
use rand::{seq::IteratorRandom, thread_rng};
use std::{sync::{mpsc, Arc}, thread};
use victory_state::VictoryState;

const MAX_MOVES: u32 = 700;
const MAX_THREADS: u32 = 8; // FIXME Should be based on the number of cores available
const MAX_REPEATS: u32 = 3200;

type WinningMoves = Vec<Movement>;

fn shuffled_deck() -> Deck {
    let mut deck = Deck::new();
    deck.shuffle();
    deck
}

pub fn find_winnable_deck() -> (Deck, Vec<Movement>) {
    let mut i = 0;
    loop {
        println!("\n{} decks tried", i); // DEBUG
        let deck = shuffled_deck();
        let deck_ref = Arc::new(Box::new(deck.clone()));
        if let Some(movements) = can_find_win(deck_ref) {
            return (deck, movements);
        } else {
            i += 1
        }
    }
}

fn make_random_move(board: &mut Board, offset: u32) -> Option<Movement> {
    let permitted_moves = board.permitted_moves();
    if permitted_moves.is_empty() {
        None
    } else {
        let mut rng = thread_rng();
        let range = 0..permitted_moves.len();
        let chosen_index = range.into_iter().choose(&mut rng).unwrap();
        let index = (chosen_index + (offset as usize)) % permitted_moves.len();
        let movement = permitted_moves[index];
        board.execute(&movement);
        Some(movement)
    }
}

fn can_find_win(deck: Arc<Box<Deck>>) -> Option<Vec<Movement>> {
    for j in 0..MAX_REPEATS {
        print!("\r{} repeats", j); // DEBUG
        let (sender, receiver) = mpsc::channel();
        for i in 0..MAX_THREADS {
            let sender_n = sender.clone();
            let deck_clone: Arc<Box<Deck>> = Arc::clone(&deck);
            thread::spawn(move || {
                try_deck(sender_n, deck_clone, i);
            });
        }
        for _ in 0..MAX_THREADS {
            let result = receiver.recv().unwrap();
            if let Some(_) = result {
                return result;
            }
        }
    }
    None
}

fn try_deck(sender: mpsc::Sender<Option<WinningMoves>>, deck: Arc<Box<Deck>>, offset: u32) {
    let mut board = Board::new(deck);
    let mut moves = Vec::new();
    for j in 0..MAX_MOVES {
        if let Some(movement) = make_random_move(&mut board, offset + j) {
            moves.push(movement);
            match board.victory_state() {
                VictoryState::Won => {
                    let _ = sender.send(Some(moves)); // FIXME ignoring potential send on closed channel
                    return;
                },
                VictoryState::Ongoing => {},
            }
        } else {
            let _ = sender.send(None); // FIXME ignoring potential send on closed channel
            return;
        }
    }
    let _ = sender.send(None); // FIXME ignoring potential send on closed channel
    return;
}
