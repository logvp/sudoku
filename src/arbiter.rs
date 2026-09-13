use log::{error, info, trace};

use crate::{Action, Board, Counter, DigitPos, Solver, SudokuRule};

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

    pub fn update(&self, board: &mut Board, action: Action) -> UpdateResult {
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

    pub fn check_one(&self, board: &Board, index: usize) -> bool {
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
