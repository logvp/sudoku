use log::{debug, error, info, trace, warn};
use std::num::ParseIntError;

type Digit = u32;
const MAX_DIGIT: Digit = 9;

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

    pub fn make(board: [Digit; Self::HEIGHT * Self::WIDTH]) -> Self {
        Self {
            board: board.map(|d| (d != 0).then_some(d)),
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

    fn print(&self) {
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

pub struct SudokuRow;
impl SudokuRule for SudokuRow {
    fn check_one(&self, board: &Board, index: usize) -> bool {
        let (_, y) = board.xy(index);
        let mut set = 0u64;
        for i in 0..board.width() {
            let mask = if let Some(digit) = board.get(i, y) {
                1 << digit
            } else {
                0
            };
            if (set & mask) != 0 {
                return false;
            }
            set |= mask;
        }
        true
    }
}

pub struct SudokuColumn;
impl SudokuRule for SudokuColumn {
    fn check_one(&self, board: &Board, index: usize) -> bool {
        let (x, _) = board.xy(index);
        let mut set = 0u64;
        for j in 0..board.height() {
            let mask = if let Some(digit) = board.get(x, j) {
                1 << digit
            } else {
                0
            };
            if (set & mask) != 0 {
                return false;
            }
            set |= mask;
        }
        true
    }
}

pub struct SudokuBox;
impl SudokuRule for SudokuBox {
    fn check_one(&self, board: &Board, index: usize) -> bool {
        let (x, y) = board.xy(index);
        let mut set = 0u64;
        let box_start_x = x - (x % 3);
        let box_start_y = y - (y % 3);
        for i in box_start_x..(box_start_x + 3) {
            for j in box_start_y..(box_start_y + 3) {
                let mask = if let Some(digit) = board.get(i, j) {
                    1 << digit
                } else {
                    0
                };
                if (set & mask) != 0 {
                    return false;
                }
                set |= mask;
            }
        }
        true
    }
}

pub enum Action {
    Set { digit: Digit, x: usize, y: usize },
    AlreadySolved,
    Abort,
}

pub trait Solver {
    fn make_move(&mut self, state: &GameState) -> Action;
}

struct HumanSolver;
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

            let nums: Result<Vec<usize>, ParseIntError> =
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
            let digit = Digit::try_from(nums[2]).unwrap();

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
        for digit in 1..=MAX_DIGIT {
            let mut solution = board.clone();
            solution.board[check_idx] = Some(digit);
            if !rules.check_board(&solution) {
                continue;
            }
            let (x, y) = solution.xy(check_idx);
            trace!("Attempting {} at ({},{})", digit, x, y);
            if let Some(solved) = self.solve_impl(solution, rules) {
                return Some(solved);
            }
        }
        return None;
    }

    pub fn count_solutions(&self, state: &GameState) -> usize {
        let solution = state.board.clone();
        self.count_solutions_impl(solution, state)
    }

    fn count_solutions_impl(&self, board: Board, rules: &GameState) -> usize {
        let Some(check_idx) = board.first_open_index() else {
            return 1;
        };
        let mut count = 0;
        for digit in 1..=MAX_DIGIT {
            let mut solution = board.clone();
            solution.board[check_idx] = Some(digit);
            if !rules.check_board(&solution) {
                continue;
            }
            count += self.count_solutions_impl(solution, rules);
        }
        return count;
    }
}
impl Solver for BacktrackingSolver {
    fn make_move(&mut self, state: &GameState) -> Action {
        if self.solution.is_none() {
            if !self.solve(state) {
                error!("Board is unsolvable!");
                return Action::Abort;
            }
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

pub struct GameState {
    board: Board,
    rules: Vec<Box<dyn SudokuRule>>,
}
impl GameState {
    pub fn new(board: Board, rules: Vec<Box<dyn SudokuRule>>) -> Self {
        Self { board, rules }
    }

    pub fn update(&mut self, action: Action) -> Result<(), ()> {
        match action {
            Action::Set { digit, x, y } => {
                if self.board.get(x, y).is_some() {
                    error!("Attempted to set already set spot at ({},{})", x, y);
                    return Err(());
                }
                let mut new_board = self.board.clone();
                new_board.set(x, y, digit);
                if self.check_board(&new_board) {
                    self.board = new_board;
                    trace!("Set digit: {} at ({},{})", digit, x, y);
                    return Ok(());
                } else {
                    error!("Illegal digit: {} at ({},{})", digit, x, y);
                    return Err(());
                }
            }
            Action::Abort => {
                info!("Solver aborted!");
                return Err(());
            }
            Action::AlreadySolved => {
                info!("Solver reported already solved");
                return Err(());
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

    pub fn solved(&self) -> bool {
        self.check() && !self.board.has_gaps()
    }

    pub fn print_board(&self) {
        self.board.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_rule() {
        let row_rule = SudokuRow;

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
        assert!(row_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ]);
        assert!(row_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!row_rule.check(&board));
    }

    #[test]
    fn test_column_rule() {
        let column_rule = SudokuColumn;

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
        assert!(column_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
        ]);
        assert!(column_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!column_rule.check(&board));
    }

    #[test]
    fn test_box_rule() {
        let box_rule = SudokuBox;

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
        assert!(box_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
        ]);
        assert!(box_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!box_rule.check(&board));
    }

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
        let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
        // Standard sudoku rules
        rules.push(Box::new(SudokuRow));
        rules.push(Box::new(SudokuColumn));
        rules.push(Box::new(SudokuBox));
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
    fn test_count_solutions() {
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
        let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
        // Standard sudoku rules
        rules.push(Box::new(SudokuRow));
        rules.push(Box::new(SudokuColumn));
        rules.push(Box::new(SudokuBox));
        let game = GameState { board, rules };
        let solver = BacktrackingSolver::default();
        assert_eq!(solver.count_solutions(&game), 1);
    }
}
