use std::fmt;
use crate::_Object;
use crate::type_of;
use crate::_Hashable;


#[derive(Copy)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
/// Int struct, holds and _integer: i32
pub struct Int<T: Sized> {
    _integer: T,
}

impl<T> _Hashable for Int<T> {
}


/// Int::new(_anything_)
impl<T> Int<T>
where
    T: Sized,
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


/// Int::from(&str)
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


/// Int::from(&str)
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


impl<T> From<T> for Int<T>
where
    T: Sized,
{
    /// this can be any of
    /// i8, i16, i32, i64, i128, i256
    /// u8, u16, u32, u64, u128, u256
    /// usize
    fn from(_integer: T) -> Self {
        Int {
            _integer,
        }
    }
}

/// object trait
impl<T> _Object for Int<T>
where
    T: Sized + fmt::Display,
{
    fn __repr__(&self) -> String {
        format!("{}", self._integer)
    }

    fn __str__(&self) -> String {
        format!("{}", self._integer)
    }
}


/// display print
impl<T> fmt::Display for Int<T>
where
    T: Sized + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self._integer)
    }
}


/// debug print
impl<T> fmt::Debug for Int<T>
where
    T: Sized + fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _type = type_of(&self._integer);
        write!(f, "Int<{}> {{ _integer: {} }}", _type, self._integer)
    }
}
