use std::io;

mod snake;
mod game;
mod apple;
mod input;

fn main() {
    let game = game::Game::new(10, 10);
    loop {
        print!("{}", game);
        io::stdin().reaex
    }
}
