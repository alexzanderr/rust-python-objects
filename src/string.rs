use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Index;
use std::str::Chars;

use crate::_Object;

pub struct _String {
    _string: String,
}

impl _String {
    pub fn new() -> _String {
        _String {
            _string: String::new(),
        }
    }

    pub fn from_chars(_chars: Chars) -> Self {
        let mut allocator = String::new();
        for _char in _chars {
            allocator.push(_char)
        }
        _String {
            _string: allocator
        }
    }

    pub fn from_str(_str: &str) -> Self {
        _String {
            _string: String::from(_str),
        }
    }

    pub fn from_string(_string: String) -> Self {
        _String { _string }
    }
}

impl Default for _String {
    fn default() -> Self {
        Self::new()
    }
}

impl _Object for _String {
    fn __len__(&self) -> usize {
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


// impl Index<usize> for _String {
//     type Output = char;
//     fn index(&self, _index: usize) -> &Self::Output {
//         self._string.index(_index)
//     }
// }