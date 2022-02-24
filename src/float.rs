

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct Float {
    _float: f32
}

impl Float {
    pub fn new(_float: f32) -> Float {
        Float {
            _float
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self._float)
    }
}

// impl _Object for Float {
//     fn __str__(self) {
//         println!("{}", self._float);
//     }

//     fn __len__(self) -> i32 {
//         self._float as i32
//     }
// }