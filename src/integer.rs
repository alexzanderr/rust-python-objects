

use std::fmt;
use crate::_Object;

/// Int struct, holds and _integer: i32
#[derive(Copy)]
#[derive(Clone)]
#[derive(Default)]
pub struct Int<T: Sized> {
    _integer: T,
}


impl<T> Int<T> where T: Sized
{
    /// create a new Int object
    ///
    /// let integer = Int::new(123)
    pub fn new(_integer: T) -> Self {
        Int {
            _integer,
        }
    }
}

/// this must be called like this
///  Object::Int32(Int::<i32>::new(123))
/// and its very ugly
// impl Int<i32> {
//     /// create a new Int object
//     ///
//     /// let integer = Int::new(123)
//     pub fn new(_integer: i32) -> Self {
//         Int {
//             _integer,
//         }
//     }
// }

/// this must be called like this
///  Object::Int64(Int::<i64>::new(123))
/// and its very ugly
// impl Int<i64> {
//     /// create a new Int object
//     ///
//     /// let integer = Int::new(123)
//     pub fn new(_integer: i64) -> Self {
//         Int {
//             _integer,
//         }
//     }
// }



impl From<&str> for Int<i32> {
    /// create new Int object from static str
    /// usage
    /// let integer = Int::from_str("123")
    fn from(_static_string: &str) -> Self {
        let mut _integer: i32 = 0;
        for _char in _static_string.chars() {
            _integer =
                _integer * 10 + _char.to_digit(10).unwrap() as i32;
        }
        Int {
            _integer,
        }
    }
}

impl From<&str> for Int<i64> {
    /// create new Int object from static str
    /// usage
    /// let integer = Int::from_str("123")
    fn from(_static_string: &str) -> Self {
        let mut _integer: i64 = 0;
        for _char in _static_string.chars() {
            _integer =
                _integer * 10 + _char.to_digit(10).unwrap() as i64;
        }
        Int {
            _integer,
        }
    }
}

// impl From<i32> for Int<i32> {
//     fn from(_integer: i32) -> Self {
//         Int {
//             _integer
//         }
//     }
// }

// impl From<i64> for Int<i64> {
//     fn from(_integer: i64) -> Self {
//         Int {
//             _integer
//         }
//     }
// }

impl<T> From<T> for Int<T>
where T: Sized
{
    /// this can be any of
    /// i8, i16, i32, i64, i128, i256
    /// u8, u16, u32, u64, u128, u256
    /// usize
    fn from(_integer: T) -> Self {
        Int {
            _integer
        }
    }
}

impl _Object for Int<i32> {
    fn __repr__(&self) -> String {
        format!("{}", self._integer)
    }

    fn __len__(&self) -> usize {
        self._integer as usize
    }

    fn __str__(&self) -> String {
        format!("{}", self._integer)
    }
}

impl _Object for Int<i64> {
    fn __repr__(&self) -> String {
        format!("{}", self._integer)
    }

    fn __len__(&self) -> usize {
        self._integer as usize
    }

    fn __str__(&self) -> String {
        format!("{}", self._integer)
    }
}

impl<T> fmt::Display for Int<T>
where T: Sized + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self._integer)
    }
}



impl PartialEq for Int<i32> {
    fn eq(&self, other: &Self) -> bool {
        self._integer == other._integer
    }

    fn ne(&self, other: &Self) -> bool {
        self._integer != other._integer
    }
}


impl PartialEq for Int<i64> {
    fn eq(&self, other: &Self) -> bool {
        self._integer == other._integer
    }

    fn ne(&self, other: &Self) -> bool {
        self._integer != other._integer
    }
}
