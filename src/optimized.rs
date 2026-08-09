use log::{debug, error};

use crate::{Board, Digit, SudokuBox, SudokuColumn, SudokuRow, SudokuRule};

fn check_board_one(board: &Board, index: usize) -> bool {
    SudokuRow {}.check_one(board, index)
        && SudokuColumn {}.check_one(board, index)
        && SudokuBox {}.check_one(board, index)
}

fn check_board_all(board: &Board) -> bool {
    SudokuRow {}.check(board) && SudokuColumn {}.check(board) && SudokuBox {}.check(board)
}

pub fn solve_standard(mut board: Board) -> Option<Board> {
    if !check_board_all(&board) {
        error!("Board is unsolvable");
        return None;
    }

    let mut stack: Vec<usize> = Vec::new();
    if let Some(check_idx) = board.first_open_index() {
        stack.push(check_idx);
        board.board[check_idx] = Some(Digit::_1);
    } else {
        debug!("Board is already solved!");
        return Some(board);
    }

    let mut must_backtrack = false;
    while !stack.is_empty() {
        // if the guess was valid, continue on to the next open spot
        let check_idx = stack
            .last()
            .copied()
            .expect("unreachable because stack is not empty");
        if !must_backtrack && check_board_one(&board, check_idx) {
            let Some(next_open) = board.next_open_index(check_idx) else {
                return Some(board);
            };
            stack.push(next_open);
            board.board[next_open] = Some(Digit::_1);
        } else {
            must_backtrack = false;
            // if the guess was invalid, increment the guess
            if let Some(next_digit) = board.board[check_idx].unwrap().next() {
                board.board[check_idx] = Some(next_digit);
            }
            // if we exhausted all guesses for this index, rewind guess and backtrack
            else {
                board.board[check_idx] = None;
                stack.pop().expect("unreachable because stack is not empty");
                must_backtrack = true;
            }
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
        println!("input:");
        board.print();
        let answer = solve_standard(board).unwrap();
        answer.print();
        assert_eq!(answer, solution)
    }
}
