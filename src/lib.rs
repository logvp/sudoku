#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::suspicious,
    unused_must_use,
    unfulfilled_lint_expectations
)]
#![warn(clippy::complexity, clippy::perf, clippy::style)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::wildcard_imports,
    clippy::semicolon_if_nothing_returned,
    clippy::uninlined_format_args,
    clippy::missing_errors_doc,
    clippy::match_same_arms,
    clippy::needless_continue,
    clippy::ignored_unit_patterns,
    clippy::must_use_candidate
)]

mod board;
mod rules;
mod solver;
use std::{cell::Cell, fmt::Debug};

use log::{debug, error, info, trace, warn};

pub use board::*;
pub use rules::*;
pub use solver::*;

struct Counter {
    name: &'static str,
    data: Cell<usize>,
}
impl Counter {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            data: Cell::new(0),
        }
    }
    fn inc(&self) {
        self.data.update(|n| n + 1);
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        println!("Counter {}: {}", self.name, self.data.get())
    }
}

pub trait SudokuRule {
    fn check(&self, board: &Board) -> bool {
        for i in 0..board.len() {
            if !self.check_one(board, i) {
                return false;
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool;
}

pub struct DigitPos {
    digit: Digit,
    x: usize,
    y: usize,
}
pub enum Action {
    Set(Vec<DigitPos>),
    AlreadySolved,
    Abort,
}

pub enum UpdateResult {
    Ok,
    IllegalMove,
    Done,
    Aborted,
}
impl UpdateResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, UpdateResult::Ok | UpdateResult::Done)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardStatus {
    Unsolvable,
    AlreadySolved,
    OneSolution,
    MultipleSolutions,
}

pub trait Solver {
    fn make_move(&mut self, board: &Board, rules: &Arbiter) -> Action;
}

#[derive(Default)]
pub struct HumanSolver {}
impl Solver for HumanSolver {
    fn make_move(&mut self, board: &Board, _rules: &Arbiter) -> Action {
        let mut buf = String::new();
        loop {
            println!("Board:");
            board.print();

            println!("Enter your move");
            println!("x y digit");
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();

            let nums: Result<Vec<usize>, _> =
                buf.split_whitespace().map(str::parse::<usize>).collect();
            let Ok(nums) = nums else {
                println!("Could not parse input");
                continue;
            };
            if nums.len() != 3 {
                println!("Invalid number of entries");
                continue;
            }
            let x = nums[0];
            let y = nums[1];
            let digit = match u32::try_from(nums[2]) {
                Ok(ok) => ok,
                Err(e) => {
                    println!("Could not parse digit: {}", e);
                    continue;
                }
            };
            let Ok(digit) = Digit::try_from(digit) else {
                println!("Could not parse digit");
                continue;
            };

            break Action::Set(vec![DigitPos { digit, x, y }]);
        }
    }
}

pub type Rules = Vec<Box<dyn SudokuRule>>;
pub struct Arbiter {
    check_one_counter: Counter,
    check_counter: Counter,
    rules: Rules,
}
impl Arbiter {
    pub fn new(rules: Rules) -> Self {
        Self {
            rules,
            check_counter: Counter::new("check_full"),
            check_one_counter: Counter::new("check_one"),
        }
    }

    pub fn step(&self, board: &mut Board, solver: &mut dyn Solver) -> UpdateResult {
        let action = solver.make_move(board, self);
        self.update(board, action)
    }

    fn update(&self, board: &mut Board, action: Action) -> UpdateResult {
        match action {
            Action::Set(digit_list) => {
                for DigitPos { digit, x, y } in digit_list {
                    if board.get(x, y).is_some() {
                        error!("Attempted to set already set spot at ({},{})", x, y);
                        return UpdateResult::IllegalMove;
                    }
                    let mut new_board = board.clone();
                    new_board.set(x, y, digit);
                    if self.check_one(&new_board, board.index(x, y)) {
                        *board = new_board;
                        trace!("Set digit: {} at ({},{})", digit, x, y);
                    } else {
                        error!("Illegal digit: {} at ({},{})", digit, x, y);
                        return UpdateResult::IllegalMove;
                    }
                }
                UpdateResult::Ok
            }
            Action::Abort => {
                info!("Solver aborted!");
                UpdateResult::Aborted
            }
            Action::AlreadySolved => {
                if !self.is_solved(board) {
                    error!("Solver reported solved but its not!");
                    return UpdateResult::IllegalMove;
                }
                info!("Solver reported already solved");
                UpdateResult::Done
            }
        }
    }

    pub fn check(&self, board: &Board) -> bool {
        self.check_counter.inc();
        for rule in &self.rules {
            if !rule.check(board) {
                return false;
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool {
        self.check_one_counter.inc();
        for rule in &self.rules {
            if !rule.check_one(board, index) {
                return false;
            }
        }
        true
    }

    pub fn is_solved(&self, board: &Board) -> bool {
        !board.has_gaps() && self.check(board)
    }
}

pub fn standard_sudoku_rules() -> Rules {
    let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
    rules.push(Box::new(SudokuRow));
    rules.push(Box::new(SudokuColumn));
    rules.push(Box::new(SudokuBox));
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solver_harness<T: Solver>(mut solver: T) {
        #[rustfmt::skip]
        let mut board: Board = Board::make([
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
        let rules = Arbiter::new(standard_sudoku_rules());
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
        loop {
            let action = solver.make_move(&board, &rules);
            assert!(rules.update(&mut board, action).is_ok());
            assert!(rules.check(&board));
            if rules.is_solved(&board) {
                break;
            }
        }
        assert_eq!(board, solution);
    }

    #[test]
    fn test_backtracking_solver() {
        let solver = BacktrackingSolver::default();
        test_solver_harness(solver);
    }

    #[test]
    fn test_verify_board() {
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
        assert_eq!(verify(board, None), BoardStatus::OneSolution);

        #[rustfmt::skip]
        let board: Board = Board::make([
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
        assert_eq!(verify(board, None), BoardStatus::AlreadySolved);

        // #[rustfmt::skip]
        // let board: Board = Board::make([
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0, 0, 0, 0, 0, 0, 0,
        // ]);
        // assert_eq!(verify(board, None), BoardStatus::MultipleSolutions);

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]);
        assert_eq!(verify(board, None), BoardStatus::Unsolvable);
    }
}
