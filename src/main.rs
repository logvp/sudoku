use sudoku::Board;

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let board = Board::default();

    let solved = sudoku::solve(board).unwrap();
    println!("Solved!");
    solved.print();
}
