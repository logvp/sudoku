use log::{error, info, warn};

use crate::{
    Action, Arbiter, Board, BoardStatus, Digit, DigitSet, Solver, Verifier, make_move_from_solution,
};

#[derive(Debug, PartialEq, Clone)]
struct PossibleDigits {
    digits: [DigitSet; Board::WIDTH * Board::HEIGHT],
}
impl PossibleDigits {
    fn new_empty() -> Self {
        Self {
            digits: std::array::repeat(DigitSet::new()),
        }
    }

    fn from(board: &Board) -> Self {
        let digits = board.board.map(|digit| {
            if let Some(digit) = digit {
                let mut set = DigitSet::new();
                set.set(digit);
                set
            } else {
                let mut set = DigitSet::new();
                set.invert();
                set
            }
        });
        Self { digits }
    }

    fn union(&mut self, other: &Self) {
        for idx in 0..self.digits.len() {
            self.digits[idx].union(&other.digits[idx])
        }
    }

    fn get_index(&self, index: usize) -> &DigitSet {
        &self.digits[index]
    }

    fn get_index_mut(&mut self, index: usize) -> &mut DigitSet {
        &mut self.digits[index]
    }

    fn print(&self) {
        let mut idx = 0;
        for _ in 0..Board::HEIGHT {
            for _ in 0..Board::WIDTH {
                print!("{:?} ", self.digits[idx]);
                idx += 1;
            }
            println!();
        }
    }
}

enum PartialConstraintResult {
    Complete(ConstraintResult),
    Incomplete {
        board: Board,
        all_options: PossibleDigits,
    },
}

#[derive(Debug)]
enum ConstraintResult {
    DepthLimit(PossibleDigits),
    Contradiction,
    Solvable(Board),
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SolveType {
    FindFirstSolution,
    ValidateOneSolution,
}

#[derive(Default)]
pub struct ConstraintSolver {
    _state: (),
}
impl ConstraintSolver {
    const PRINT_PROGRESS: bool = false;
    const DEPTH_LIMIT: usize = 5;

    fn solve_board(
        board: Board,
        rules: &Arbiter,
        validate_one_solution: SolveType,
    ) -> ConstraintResult {
        if !rules.check(&board) {
            warn!("Presented board is invalid");
            return ConstraintResult::Contradiction;
        }

        let all_options = PossibleDigits::from(&board);
        ConstraintSolver::solve(
            board,
            all_options,
            rules,
            Self::DEPTH_LIMIT,
            validate_one_solution,
        )
    }
    fn solve(
        mut board: Board,
        mut all_options: PossibleDigits,
        rules: &Arbiter,
        remaining_depth: usize,
        validate_one_solution: SolveType,
    ) -> ConstraintResult {
        'work_loop: loop {
            // Shake out the constrained cells
            match Self::resolve_constraints(board, all_options, rules) {
                PartialConstraintResult::Complete(result) => return result,
                PartialConstraintResult::Incomplete {
                    board: new_board,
                    all_options: new_options,
                } => {
                    board = new_board;
                    all_options = new_options;
                }
            }

            if rules.is_solved(&board) {
                if Self::PRINT_PROGRESS {
                    info!("{}: Final:", remaining_depth);
                    board.print();
                }
                return ConstraintResult::Solvable(board);
            } else if remaining_depth == 0 {
                return ConstraintResult::DepthLimit(all_options);
            }

            // If the constraints made no progress do some guess and check to rule out possibilities
            let mut did_work = false;

            let mut unset_cells: Vec<_> = board
                .board
                .iter()
                .enumerate()
                .filter_map(|(i, cell)| cell.is_none().then_some(i))
                .collect(); // TODO: reuse one allocation for this
            unset_cells.sort_unstable_by_key(|i| all_options.get_index(*i).count());

            // Search with less depth first. Only if that is unsuccessful, increase it
            for search_depth in 0..remaining_depth {
                for idx in unset_cells.iter().copied() {
                    let mut new_options = PossibleDigits::new_empty();
                    let mut solved_board = None;
                    let mut num_solved = 0usize;
                    let mut reached_depth_limit = false;
                    let options = all_options.get_index(idx);
                    assert!(board.board[idx].is_none());
                    assert!(options.count() > 0);
                    // TODO: DigitSet does not have an iterator
                    for digit in Digit::DIGITS {
                        if options.get(digit) {
                            board.board[idx] = Some(digit);
                            match ConstraintSolver::solve(
                                board.clone(),
                                all_options.clone(),
                                rules,
                                search_depth,
                                validate_one_solution,
                            ) {
                                ConstraintResult::Ambiguous => {
                                    assert!(
                                        !matches!(
                                            validate_one_solution,
                                            SolveType::FindFirstSolution
                                        ),
                                        "Cannot be ambiguous, should have taken the first solution"
                                    );
                                    return ConstraintResult::Ambiguous;
                                }
                                ConstraintResult::Contradiction => (),
                                ConstraintResult::DepthLimit(possible) => {
                                    reached_depth_limit = true;
                                    new_options.union(&possible);
                                }
                                ConstraintResult::Solvable(solved) => match validate_one_solution {
                                    SolveType::ValidateOneSolution => {
                                        num_solved += 1;
                                        if num_solved > 1 {
                                            return ConstraintResult::Ambiguous;
                                        }
                                        new_options.union(&PossibleDigits::from(&solved));
                                        solved_board = Some(solved);
                                    }
                                    SolveType::FindFirstSolution => {
                                        return ConstraintResult::Solvable(solved);
                                    }
                                },
                            }
                        }
                    }
                    match (reached_depth_limit, num_solved) {
                        (_, 2..) => return ConstraintResult::Ambiguous,
                        (true, _) => {
                            did_work |= all_options != new_options;
                            all_options = new_options;
                        }
                        (false, 0) => return ConstraintResult::Contradiction,
                        (false, 1) => {
                            return ConstraintResult::Solvable(
                                solved_board.expect("Should be some if num_solved > 0"),
                            );
                        }
                    }
                    match all_options.get_index(idx).count() {
                        0 => {
                            unreachable!()
                        }
                        1 => {
                            let digit = all_options.get_index_mut(idx).first().expect("Count is 1");
                            board.board[idx] = Some(digit);
                            continue 'work_loop;
                        }
                        _ => board.board[idx] = None,
                    }
                }
                if did_work {
                    break;
                }
            }

            if rules.is_solved(&board) {
                if Self::PRINT_PROGRESS {
                    info!("{}: Final:", remaining_depth);
                    board.print();
                }
                return ConstraintResult::Solvable(board);
            } else if !did_work {
                if Self::PRINT_PROGRESS {
                    info!("{}: Final:", remaining_depth);
                    board.print();
                }
                return ConstraintResult::DepthLimit(all_options);
            }
        } // end main loop
    }

