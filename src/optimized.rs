use log::debug;

use crate::{Board, Digit, DigitSet};

pub fn solve_standard(board: Board) -> Option<Board> {
    let Some(empty_idx) = board.first_open_index() else {
        debug!("Board is already solved!");
        return Some(board);
    };
    let mut seen = DigitSet::new();
    let (x, y) = board.xy(empty_idx);
    for i in 0..board.width() {
        if let Some(digit) = board.get(i, y) {
            seen.set(digit);
        }
    }
    for j in 0..board.height() {
        if let Some(digit) = board.get(x, j) {
            seen.set(digit);
        }
    }
    let box_start_x = x - (x % 3);
    let box_start_y = y - (y % 3);
    for i in box_start_x..(box_start_x + 3) {
        for j in box_start_y..(box_start_y + 3) {
            if let Some(digit) = board.get(i, j) {
                seen.set(digit);
            }
        }
    }
    seen.invert();
    for digit in seen {
        let mut solution = board.clone();
        solution.board[empty_idx] = Some(digit);
        if let Some(solved) = solve_standard(solution) {
            return Some(solved);
        }
    }
    None
}
