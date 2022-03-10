use std::fmt;

use crate::_Object;


/// Bool structure for True and False
#[derive(Copy)]
#[derive(Clone)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Bool {
    _bool: bool,
}

impl Bool {
    /// create a new bool
    /// example
    /// let boolean = Bool::new(true)
    /// let boolean = Bool::new(false)
    pub fn new(_bool: bool) -> Bool {
        Bool {
            _bool,
        }
    }
}


impl From<bool> for Bool {
    fn from(_bool: bool) -> Self {
        Bool {
            _bool,
        }
    }
}


impl From<i32> for Bool {
    fn from(_int: i32) -> Self {
        if _int > 0 {
            return Bool {
                _bool: true
            };
        }
        Bool {
            _bool: false
        }
    }
}


impl _Object for Bool {
    fn __str__(&self) -> String {
        if self._bool {
            format!("True")
        } else {
            format!("False")
        }
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}


impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.__str__())
    }
}


impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bool<{}>", self.__str__())
    }
}


impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        self._bool == *other
    }
}
