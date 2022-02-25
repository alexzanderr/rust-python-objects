




use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::_Object;


/// Bool structure for True and False
#[derive(Copy, Clone)]
pub struct Bool {
    _bool: bool
}

impl Bool {
    /// create a new bool
    /// example
    /// let boolean = Bool::new(true)
    /// let boolean = Bool::new(false)
    pub fn new(_bool: bool) -> Bool {
        Bool {
            _bool
        }
    }
}

impl _Object for Bool {
    fn __str__(&self) -> String {
        match self._bool {
            true => format!("True"),
            false => format!("False")
        }
        // if self._bool {
        //     format!("True")
        // } else {
        //     format!("False")
        // }
    }

    fn __len__(&self) -> usize {
        8usize
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.__str__())
    }
}