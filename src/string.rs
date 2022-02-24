

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


use crate::_Object;

pub struct _String {
    _string: String
}

impl _String {
    pub fn new() -> _String {
        _String {
            _string: String::new()
        }
    }

    pub fn from_string(_string: String) -> Self {
        _String {
            _string
        }
    }

}

impl _Object for _String {
    fn __len__ (&self) -> usize {
        self._string.len()
    }

    fn __repr__(&self) -> String {
        format!("'{}'", self._string)
    }
}

impl Display for _String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._string)
    }
}

