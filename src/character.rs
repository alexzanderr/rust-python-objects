

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Char {
    _char: char
}

impl Char {
    pub fn new(_char: char) -> Char {
        Char {
            _char
        }
    }

    pub fn __repr__(&self) -> String {
        format!("'{}'", self._char)
    }
}

// impl _Object for Char {
//     fn __str__(self) {
//         prCharln!("{}", self._Chareger);
//     }

//     fn __len__(self) -> i32 {
//         self._Chareger
//     }
// }

impl Display for Char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._char)
    }
}