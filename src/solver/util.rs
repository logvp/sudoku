use log::debug;

use crate::{
    Action, Arbiter, Board, ConstraintSolver, DigitPos, Rules, Solver, UpdateResult,
    standard_sudoku_rules,
};

pub fn make_move_from_solution(input: &Board, solution: &Board) -> Action {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardStatus {
    Unsolvable,
    AlreadySolved,
    OneSolution,
    MultipleSolutions,
}

type DefaultSolver = ConstraintSolver;
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
