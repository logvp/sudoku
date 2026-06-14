use std::num::ParseIntError;

type Digit = u32;
const MAX_DIGIT: Digit = 9;

#[derive(Clone)]
struct Board {
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

    fn make(board: [Digit; Self::HEIGHT * Self::WIDTH]) -> Self {
        Self {
            board: board.map(|d| (d != 0).then_some(d)),
        }
    }

    fn len(&self) -> usize {
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

    fn index(&self, x: usize, y: usize) -> usize {
        y * Self::WIDTH + x
    }

    fn xy(&self, index: usize) -> (usize, usize) {
        (index % Self::WIDTH, index / Self::WIDTH)
    }

    fn get(&self, x: usize, y: usize) -> Option<Digit> {
        self.board[self.index(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, digit: Digit) {
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

trait SudokuRule {
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

struct SudokuRow;
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

struct SudokuColumn;
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

struct SudokuBox;
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

enum Action {
    Set { digit: Digit, x: usize, y: usize },
    AlreadySolved,
    Abort,
}

trait Solver {
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
struct BacktrackingSolver {
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
            return Some(board);
        };
        for digit in 1..=MAX_DIGIT {
            let mut solution = board.clone();
            solution.board[check_idx] = Some(digit);
            if !rules.check_board(&solution) {
                continue;
            }
            if let Some(solved) = self.solve_impl(solution, rules) {
                return Some(solved);
            }
        }
        return None;
    }
}
impl Solver for BacktrackingSolver {
    fn make_move(&mut self, state: &GameState) -> Action {
        if self.solution.is_none() {
            if !self.solve(state) {
                println!("Board is unsolvable!");
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

struct GameState {
    board: Board,
    rules: Vec<Box<dyn SudokuRule>>,
}
impl GameState {
    fn update(&mut self, action: Action) -> Result<(), ()> {
        match action {
            Action::Set { digit, x, y } => {
                if self.board.get(x, y).is_some() {
                    return Err(());
                }
                let mut new_board = self.board.clone();
                new_board.set(x, y, digit);
                if self.check_board(&new_board) {
                    self.board = new_board;
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            Action::Abort => {
                println!("Solver aborted!");
                return Err(());
            }
            Action::AlreadySolved => {
                println!("Solver reported already solved");
                return Err(());
            }
        }
    }

    fn check(&self) -> bool {
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

    fn solved(&self) -> bool {
        self.check() && !self.board.has_gaps()
    }
}

fn main() {
    let board = Board::default();
    let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
    rules.push(Box::new(SudokuRow));
    rules.push(Box::new(SudokuColumn));
    rules.push(Box::new(SudokuBox));
    let mut game = GameState { board, rules };

    let mut solver = BacktrackingSolver::default();

    loop {
        let action = solver.make_move(&game);

        let status = game.update(action);
        if status.is_err() {
            println!("Invalid move!");
            continue;
        }

        assert!(game.check());

        if game.solved() {
            println!("Solved!");
            game.board.print();
            break;
        }
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
}
