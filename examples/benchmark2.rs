use sudoku::{Board, optimized::reduce_standard};

fn main() {
    // colog::basic_builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .init();

    #[rustfmt::skip]
    let mut board: Board = Board::make([
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

    let mut reducible: usize = 0;
    for i in 0..board.len() {
        let (x, y) = board.xy(i);
        board.reset(x, y);
        if let Some(reduced) = reduce_standard(board.clone()) {
            reducible += 1;
            reduced.print();
        }
    }
    println!("Done!");
    println!("Reducible: {}", reducible);
}
