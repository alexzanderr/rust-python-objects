use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
// use std::ops::Index;
use std::str::Chars;

use crate::_Object;

#[derive(Clone)]
/// the struct that handles String and &str
pub struct _String {
    /// the core field
    _string: String,
}

impl _String {
    /// constructor: creates a new _String struct
    pub fn new() -> _String {
        _String {
            _string: String::new(),
        }
    }

    #[inline]
    /// _String.index(0) -> first char
    pub fn index(&self, _index: usize) -> Option<char> {
        self._string.chars().nth(_index)
    }

    /// _String.get(0) -> first char
    pub fn get(&self, _index: usize) -> Option<char> {
        self.index(_index)
    }
}

impl From<Chars<'_>> for _String {
    fn from(_chars: Chars) -> Self {
        let mut allocator = String::new();
        for _char in _chars {
            allocator.push(_char)
        }
        _String {
            _string: allocator
        }
    }
}

impl From<&str> for _String {
    fn from(_str: &str) -> Self {
        _String {
            _string: String::from(_str),
        }
    }
}
impl From<String> for _String {
    fn from(_string: String) -> Self {
        _String {
            _string,
        }
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

    fn __str__(&self) -> String {
        self._string.clone()
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
