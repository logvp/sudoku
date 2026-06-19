use sudoku::{
    BacktrackingSolver, Board, GameState, Solver, SudokuBox, SudokuColumn, SudokuRow, SudokuRule,
};

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let board = Board::default();
    let mut rules: Vec<Box<dyn SudokuRule>> = Vec::new();
    // Standard sudoku rules
    rules.push(Box::new(SudokuRow));
    rules.push(Box::new(SudokuColumn));
    rules.push(Box::new(SudokuBox));
    let mut game = GameState::new(board, rules);

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
            game.print_board();
            break;
        }
    }
}
