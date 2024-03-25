use std::io;

use crate::snake::{Direction, Snake};

pub fn get_next_position(snake: Snake) -> (usize,usize) {
    let ch = get_user_input();

    match ch {
        b'w'|b'W' => snake.head() + Direction::UP,
        b's'|b'S' => snake.head() + Direction::DOWN,
        b'd'|b'D' => snake.head() + Direction::RIGHT,
        b'a'|b'A' => snake.head() + Direction::LEFT,
        _ => panic!("invalid move")
    }
}

fn get_user_input() -> u8 {
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    line.as_bytes()[0]
}