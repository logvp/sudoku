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

pub mod optimized;
mod rules;
use std::fmt::Display;

use log::{debug, error, info, trace};

pub use rules::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    _1 = 1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}
impl TryFrom<u32> for Digit {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Digit::_1),
            2 => Ok(Digit::_2),
            3 => Ok(Digit::_3),
            4 => Ok(Digit::_4),
            5 => Ok(Digit::_5),
            6 => Ok(Digit::_6),
            7 => Ok(Digit::_7),
            8 => Ok(Digit::_8),
            9 => Ok(Digit::_9),
            _ => Err(()),
        }
    }
}
impl Into<u32> for Digit {
    fn into(self) -> u32 {
        match self {
            Digit::_1 => 1,
            Digit::_2 => 2,
            Digit::_3 => 3,
            Digit::_4 => 4,
            Digit::_5 => 5,
            Digit::_6 => 6,
            Digit::_7 => 7,
            Digit::_8 => 8,
            Digit::_9 => 9,
        }
    }
}
impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Digit::_1 => "1",
            Digit::_2 => "2",
            Digit::_3 => "3",
            Digit::_4 => "4",
            Digit::_5 => "5",
            Digit::_6 => "6",
            Digit::_7 => "7",
            Digit::_8 => "8",
            Digit::_9 => "9",
        })
    }
}
impl Digit {
    pub const DIGITS: [Digit; 9] = [
        Digit::_1,
        Digit::_2,
        Digit::_3,
        Digit::_4,
        Digit::_5,
        Digit::_6,
        Digit::_7,
        Digit::_8,
        Digit::_9,
    ];

    pub const MAX_DIGIT: Digit = Digit::_9;
    pub const ARRAY_SIZE: usize = Digit::MAX_DIGIT as usize + 1;

    pub fn shift(self) -> Digit {
        match self {
            Digit::_1 => Digit::_2,
            Digit::_2 => Digit::_3,
            Digit::_3 => Digit::_4,
            Digit::_4 => Digit::_5,
            Digit::_5 => Digit::_6,
            Digit::_6 => Digit::_7,
            Digit::_7 => Digit::_8,
            Digit::_8 => Digit::_9,
            Digit::_9 => Digit::_1,
        }
    }

