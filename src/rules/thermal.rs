use crate::{Board, SudokuRule};

use super::Line;

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
