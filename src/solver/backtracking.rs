use log::{debug, error};

use crate::{Action, Arbiter, Board, BoardStatus, Digit, Solver, make_move_from_solution};

#[derive(Default)]
pub struct BacktrackingSolver {
    solution: Option<Board>,
}
impl BacktrackingSolver {
    fn solve(&mut self, mut board: Board, rules: &Arbiter) -> Option<Board> {
        if !rules.check(&board) {
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
            if !must_backtrack && rules.check_one(&board, check_idx) {
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

    pub fn verify_board(&self, mut board: Board, rules: &Arbiter) -> BoardStatus {
        if !rules.check(&board) {
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
            if !must_backtrack && rules.check_one(&board, check_idx) {
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
}
impl Solver for BacktrackingSolver {
    fn make_move(&mut self, board: &Board, rules: &Arbiter) -> Action {
        if self.solution.is_none() {
            self.solution = self.solve(board.clone(), rules);
            if self.solution.is_none() {
                error!("Board is unsolvable!");
                return Action::Abort;
            }
        }
        let solution = self.solution.as_ref().unwrap();
        make_move_from_solution(board, solution)
    }
}
