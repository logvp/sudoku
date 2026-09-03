use super::{Board, DigitSet, SudokuRule};

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

pub struct KnightsMove;
impl SudokuRule for KnightsMove {
    fn check_one(&self, board: &Board, index: usize) -> bool {
        const KNIGHT_OFFSETS: [(isize, isize); 8] = [
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
        ];
        let (x, y) = board.xy(index);
        if let Some(digit) = board.get(x, y) {
            for (d_x, d_y) in KNIGHT_OFFSETS {
                let Ok(i) = usize::try_from(x as isize + d_x) else {
                    continue;
                };
                let Ok(j) = usize::try_from(y as isize + d_y) else {
                    continue;
                };
                if !board.in_bounds(i, j) {
                    continue;
                }
                if Some(digit) == board.get(i, j) {
                    return false;
                }
            }
        }
        true
    }
}

pub type Line = Vec<usize>;
pub struct ThermalSudoku {
    thermometers: Vec<Line>,
    lookup: [Vec<usize>; Board::WIDTH * Board::HEIGHT], // board index -> list of thermometers
}
impl ThermalSudoku {
    pub fn new(thermometers: Vec<Line>) -> Self {
        let mut lookup = std::array::from_fn(|_| Vec::new());
        for (thermometer_id, thermometer) in thermometers.iter().enumerate() {
            for idx in thermometer.iter() {
                lookup[*idx].push(thermometer_id);
            }
        }
        Self {
            thermometers,
            lookup,
        }
    }

    fn check_thermometer(thermometer: &Line, board: &Board) -> bool {
        let mut last = 0;
        for index in thermometer {
            if let Some(digit) = board.board[*index] {
                let num: u32 = digit.into();
                if num <= last {
                    return false;
                }
                last = num;
            }
        }
        true
    }
}
impl SudokuRule for ThermalSudoku {
    fn check_one(&self, board: &Board, index: usize) -> bool {
        for thermometer_id in &self.lookup[index] {
            if !Self::check_thermometer(&self.thermometers[*thermometer_id], board) {
                return false;
            }
        }
        true
    }

    fn check(&self, board: &Board) -> bool {
        for thermometer in self.thermometers.iter() {
            if !Self::check_thermometer(thermometer, board) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_rule() {
        let row_rule = SudokuRow;

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(row_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ]);
        assert!(row_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!row_rule.check(&board));
    }

    #[test]
    fn test_column_rule() {
        let column_rule = SudokuColumn;

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(column_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
        ]);
        assert!(column_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!column_rule.check(&board));
    }

    #[test]
    fn test_box_rule() {
        let box_rule = SudokuBox;

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(box_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
        ]);
        assert!(box_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!box_rule.check(&board));
    }

    #[test]
    fn test_knights_move() {
        let knights_rule = KnightsMove;

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(knights_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(knights_rule.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!knights_rule.check(&board));
    }

    #[test]
    fn test_thermal() {
        let thermal = ThermalSudoku::new(vec![vec![0, 1, 2, 3]]);

        #[rustfmt::skip]
        let board: Board = Board::make([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(thermal.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 2, 0, 4, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(thermal.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            2, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!thermal.check(&board));

        #[rustfmt::skip]
        let board: Board = Board::make([
            1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert!(!thermal.check(&board));
    }
}
