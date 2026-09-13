use crate::{Board, DigitSet, SudokuRule};

pub struct SudokuRow;
impl SudokuRow {
    fn check_row(&self, board: &Board, y: usize) -> bool {
        let mut set = DigitSet::new();
        for i in 0..board.width() {
            if let Some(digit) = board.get(i, y) {
                if set.get(digit) {
                    return false;
                } else {
                    set.set(digit);
                }
            };
        }
        true
    }
}
impl SudokuRule for SudokuRow {
    fn check(&self, board: &Board) -> bool {
        for y in 0..board.height() {
            if !self.check_row(board, y) {
                return false;
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool {
        if let Some(digit) = board.board[index] {
            let (x, y) = board.xy(index);
            for i in 0..board.width() {
                if x != i && Some(digit) == board.get(i, y) {
                    return false;
                }
            }
        }
        true
    }
}

pub struct SudokuColumn;
impl SudokuColumn {
    fn check_column(&self, board: &Board, x: usize) -> bool {
        let mut set = DigitSet::new();
        for j in 0..board.height() {
            if let Some(digit) = board.get(x, j) {
                if set.get(digit) {
                    return false;
                } else {
                    set.set(digit);
                }
            };
        }
        true
    }
}
impl SudokuRule for SudokuColumn {
    fn check(&self, board: &Board) -> bool {
        for x in 0..board.width() {
            if !self.check_column(board, x) {
                return false;
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool {
        if let Some(digit) = board.board[index] {
            let (x, y) = board.xy(index);
            for j in 0..board.height() {
                if y != j && Some(digit) == board.get(x, j) {
                    return false;
                }
            }
        }
        true
    }
}

pub struct SudokuBox;
impl SudokuRule for SudokuBox {
    fn check(&self, board: &Board) -> bool {
        for i in 0..(board.width() / 3) {
            for j in 0..(board.height() / 3) {
                if !self.check_one(board, board.index(i * 3, j * 3)) {
                    return false;
                }
            }
        }
        true
    }

    fn check_one(&self, board: &Board, index: usize) -> bool {
        let (x, y) = board.xy(index);
        let mut set = DigitSet::new();
        let box_start_x = x - (x % 3);
        let box_start_y = y - (y % 3);
        for i in box_start_x..(box_start_x + 3) {
            for j in box_start_y..(box_start_y + 3) {
                if let Some(digit) = board.get(i, j) {
                    if set.get(digit) {
                        return false;
                    } else {
                        set.set(digit);
                    }
                };
            }
        }
        true
    }
}
