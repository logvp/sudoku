use crate::{Board, SudokuRule};

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
