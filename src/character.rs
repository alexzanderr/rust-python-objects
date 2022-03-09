
use std::fmt;


use crate::_Object;


#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
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

impl From<u8> for Char {
    fn from(_int: u8) -> Self {
        Char {
            _char: _int as char,
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

impl _Object for Char
{
    fn __str__(&self) -> String {
        String::from(self._char)
    }

    fn __repr__(&self) -> String {
        format!("'{}'", self._char)
    }
}

impl fmt::Display for Char {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if formatter.alternate() {
            write!(formatter, "'{}'", self._char)
        } else {
            write!(formatter, "{}", self._char)
        }
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if formatter.alternate() {
            write!(formatter, "Char<'{}'>", self._char)
        } else {
            write!(formatter, "Char<{}>", self._char)
        }
    }
}

impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        self._char == *other
    }
}
