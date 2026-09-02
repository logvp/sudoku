use sudoku::{
    BacktrackingSolver, Board, Digit, GameState, Solver, ThermalSudoku, solve,
    standard_sudoku_rules,
};

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    #[rustfmt::skip]
    let board: Board = Board::default();

    let mut rules = standard_sudoku_rules();
    rules.push(Box::new(ThermalSudoku::new(vec![
        vec![36, 28],
        vec![37, 29, 21, 13],
        vec![38, 30, 22, 14, 6],
        vec![39, 31, 23, 15],
        vec![40, 32],
        vec![41, 33, 25, 17],
        vec![42, 34, 26],
        vec![43, 35],
        vec![37, 45],
        vec![38, 46, 54],
        vec![39, 47, 55, 63],
        vec![40, 48],
        vec![41, 49, 57, 65],
        vec![42, 50, 58, 66],
        vec![43, 51, 59],
    ])));

    if let Ok(soln) = solve(board, Some(rules)) {
        println!("Solved!");
        soln.print();
    } else {
        println!("Unsolvable")
    }

    println!("Done!");
}
