mod rules;
use std::fmt::Display;

use log::{debug, error, info, trace};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Digit {
    _1 = 0,
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
}

#[derive(Clone, PartialEq, Debug)]
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

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * Self::WIDTH + x
    }

    pub fn xy(&self, index: usize) -> (usize, usize) {
        (index % Self::WIDTH, index / Self::WIDTH)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Digit> {
        self.board[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, digit: Digit) {
        self.board[self.index(x, y)] = Some(digit)
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

pub enum Action {
    Set { digit: Digit, x: usize, y: usize },
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

#[derive(Debug, PartialEq)]
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
            let digit = match Digit::try_from(digit) {
                Ok(ok) => ok,
                Err(_) => {
                    println!("Could not parse digit");
                    continue;
                }
            };

            break Action::Set { digit, x, y };
        }
    }
}

#[derive(Default)]
pub struct BacktrackingSolver {
    solution: Option<Board>,
}
impl BacktrackingSolver {
    fn solve(&mut self, state: &GameState) -> bool {
        let solution = state.board.clone();
        self.solution = self.solve_impl(solution, state);
        self.solution.is_some()
    }

    fn solve_impl(&mut self, board: Board, rules: &GameState) -> Option<Board> {
        let Some(check_idx) = board.first_open_index() else {
            debug!("Board is already solved!");
            return Some(board);
        };
        for digit in Digit::DIGITS {
            let mut solution = board.clone();
            solution.board[check_idx] = Some(digit);
            if !rules.check_board_one(&solution, check_idx) {
                continue;
            }
            let (x, y) = solution.xy(check_idx);
            trace!("Attempting {} at ({},{})", digit, x, y);
            if let Some(solved) = self.solve_impl(solution, rules) {
                return Some(solved);
            }
        }
        None
    }

    pub fn verify_board(&self, state: &GameState) -> BoardStatus {
        if state.solved() {
            return BoardStatus::AlreadySolved;
        }
        let solution = state.board.clone();
        match self.count_solutions_impl(solution, state) {
            0 => BoardStatus::Unsolvable,
            1 => BoardStatus::OneSolution,
            _ => BoardStatus::MultipleSolutions,
        }
    }

    fn count_solutions_impl(&self, board: Board, rules: &GameState) -> usize {
        let Some(check_idx) = board.first_open_index() else {
            return 1;
        };
        let mut count = 0;
        for digit in Digit::DIGITS {
            let mut solution = board.clone();
            solution.board[check_idx] = Some(digit);
            if !rules.check_board_one(&solution, check_idx) {
                continue;
            }
            count += self.count_solutions_impl(solution, rules);
            if count > 1 {
                return count;
            }
        }
        count
    }
}
impl Solver for BacktrackingSolver {
    fn make_move(&mut self, state: &GameState) -> Action {
        if self.solution.is_none() && !self.solve(state) {
            error!("Board is unsolvable!");
            return Action::Abort;
        }
        let solution = self.solution.as_ref().unwrap();
        let Some(index) = state.board.first_open_index() else {
            return Action::AlreadySolved;
        };
        let (x, y) = state.board.xy(index);
        Action::Set {
            digit: solution.board[index].unwrap(),
            x,
            y,
        }
    }
}

type Rules = Vec<Box<dyn SudokuRule>>;
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
            Action::Set { digit, x, y } => {
                if self.board.get(x, y).is_some() {
                    error!("Attempted to set already set spot at ({},{})", x, y);
                    return UpdateResult::IllegalMove;
                }
                let mut new_board = self.board.clone();
                new_board.set(x, y, digit);
                if self.check_board_one(&new_board, self.board.index(x, y)) {
                    self.board = new_board;
                    trace!("Set digit: {} at ({},{})", digit, x, y);
                    UpdateResult::Ok
                } else {
                    error!("Illegal digit: {} at ({},{})", digit, x, y);
                    UpdateResult::IllegalMove
                }
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
    rules.push(Box::new(rules::SudokuRow));
    rules.push(Box::new(rules::SudokuColumn));
    rules.push(Box::new(rules::SudokuBox));
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

pub fn solve(board: Board) -> Result<Board, SolveError> {
    let mut solver = BacktrackingSolver::default();
    let rules = standard_sudoku_rules();
    solve_with(board, rules, &mut solver)
}

pub fn verify(board: Board) -> BoardStatus {
    let solver = BacktrackingSolver::default();
    let rules = standard_sudoku_rules();
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
        let rules = standard_sudoku_rules();
        let game = GameState { board, rules };
        let solver = BacktrackingSolver::default();
        assert_eq!(solver.verify_board(&game), BoardStatus::OneSolution);
    }
}
