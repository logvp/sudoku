use sudoku::{
    BacktrackingSolver, Board, GameState, Solver, SudokuBox, SudokuColumn, SudokuRow, SudokuRule,
};

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
        let digit = this_board.get(x, y).unwrap_or(0);
        this_board.set(x, y, (digit + 1) % 10);

        let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
        // Standard sudoku rules
        rules.push(Box::new(SudokuRow));
        rules.push(Box::new(SudokuColumn));
        rules.push(Box::new(SudokuBox));
        let mut game = GameState::new(this_board, rules);
        let mut solver = BacktrackingSolver::default();

        loop {
            let action = solver.make_move(&game);

            let status = game.update(action);
            if status.is_err() {
                break;
            }

            assert!(game.check());

            if game.solved() {
                solvable += 1;
                break;
            }
        }
    }
    println!("Done!");
    println!("Solvable: {}", solvable);
}
