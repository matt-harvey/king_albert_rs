extern crate king_albert;

use std::io;
use std::io::Write;

use king_albert::board::Board;
use king_albert::board::Movement;
use king_albert::victory_state::VictoryState;

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
    let clear_screen = "\x1b[2J\x1b[1;1H";
    println!("{}\n{}", clear_screen, board);

    while board.victory_state() != VictoryState::Won {
        let mut movement = Movement { origin: 'a', destination: 'a' }; // dummy

        loop {
            let c = get_char("\nEnter position to move FROM (labelled e-t): ");
            if c >= 'e' && c <= 't' {
                movement.origin = c;
                break;
            }
            println!("You must enter a letter from e to t");
        }

        loop {
            let c = get_char("\nEnter position to move TO (labelled a-m): ");
            if c >= 'a' && c <= 'm' {
                movement.destination = c;
                break;
            }
            println!("You must enter a letter from a to m");
        }

        if board.permits(movement) {
            board.execute(movement);
            println!("{}\n{}", clear_screen, board);
        } else {
            println!("That move is not permitted, try again!");
        }
    }
    println!("{}\n{}\n{}", clear_screen, board, "You won, hooray!");
}
