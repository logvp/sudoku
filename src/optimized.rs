use log::debug;

use crate::{Board, Digit, SudokuBox, SudokuColumn, SudokuRow, SudokuRule};

fn check_board_one(board: &Board, index: usize) -> bool {
    SudokuRow {}.check_one(board, index)
        && SudokuColumn {}.check_one(board, index)
        && SudokuBox {}.check_one(board, index)
}

pub fn solve_standard(board: Board) -> Option<Board> {
    let Some(check_idx) = board.first_open_index() else {
        debug!("Board is already solved!");
        return Some(board);
    };
    for digit in Digit::DIGITS {
        let mut solution = board.clone();
        solution.board[check_idx] = Some(digit);
        if !check_board_one(&solution, check_idx) {
            continue;
        }
        if let Some(solved) = solve_standard(solution) {
            return Some(solved);
        }
    }
    None
}
