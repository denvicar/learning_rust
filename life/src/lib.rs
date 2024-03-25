//! Simple implementation in Rust of the game of life.
//! 
//! Just run the game, it will start with a random state and evolve from there.
//! 
//! ```
//! use life::{random_board,next_board_state, render};
//! let mut board = random_board(20,20);
//! loop {
//! #   let old = board.clone();
//!     render(&board, &mut std::io::stdout());
//!     board = next_board_state(&board);
//! #   assert_ne!(old,board);
//! #   break;
//! }
//! ```
//!

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Returns a dead board of given width and height
/// 
/// A dead board is a board composed of 0s (or false given that the board is made of booleans)
fn dead_board(height: usize, width: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height]
}

/// Returns a random board of given size
/// 
/// ```
/// use life::random_board;
/// let board = random_board(20,20);
/// assert!(board.len()==20);
/// assert!(board[0].len()==20);
/// ```
pub fn random_board(height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut board = dead_board(height, width);
    for row in &mut board {
        for col in row {
            *col = rand::random();
        }
    }
    board
}

/// Renders a board to a given output stream
pub fn render(board: &Vec<Vec<bool>>, mut output: impl io::Write) {
    let mut out = String::with_capacity(board.len() * board[0].len() + board.len());
    for row in board {
        for col in row {
            if *col {
                out.push('█');
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    write!(output, "{}", out).expect("error while trying to print board");
}

/// Calculates next board state given a set of rules.
/// 
/// Rules are:
/// 
/// *(note: a cell neighbours are the 8 adjacent and diagonal cells)*
/// 
/// *(edge cells have only 5 neighbours; corner cells have only 3)*
/// 
/// 1. Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
/// 2. Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
/// 3. Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
/// 4. Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
/// 
/// ```
/// use life::next_board_state;
/// let board = vec![vec![true,false],vec![true,true]];
/// let expected = vec![vec![true, true], vec![true, true]];
/// assert_eq!(expected, next_board_state(&board));
/// ``` 
pub fn next_board_state(old: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new = dead_board(old.len(), old[0].len());
    let mut neighbours_count;

    for (r_index, row) in old.iter().enumerate() {
        for (c_index, cell) in row.iter().enumerate() {
            neighbours_count = get_neighbours_count(old, (r_index, c_index));
            if *cell && (neighbours_count == 2 || neighbours_count == 3) {
                new[r_index][c_index] = true;
            }
            if !*cell && neighbours_count == 3 {
                new[r_index][c_index] = true;
            }
        }
    }

    new
}

fn get_neighbours_count(board: &[Vec<bool>], cell: (usize, usize)) -> u32 {
    let alives = board
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter(|&(col_index, _)| {
                    col_index.wrapping_sub(1) == cell.1
                        || col_index + 1 == cell.1
                        || col_index == cell.1
                })
                .map(|(_, &val)| val)
                .collect::<Vec<bool>>()
        })
        .enumerate()
        .filter(|(row_index, _)| {
            *row_index + 1 == cell.0 || row_index.wrapping_sub(1) == cell.0 || *row_index == cell.0
        })
        .flat_map(|(_, row)| row)
        .filter(|val| *val)
        .count();

    if board[cell.0][cell.1] {
        alives.saturating_sub(1) as u32
    } else {
        alives as u32
    }
}

/// Load a starting board from a file.
/// 
/// The board should be a plain text file with rows made of 1s and 0s to 
/// indicate respectively an alive or dead cell.
pub fn load_from_file<F: AsRef<Path>>(path: F) -> Vec<Vec<bool>> {
    let mut board = Vec::new();
    let mut row = Vec::new();
    let mut row_size = None;
    let f = File::open(path).expect("cannot open file");
    for line in io::BufReader::new(f).lines().map_while(Result::ok) {
        line.trim()
            .chars()
            .map(|c| c.to_digit(2).expect("file is badly formatted"))
            .for_each(|c| row.push(c == 1));
        if let Some(sz) = row_size {
            if sz != row.len() {
                panic!("file is badly formatted");
            }
        } else {
            row_size = Some(row.len());
        }
        board.push(row.clone());
        row.clear();
    }
    board
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{dead_board, next_board_state, random_board, render};

    #[test]
    pub fn test_dead_board() {
        assert_eq!(dead_board(5, 8), vec![vec![false; 8]; 5]);
    }

    #[test]
    pub fn test_random_board() {
        let random_board = random_board(5, 8);
        assert!(random_board.len() == 5);
        assert!(random_board[0].len() == 8);
        assert_ne!(random_board, vec![vec![false; 8]; 5]);
    }

    #[test]
    pub fn test_render() {
        let board = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![true, true, false],
        ];
        let mut output = Vec::with_capacity(12);
        render(&board, &mut output);
        let expected = " █ \n█ █\n██ \n";
        assert_eq!(output, expected.as_bytes());
    }

    #[test]
    pub fn test_next_board_state() {
        let board = vec![
            vec![true, false, true, true],
            vec![false, false, true, false],
            vec![true, false, true, false],
            vec![true, true, true, false],
        ];
        let expected = vec![
            vec![false, true, true, true],
            vec![false, false, true, false],
            vec![true, false, true, true],
            vec![true, false, true, false],
        ];
        assert_eq!(expected, next_board_state(&board));
    }
}
