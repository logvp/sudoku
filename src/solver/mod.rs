mod backtracking;
mod constraint;
mod human;
mod util;

pub use backtracking::*;
pub use constraint::*;
pub use human::*;
pub use util::*;

use crate::Digit;

pub struct DigitPos {
    pub digit: Digit,
    pub x: usize,
    pub y: usize,
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
