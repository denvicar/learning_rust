use std::{collections::VecDeque, fmt::Display};

use crate::snake::{Direction, Snake};

pub struct Game {
    height: usize,
    width: usize,
    snake: Snake,
}

impl Game {
    pub fn new(height: usize, width: usize) -> Self {
        let snake_body = VecDeque::from([(0,0),(0,1),(0,2),(0,3),(1,3)]);
        
        Game { height, width, snake: Snake::new(snake_body, Direction::UP) }
    }

    pub fn game_matrix(&self) -> Vec<Vec<u8>> {
        let mut mat = vec![vec![0;self.width];self.height];
        for (x,y) in self.snake.body() {
            mat[*x][*y] = 1;
        }
        let (x,y) = self.snake.head();
        mat[x][y] = 2;

        mat
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matrix = self.game_matrix();
        let mut out = String::with_capacity((self.width+2)*(self.height+2)+self.height+2);
        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push('+');
        out.push('\n');

        for row in matrix {
            out.push('|');
            for spot in row {
                out.push(match spot {
                    0 => ' ',
                    1 => 'X',
                    2 => 'O',
                    _ => unreachable!()
                });
            }
            out.push_str("|\n")
        }

        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push('+');
        out.push('\n');

        write!(f, "{}", out)
    }
}