

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::_Object;


#[derive(Copy, Clone)]
pub struct Char {
    _char: char
}

impl Char {
    pub fn new(_char: char) -> Char {
        Char {
            _char
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._char)
    }
}