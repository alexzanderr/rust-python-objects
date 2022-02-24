



use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Int {
    _integer: i32
}

impl Int {
    pub fn new(_integer: i32) -> Int {
        Int {
            _integer
        }
    }
}

// impl _Object for Int {
//     fn __str__(self) {
//         println!("{}", self._integer);
//     }

//     fn __len__(self) -> i32 {
//         self._integer
//     }
// }

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._integer)
    }
}