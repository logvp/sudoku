use log::{debug, error};

use crate::{Board, BoardStatus, Digit, SudokuBox, SudokuColumn, SudokuRow, SudokuRule};

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

    let num_gaps = board.board.iter().filter(|x| x.is_none()).count();
    if num_gaps == 0 {
        debug!("Board is already solved!");
        return Some(board);
    }
    let mut stack: Vec<usize> = Vec::new();
    stack.reserve_exact(num_gaps);
    if let Some(check_idx) = board.first_open_index() {
        stack.push(check_idx);
        board.board[check_idx] = Some(Digit::_1);
    } else {
        unreachable!()
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

pub fn verify_standard(mut board: Board) -> BoardStatus {
    if !check_board_all(&board) {
        error!("Board is unsolvable");
        return BoardStatus::Unsolvable;
    }

    let num_gaps = board.board.iter().filter(|x| x.is_none()).count();
    if num_gaps == 0 {
        debug!("Board is already solved!");
        return BoardStatus::AlreadySolved;
    }
    let mut stack: Vec<usize> = Vec::new();
    stack.reserve_exact(num_gaps);
    if let Some(check_idx) = board.first_open_index() {
        stack.push(check_idx);
        board.board[check_idx] = Some(Digit::_1);
    } else {
        unreachable!()
    }

    let mut num_solutions = 0;
    let mut must_backtrack = false;
    while !stack.is_empty() {
        // if the guess was valid, continue on to the next open spot
        let check_idx = stack
            .last()
            .copied()
            .expect("unreachable because stack is not empty");
        if !must_backtrack && check_board_one(&board, check_idx) {
            let Some(next_open) = board.next_open_index(check_idx) else {
                num_solutions += 1;
                if num_solutions > 1 {
                    return BoardStatus::MultipleSolutions;
                } else {
                    must_backtrack = true;
                    continue;
                }
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

    if num_solutions == 0 {
        BoardStatus::Unsolvable
    } else if num_solutions == 1 {
        BoardStatus::OneSolution
    } else {
        unreachable!()
    }
}

pub fn is_minimal_standard(board: Board) -> bool {
    match verify_standard(board.clone()) {
        BoardStatus::AlreadySolved | BoardStatus::OneSolution => (), // ok
        BoardStatus::MultipleSolutions => {
            error!("Starting board is already ambiguous");
            return false;
        }
        BoardStatus::Unsolvable => {
            error!("Starting board is not solvable");
            return false;
        }
    }

    let set_cells = board
        .board
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| cell.is_some().then_some(i));

    for index in set_cells {
        let mut this_board = board.clone();
        this_board.board[index] = None;
        if verify_standard(this_board) == BoardStatus::OneSolution {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_standard() {
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

    #[test]
    fn test_is_minimal_standard() {
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
        assert!(!is_minimal_standard(board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            9, 6, 0, 5, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 4, 0, 0, 0, 7, 0, 0,
            7, 0, 0, 0, 0, 1, 0, 5, 0,
            0, 0, 5, 0, 7, 0, 2, 0, 0,
            0, 2, 0, 0, 0, 9, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 7,
            0, 0, 2, 6, 1, 0, 8, 0, 0,
            0, 4, 0, 0, 0, 3, 0, 0, 6,
        ]);
        assert!(is_minimal_standard(board))
    }
}
