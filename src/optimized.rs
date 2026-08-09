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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_solver() {
        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 2, 0, 9, 0, 0, 0,
            9, 7, 6, 0, 0, 0, 2, 0, 5,
            0, 0, 5, 6, 7, 0, 1, 0, 8,
            0, 8, 0, 9, 0, 0, 0, 0, 7,
            7, 0, 0, 4, 3, 8, 0, 0, 2,
            6, 0, 0, 0, 0, 7, 0, 8, 0,
            5, 0, 8, 0, 1, 2, 3, 0, 0,
            1, 0, 2, 0, 0, 0, 5, 7, 9,
            0, 0, 0, 5, 0, 3, 0, 0, 0,
        ]);
        #[rustfmt::skip]
        let solution: Board = Board::make([
            8, 4, 1, 2, 5, 9, 7, 3, 6,
            9, 7, 6, 3, 8, 1, 2, 4, 5,
            3, 2, 5, 6, 7, 4, 1, 9, 8,
            2, 8, 3, 9, 6, 5, 4, 1, 7,
            7, 1, 9, 4, 3, 8, 6, 5, 2,
            6, 5, 4, 1, 2, 7, 9, 8, 3,
            5, 9, 8, 7, 1, 2, 3, 6, 4,
            1, 3, 2, 8, 4, 6, 5, 7, 9,
            4, 6, 7, 5, 9, 3, 8, 2, 1,
        ]);
        assert_eq!(solve_standard(board), Some(solution))
    }
}