    pub fn next(self) -> Option<Digit> {
        match self {
            Digit::_1 => Some(Digit::_2),
            Digit::_2 => Some(Digit::_3),
            Digit::_3 => Some(Digit::_4),
            Digit::_4 => Some(Digit::_5),
            Digit::_5 => Some(Digit::_6),
            Digit::_6 => Some(Digit::_7),
            Digit::_7 => Some(Digit::_8),
            Digit::_8 => Some(Digit::_9),
            Digit::_9 => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DigitSet {
    storage: [bool; Digit::ARRAY_SIZE],
}
impl DigitSet {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }
    #[inline]
    pub fn set(&mut self, digit: Digit) {
        self.storage[digit as usize] = true;
    }
    #[inline]
    pub fn clear(&mut self, digit: Digit) {
        self.storage[digit as usize] = false;
    }
    #[inline]
    pub fn get(&self, digit: Digit) -> bool {
        self.storage[digit as usize]
    }
    pub fn invert(&mut self) {
        for x in self.storage.iter_mut() {
            *x = !*x;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    board: [Option<Digit>; Self::HEIGHT * Self::WIDTH],
}
impl Default for Board {
    fn default() -> Self {
        Self {
            board: [None; Self::HEIGHT * Self::WIDTH],
        }
    }
}
impl Board {
    const HEIGHT: usize = 9;
    const WIDTH: usize = 9;

    pub fn make(board: [u32; Self::HEIGHT * Self::WIDTH]) -> Self {
        Self {
            board: board.map(Digit::try_from).map(Result::ok),
        }
    }

    pub fn len(&self) -> usize {
        self.board.len()
    }

    fn height(&self) -> usize {
        Self::HEIGHT
    }

    fn width(&self) -> usize {
        Self::WIDTH
    }

    fn first_open_index(&self) -> Option<usize> {
        for i in 0..self.len() {
            if let Some(None) = self.board.get(i) {
                return Some(i);
            }
        }
        None
    }

    fn next_open_index(&self, after: usize) -> Option<usize> {
        for i in (after + 1)..self.len() {
            if let Some(None) = self.board.get(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(self.in_bounds(x, y));
        y * Self::WIDTH + x
    }

    pub fn xy(&self, index: usize) -> (usize, usize) {
        assert!(index < self.board.len());
        (index % Self::WIDTH, index / Self::WIDTH)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Digit> {
        self.board[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, digit: Digit) {
        self.board[self.index(x, y)] = Some(digit)
    }

    pub fn reset(&mut self, x: usize, y: usize) {
        self.board[self.index(x, y)] = None
    }

    fn has_gaps(&self) -> bool {
        self.board.iter().any(Option::is_none)
    }

    pub fn print(&self) {
        for j in 0..self.height() {
            for i in 0..self.width() {
                if let Some(digit) = self.get(i, j) {
                    print!("{} ", digit);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
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
    fn make_move(&mut self, state: &GameState) -> Action;
}

#[derive(Default)]
pub struct HumanSolver {}
impl Solver for HumanSolver {
    fn make_move(&mut self, state: &GameState) -> Action {
        let mut buf = String::new();
        loop {
            println!("Board:");
            state.board.print();

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

#[derive(Default)]
pub struct BacktrackingSolver {
    solution: Option<Board>,
}
impl BacktrackingSolver {
    fn solve(&mut self, mut board: Board, rules: &GameState) -> Option<Board> {
        if !rules.check_board(&board) {
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
            if !must_backtrack && rules.check_board_one(&board, check_idx) {
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

    pub fn verify_board(&self, state: &GameState) -> BoardStatus {
        if !state.check() {
            error!("Board is unsolvable");
            return BoardStatus::Unsolvable;
        }

        let mut board = state.board.clone();

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
            if !must_backtrack && state.check_board_one(&board, check_idx) {
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
    fn make_move(&mut self, state: &GameState) -> Action {
        if self.solution.is_none() {
            self.solution = self.solve(state.board.clone(), state);
            if self.solution.is_none() {
                error!("Board is unsolvable!");
                return Action::Abort;
            }
        }
        let solution = self.solution.as_ref().unwrap();
        let mut placed_digits = Vec::new();
        for blank_idx in state
            .board
            .board
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.is_none().then_some(i))
        {
            let (x, y) = state.board.xy(blank_idx);
            placed_digits.push(DigitPos {
                digit: solution.board[blank_idx].unwrap(),
                x,
                y,
            });
        }
        if placed_digits.is_empty() {
            Action::AlreadySolved
        } else {
            Action::Set(placed_digits)
        }
    }
}

pub type Rules = Vec<Box<dyn SudokuRule>>;
pub struct GameState {
    board: Board,
    rules: Rules,
}
impl GameState {
    pub fn new(board: Board, rules: Rules) -> Self {
        Self { board, rules }
    }

    pub fn update(&mut self, action: Action) -> UpdateResult {
        match action {
            Action::Set(digit_list) => {
                for DigitPos { digit, x, y } in digit_list {
                    if self.board.get(x, y).is_some() {
                        error!("Attempted to set already set spot at ({},{})", x, y);
                        return UpdateResult::IllegalMove;
                    }
                    let mut new_board = self.board.clone();
                    new_board.set(x, y, digit);
                    if self.check_board_one(&new_board, self.board.index(x, y)) {
                        self.board = new_board;
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
                if !self.solved() {
                    error!("Solver reported solved but its not!");
                    return UpdateResult::IllegalMove;
                }
                info!("Solver reported already solved");
                UpdateResult::Done
            }
        }
    }

    pub fn check(&self) -> bool {
        self.check_board(&self.board)
    }

    fn check_board(&self, board: &Board) -> bool {
        for rule in &self.rules {
            if !rule.check(board) {
                return false;
            }
        }
        true
    }

    fn check_board_one(&self, board: &Board, index: usize) -> bool {
        for rule in &self.rules {
            if !rule.check_one(board, index) {
                return false;
            }
        }
        true
    }

    pub fn solved(&self) -> bool {
        !self.board.has_gaps() && self.check()
    }

    pub fn print_board(&self) {
        self.board.print();
    }
}

pub fn standard_sudoku_rules() -> Rules {
    let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
    rules.push(Box::new(SudokuRow));
    rules.push(Box::new(SudokuColumn));
    rules.push(Box::new(SudokuBox));
    rules
}

pub enum SolveError {
    Aborted,
    Illegal,
}

pub fn solve_with(
    board: Board,
    rules: Rules,
    solver: &mut dyn Solver,
) -> Result<Board, SolveError> {
    let mut game = GameState::new(board, rules);

    while !game.solved() {
        debug!("Making a move");
        let action = solver.make_move(&game);

        let status = game.update(action);
        match status {
            UpdateResult::Done => {
                assert!(game.solved());
                break;
            }
            UpdateResult::Ok => {
                assert!(game.check());
            }
            UpdateResult::Aborted => {
                return Err(SolveError::Aborted);
            }
            UpdateResult::IllegalMove => return Err(SolveError::Illegal),
        }
    }
    Ok(game.board)
}

type DefaultSolver = BacktrackingSolver;
pub fn solve(board: Board, rules: Option<Rules>) -> Result<Board, SolveError> {
    let mut solver = DefaultSolver::default();
    let rules = rules.unwrap_or_else(standard_sudoku_rules);
    solve_with(board, rules, &mut solver)
}

pub fn verify(board: Board, rules: Option<Rules>) -> BoardStatus {
    let solver = DefaultSolver::default();
    let rules = rules.unwrap_or_else(standard_sudoku_rules);
    let game = GameState::new(board, rules);

    solver.verify_board(&game)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solver_harness<T: Solver>(mut solver: T) {
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
        let rules = standard_sudoku_rules();
        let mut game = GameState { board, rules };
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
            let action = solver.make_move(&game);
            assert!(game.update(action).is_ok());
            assert!(game.check());
            if game.solved() {
                break;
            }
        }
        assert_eq!(game.board, solution);
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

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert_eq!(verify(board, None), BoardStatus::MultipleSolutions);

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
