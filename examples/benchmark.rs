use sudoku::{
    Board, BoardStatus, Digit,
    optimized::{is_minimal_standard, solve_standard, verify_standard},
};

fn main() {
    // colog::basic_builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .init();

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

    let mut solvable: usize = 0;
    let mut valid: usize = 0;
    let mut minimal: usize = 0;
    for j in 0..board.len() {
        let mut holed_board = board.clone();
        let (x, y) = holed_board.xy(j);
        holed_board.reset(x, y);
        for digit in Digit::DIGITS {
            for i in 0..holed_board.len() {
                let mut this_board = holed_board.clone();

                let (x, y) = this_board.xy(i);
                let digit = this_board.get(x, y).unwrap_or(digit);
                this_board.set(x, y, digit.shift());

                if let Some(_board) = solve_standard(this_board.clone()) {
                    solvable += 1;
                }

                if verify_standard(this_board.clone()) == BoardStatus::OneSolution {
                    valid += 1;
                }

                if is_minimal_standard(this_board.clone()) {
                    minimal += 1;
                }
            }
        }
    }
    println!("Done!");
    println!("Solvable: {}", solvable);
    println!("Valid:    {}", valid);
    println!("Minimal:  {}", minimal);
}
