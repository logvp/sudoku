use std::fmt::{Debug, Display};

use crate::BoardDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    _1 = 1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}
impl TryFrom<u32> for Digit {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Digit::_1),
            2 => Ok(Digit::_2),
            3 => Ok(Digit::_3),
            4 => Ok(Digit::_4),
            5 => Ok(Digit::_5),
            6 => Ok(Digit::_6),
            7 => Ok(Digit::_7),
            8 => Ok(Digit::_8),
            9 => Ok(Digit::_9),
            _ => Err(()),
        }
    }
}
impl From<Digit> for u32 {
    fn from(val: Digit) -> Self {
        match val {
            Digit::_1 => 1,
            Digit::_2 => 2,
            Digit::_3 => 3,
            Digit::_4 => 4,
            Digit::_5 => 5,
            Digit::_6 => 6,
            Digit::_7 => 7,
            Digit::_8 => 8,
            Digit::_9 => 9,
        }
    }
}
impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Digit::_1 => "1",
            Digit::_2 => "2",
            Digit::_3 => "3",
            Digit::_4 => "4",
            Digit::_5 => "5",
            Digit::_6 => "6",
            Digit::_7 => "7",
            Digit::_8 => "8",
            Digit::_9 => "9",
        })
    }
}
impl Digit {
    pub const DIGITS: [Digit; 9] = [
        Digit::_1,
        Digit::_2,
        Digit::_3,
        Digit::_4,
        Digit::_5,
        Digit::_6,
        Digit::_7,
        Digit::_8,
        Digit::_9,
    ];

    pub const MAX_DIGIT: Digit = Digit::_9;
    pub const ARRAY_SIZE: usize = Digit::MAX_DIGIT as usize + 1;

    pub fn shift(self) -> Digit {
        match self {
            Digit::_1 => Digit::_2,
            Digit::_2 => Digit::_3,
            Digit::_3 => Digit::_4,
            Digit::_4 => Digit::_5,
            Digit::_5 => Digit::_6,
            Digit::_6 => Digit::_7,
            Digit::_7 => Digit::_8,
            Digit::_8 => Digit::_9,
            Digit::_9 => Digit::_1,
        }
    }

    pub fn next(self) -> Option<Digit> {
        match self {
            Digit::_1 => Some(Digit::_2),
            Digit::_2 => Some(Digit::_3),
            Digit::_3 => Some(Digit::_4),
            Digit::_4 => Some(Digit::_5),
            Digit::_5 => Some(Digit::_6),
            Digit::_6 => Some(Digit::_7),
            Digit::_7 => Some(Digit::_8),
            Digit::_8 => Some(Digit::_9),
            Digit::_9 => None,
        }
    }
}

#[derive(Default, Clone)]
pub struct DigitSet {
    storage: [bool; Digit::ARRAY_SIZE],
}
impl DigitSet {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }
    #[inline]
    pub fn set(&mut self, digit: Digit) {
        self.storage[digit as usize] = true;
    }
    #[inline]
    pub fn clear(&mut self, digit: Digit) {
        self.storage[digit as usize] = false;
    }
    #[inline]
    pub fn get(&self, digit: Digit) -> bool {
        self.storage[digit as usize]
    }
    pub fn invert(&mut self) {
        for x in &mut self.storage {
            *x = !*x;
        }
    }
    pub fn count(&self) -> usize {
        let mut count = 0;
        for digit in Digit::DIGITS {
            if self.get(digit) {
                count += 1;
            }
        }
        count
    }
    pub fn first(&self) -> Option<Digit> {
        Digit::DIGITS.into_iter().find(|&digit| self.get(digit))
    }
    pub fn union(&mut self, other: &Self) {
        // TODO: Can't iterate over DigitSet
        for digit in Digit::DIGITS {
            if other.get(digit) {
                self.set(digit);
            }
        }
    }
}
impl PartialEq for DigitSet {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Can't iterate over DigitSet
        for digit in Digit::DIGITS {
            if self.get(digit) != other.get(digit) {
                return false;
            }
        }
        true
    }
}
impl Debug for DigitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DigitSet {{")?;
        let mut first = true;
        // TODO: Can't iterate over DigitSet
        for digit in Digit::DIGITS {
            if self.get(digit) {
                if !first {
                    write!(f, ",")?;
                }
                write!(f, " {}", digit)?;
                first = false;
            }
        }
        write!(f, " }}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // Board does not preserve any invariants so this can be public
    pub board: [Option<Digit>; Self::HEIGHT * Self::WIDTH],
}
impl Default for Board {
    fn default() -> Self {
        Self {
            board: [None; Self::HEIGHT * Self::WIDTH],
        }
    }
}
impl Board {
    pub const HEIGHT: usize = 9;
    pub const WIDTH: usize = 9;

    pub fn make(board: [u32; Self::HEIGHT * Self::WIDTH]) -> Self {
        Self {
            board: board.map(Digit::try_from).map(Result::ok),
        }
    }

    pub fn len(&self) -> usize {
        self.board.len()
    }

    pub fn height(&self) -> usize {
        Self::HEIGHT
    }

    pub fn width(&self) -> usize {
        Self::WIDTH
    }

    pub fn first_open_index(&self) -> Option<usize> {
        for i in 0..self.len() {
            if let Some(None) = self.board.get(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn next_open_index(&self, after: usize) -> Option<usize> {
        for i in (after + 1)..self.len() {
            if let Some(None) = self.board.get(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(self.in_bounds(x, y));
        y * Self::WIDTH + x
    }

    pub fn xy(&self, index: usize) -> (usize, usize) {
        assert!(index < self.board.len());
        (index % Self::WIDTH, index / Self::WIDTH)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Digit> {
        self.board[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, digit: Digit) {
        self.board[self.index(x, y)] = Some(digit)
    }

    pub fn reset(&mut self, x: usize, y: usize) {
        self.board[self.index(x, y)] = None
    }

    pub fn has_gaps(&self) -> bool {
        self.board.iter().any(Option::is_none)
    }

    pub fn print(&self) {
        for j in 0..self.height() {
            for i in 0..self.width() {
                if let Some(digit) = self.get(i, j) {
                    print!("{} ", digit);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    pub fn to_description(self) -> BoardDescription {
        BoardDescription {
            width: Board::WIDTH,
            height: Board::HEIGHT,
            digits: self
                .board
                .map(|d| match d {
                    Some(d) => u32::from(d),
                    None => 0,
                })
                .to_vec(),
        }
    }
}
