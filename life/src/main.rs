use std::io;

use life::{next_board_state, render};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut board = if let Some(path) = args.next() {
        life::load_from_file(path)
    } else {
        life::random_board(30, 30)
    };
    loop {
        render(&board, &mut io::stdout().lock());
        let new = next_board_state(&board);
        if new == board {
            println!("stable state");
            break;
        }
        board = new;
    }
}
