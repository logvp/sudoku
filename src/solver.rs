mod backtracking;
mod constraint;
mod human;
mod util;

pub use backtracking::*;
pub use constraint::*;
pub use human::*;
pub use util::*;

use crate::{Arbiter, Board, Digit};

pub trait Solver {
    fn make_move(&mut self, board: &Board, rules: &Arbiter) -> Action;
}

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
