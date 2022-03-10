use std::fmt; // use std::ops::Index;
use std::str::Chars;

use crate::_Object;

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
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
    fn __repr__(&self) -> String {
        format!("'{}'", self._string)
    }

    fn __str__(&self) -> String {
        self._string.clone()
    }
}

impl fmt::Display for _String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self._string)
    }
}

impl fmt::Debug for _String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self._string)
    }
}

impl PartialEq<String> for _String {
    fn eq(&self, other: &String) -> bool {
        self._string == *other
    }
}


// impl Index<usize> for _String {
//     type Output = char;
//     fn index(&self, _index: usize) -> &Self::Output {
//         self._string.index(_index)
//     }
// }
