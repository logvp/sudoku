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
use std::fmt::{Debug, Display};

use log::{debug, error, info, trace, warn};

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
    pub fn count(&self) -> usize {
        let mut count = 0;
        for digit in Digit::DIGITS {
            if self.get(digit) {
                count += 1;
            }
        }
        count
    }
    pub fn first(&self) -> Option<Digit> {
        for digit in Digit::DIGITS {
            if self.get(digit) {
                return Some(digit);
            }
        }
        None
    }
}
impl Debug for DigitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DigitSet {{")?;
        let mut first = true;
        // TODO: Can't iterate over DigitSet
        for digit in Digit::DIGITS {
            if self.get(digit) {
                if !first {
                    write!(f, ",")?;
                }
                write!(f, " {}", digit)?;
                first = false;
            }
        }
        write!(f, " }}")
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

    pub fn has_gaps(&self) -> bool {
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

fn make_move_from_solution(input: &Board, solution: &Board) -> Action {
    let mut placed_digits = Vec::new();
    for blank_idx in input
        .board
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.is_none().then_some(i))
    {
        let (x, y) = input.xy(blank_idx);
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

#[derive(Default)]
pub struct ConstraintSolver {
    state: (),
}
impl ConstraintSolver {
    fn solve(mut board: Board, rules: &Arbiter) -> Option<Board> {
        let printing = false;
        fn print_options(all_options: &[DigitSet; Board::WIDTH * Board::HEIGHT]) {
            let mut idx = 0;
            println!();
            for _ in 0..Board::HEIGHT {
                for _ in 0..Board::WIDTH {
                    print!("{:?} ", all_options[idx]);
                    idx += 1;
                }
                println!();
            }
        }

        if !rules.check(&board) {
            warn!("Presented board is invalid");
            return None;
        }

        let mut all_options = board.board.map(|digit| match digit {
            Some(_) => DigitSet::new(),
            None => {
                let mut set = DigitSet::new();
                set.invert();
                set
            }
        });

        println!("Initial:");
        board.print();

        let mut did_work = true;
        while did_work {
            did_work = false;
            let unset_cells: Vec<_> = board
                .board
                .iter()
                .enumerate()
                .filter_map(|(i, cell)| cell.is_none().then_some(i))
                .collect();

            for idx in unset_cells.iter().copied() {
                let options = &mut all_options[idx];
                match options.count() {
                    0 => unreachable!(),
                    1 => {
                        let digit = options
                            .first()
                            .expect("count == 1 so set must be non-empty");
                        board.board[idx] = Some(digit);
                        if !rules.check_one(&board, idx) {
                            // Last option left does not fit
                            return None;
                        }
                        options.clear(digit);
                        did_work = true;
                        if printing {
                            println!("Set {} at {}:", digit, idx);
                            board.print();
                        }
                    }
                    2.. => {
                        assert!(board.board[idx].is_none());

                        // TODO: DigitSet does not have an iterator
                        for digit in Digit::DIGITS {
                            if options.get(digit) {
                                board.board[idx] = Some(digit);
                                if !rules.check_one(&board, idx) {
                                    options.clear(digit);
                                    did_work = true;
                                }
                            }
                        }
                        board.board[idx] = None;
                        // TODO: benchmark best place for this check
                        if options.count() == 0 {
                            let (x, y) = board.xy(idx);
                            warn!("No possible valid digits for ({}, {})", x, y);
                            return None;
                        }
                    }
                }
            }
            assert!(rules.check(&board));
            if printing {
                print_options(&all_options);
            }

            if !did_work && !rules.is_solved(&board) {
                debug!("Guessing and checking");

                for idx in unset_cells {
                    let options = &mut all_options[idx];
                    assert!(board.board[idx].is_none());
                    assert!(options.count() > 1);
                    // TODO: DigitSet does not have an iterator
                    for digit in Digit::DIGITS {
                        if options.get(digit) {
                            board.board[idx] = Some(digit);
                            if ConstraintSolver::solve(board.clone(), rules).is_none() {
                                options.clear(digit);
                                did_work = true;
                            }
                        }
                    }
                    board.board[idx] = None;
                    // TODO: benchmark best place for this check
                    if options.count() == 0 {
                        let (x, y) = board.xy(idx);
                        warn!("No possible valid digits for ({}, {})", x, y);
                        return None;
                    }
                    if did_work {
                        break;
                    }
                }
            }
        }

        println!("Final:");
        board.print();

        if rules.is_solved(&board) {
            Some(board)
        } else {
            print_options(&all_options);
            warn!("Could not solve the board. Pretty sure it is ambiguous");
            None
        }
    }

    fn verify_board(&self, board: Board, rules: &Arbiter) -> BoardStatus {
        match Self::solve(board, rules) {
            Some(_) => BoardStatus::OneSolution,
            None => BoardStatus::Unsolvable, // TODO: catchall
        }
    }
}
impl Solver for ConstraintSolver {
    fn make_move(&mut self, board: &Board, rules: &Arbiter) -> Action {
        if let Some(solution) = Self::solve(board.clone(), rules) {
            make_move_from_solution(board, &solution)
        } else {
            Action::Abort
        }
    }
}

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

pub type Rules = Vec<Box<dyn SudokuRule>>;
pub struct Arbiter {
    rules: Rules,
}
impl Arbiter {
    pub fn new(rules: Rules) -> Self {
        Self { rules }
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
        for rule in &self.rules {
            if !rule.check(board) {
                return false;
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool {
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

pub enum SolveError {
    Aborted,
    Illegal,
}

pub fn solve_with(
    mut board: Board,
    rules: Rules,
    solver: &mut dyn Solver,
) -> Result<Board, SolveError> {
    let rules = Arbiter::new(rules);

    while !rules.is_solved(&board) {
        debug!("Making a move");
        let action = solver.make_move(&board, &rules);

        let status = rules.update(&mut board, action);
        match status {
            UpdateResult::Done => {
                assert!(rules.is_solved(&board));
                break;
            }
            UpdateResult::Ok => {
                assert!(rules.check(&board));
            }
            UpdateResult::Aborted => {
                return Err(SolveError::Aborted);
            }
            UpdateResult::IllegalMove => return Err(SolveError::Illegal),
        }
    }
    Ok(board)
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
    let arbiter = Arbiter::new(rules);

    solver.verify_board(board, &arbiter)
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
