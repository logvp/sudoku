use sudoku::{Board, BoardStatus, Digit, verify};

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

    let mut already_solved = 0;
    let mut proper = 0;
    let mut ambiguous = 0;
    let mut unsolvable = 0;
    for j in 0..board.len() {
        let mut board = board.clone();
        let (x, y) = board.xy(j);
        board.reset(x, y);

        for i in 0..board.len() {
            let mut this_board = board.clone();

            let (x, y) = this_board.xy(i);
            let digit = this_board.get(x, y).unwrap_or(Digit::_9);
            this_board.set(x, y, digit.shift());

            match verify(this_board, None) {
                BoardStatus::AlreadySolved => already_solved += 1,
                BoardStatus::OneSolution => proper += 1,
                BoardStatus::MultipleSolutions => ambiguous += 1,
                BoardStatus::Unsolvable => unsolvable += 1,
            }
        }
    }
    println!("Done!");
    println!("Already Solved = {}", already_solved);
    println!("Proper Sudoku  = {}", proper);
    println!("Ambiguous      = {}", ambiguous);
    println!("Unsolvable     = {}", unsolvable);
}
