



use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::_Object;

/// Int struct, holds and _integer: i32
pub struct Int {
    _integer: i32
}

impl Int {
    /// create a new Int object
    ///
    /// let integer = Int::new(123)
    pub fn new(_integer: i32) -> Self {
        Int {
            _integer
        }
    }

    /// create new Int object from static str
    /// usage
    /// let integer = Int::from_str("123")
    pub fn from_str(_str: &str) -> Self {
        let mut _integer: i32 = 0;
        for _char in _str.chars() {
            _integer = _integer * 10 + _char.to_digit(10).unwrap() as i32;
        }
        Int {
            _integer
        }
    }
}

impl _Object for Int {
    fn __repr__(&self) -> String {
        format!("{}", self._integer)
    }

    fn __len__(&self) -> usize {
        self._integer as usize
    }

    fn __str__(&self) -> String {
        format!("{}", self._integer)
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._integer)
    }
}