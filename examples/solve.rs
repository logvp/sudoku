use sudoku::{Arbiter, BacktrackingSolver, Board, Digit, Solver, standard_sudoku_rules};

fn main() {
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

    let mut solvable = 0;
    for i in 0..board.len() {
        let mut this_board = board.clone();

        let (x, y) = this_board.xy(i);
        let digit = this_board.get(x, y).unwrap_or(Digit::_9);
        this_board.set(x, y, digit.shift());

        let rules = Arbiter::new(standard_sudoku_rules());
        let mut solver = BacktrackingSolver::default();

        loop {
            let status = rules.step(&mut this_board, &mut solver);
            if !status.is_ok() {
                break;
            }

            assert!(rules.check(&this_board));

            if rules.is_solved(&this_board) {
                solvable += 1;
                break;
            }
        }
    }
    println!("Done!");
    println!("Solvable: {}", solvable);
}
