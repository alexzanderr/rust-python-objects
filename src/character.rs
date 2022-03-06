use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


use crate::_Object;


#[derive(Copy, Clone)]
/// Char struct that handles rust char
pub struct Char {
    _char: char,
}

impl Char {
    /// constructor creates a new Char from rust char
    /// note: its not like from, because from function
    /// creates something from something different than what we get
    pub fn new(_char: char) -> Self {
        Char {
            _char,
        }
    }
}

impl From<char> for Char {
    fn from(_char: char) -> Self {
        Char {
            _char,
        }
    }
}

impl Default for Char {
    fn default() -> Self {
        Char {
            _char: '0'
        }
    }
}

impl _Object for Char {
    fn __str__(&self) -> String {
        String::from(self._char)
    }

    fn __repr__(&self) -> String {
        format!("'{}'", self._char)
    }

    fn __len__(&self) -> usize {
        8
    }
}

impl Display for Char {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> Result {
        if formatter.alternate() {
            write!(formatter, "'{}'", self._char)
        } else {
            write!(formatter, "{}", self._char)
        }
    }
}
