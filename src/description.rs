use serde::{Deserialize, Serialize};

use crate::{Board, Digit, Line, Rules};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameDescription {
    pub board: BoardDescription,
    pub rules: RulesDescription,
}
impl GameDescription {
    pub fn build(self) -> (Board, Rules) {
        (self.board.build().unwrap(), self.rules.build())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BoardDescription {
    pub width: usize,
    pub height: usize,
    pub digits: Vec<u32>,
}
impl BoardDescription {
    pub fn build(self) -> Result<Board, ()> {
        if self.width != Board::WIDTH {
            return Err(());
        }
        if self.height != Board::HEIGHT {
            return Err(());
        }
        if !self.digits.is_empty() && self.digits.len() != self.width * self.height {
            return Err(());
        }
        if self.digits.is_empty() {
            return Ok(Board::default());
        } else {
            let digits: Vec<Option<Digit>> = self
                .digits
                .iter()
                .map(|n| {
                    if *n == 0 {
                        None
                    } else {
                        Some(Digit::try_from(*n).unwrap())
                    }
                })
                .collect();
            Ok(Board {
                board: digits.try_into().map_err(|_| ())?,
            })
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RulesDescription {
    pub rows: bool,
    pub cols: bool,
    pub boxes: bool,
    pub knight: bool,
    pub thermometers: Vec<Line>,
}
impl RulesDescription {
    pub fn standard_sudoku_rules() -> Self {
        Self {
            rows: true,
            cols: true,
            boxes: true,
            knight: false,
            thermometers: Vec::new(),
        }
    }

    pub fn build(self) -> Rules {
        Rules::from(self)
    }
}