    fn resolve_constraints(
        mut board: Board,
        mut all_options: PossibleDigits,
        rules: &Arbiter,
    ) -> PartialConstraintResult {
        loop {
            let mut did_work = false;

            for idx in 0..board.len() {
                if board.board[idx].is_some() {
                    continue;
                }
                let options = all_options.get_index_mut(idx);
                match options.count() {
                    0 => unreachable!(),
                    1 => {
                        let digit = options
                            .first()
                            .expect("count == 1 so set must be non-empty");
                        board.board[idx] = Some(digit);
                        if !rules.check_one(&board, idx) {
                            // Last option left does not fit
                            return PartialConstraintResult::Complete(
                                ConstraintResult::Contradiction,
                            );
                        }
                        did_work = true;
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
                        match options.count() {
                            0 => {
                                return PartialConstraintResult::Complete(
                                    ConstraintResult::Contradiction,
                                );
                            }
                            1 => {
                                let digit = options.first().expect("Count is 1");
                                board.board[idx] = Some(digit);
                                did_work = true;
                            }
                            _ => board.board[idx] = None,
                        }
                    }
                }
            }
            assert!(rules.check(&board));
            if !did_work {
                return PartialConstraintResult::Incomplete { board, all_options };
            }
        }
    }
}
impl Solver for ConstraintSolver {
    fn make_move(&mut self, board: &Board, rules: &Arbiter) -> Action {
        match Self::solve_board(board.clone(), rules, SolveType::FindFirstSolution) {
            ConstraintResult::Solvable(solution) => make_move_from_solution(board, &solution),
            ConstraintResult::Ambiguous => {
                panic!("Solver should not return ambiguous when validate_one_solution = false")
            }
            ConstraintResult::Contradiction => {
                info!("ConstraintSolver proved board is unsolvable");
                Action::Abort
            }
            ConstraintResult::DepthLimit(choices) => {
                error!("ConstraintSolver reached depth limit and could not solve board");
                choices.print();
                Action::Abort
            }
        }
    }
}
impl Verifier for ConstraintSolver {
    fn verify(&mut self, board: &Board, rules: &Arbiter) -> BoardStatus {
        if rules.is_solved(board) {
            return BoardStatus::AlreadySolved;
        }
        match Self::solve_board(board.clone(), rules, SolveType::ValidateOneSolution) {
            ConstraintResult::Ambiguous => BoardStatus::MultipleSolutions,
            ConstraintResult::Contradiction => BoardStatus::Unsolvable,
            ConstraintResult::DepthLimit(_) => todo!(),
            ConstraintResult::Solvable(_) => BoardStatus::OneSolution,
        }
    }
}
