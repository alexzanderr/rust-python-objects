#![allow(dead_code)]

use std::fmt;
use std::hash::Hash;
use crate::Int;
use crate::Char;
use crate::_String;
use crate::Bool;


/// supreme enum
#[derive(Eq, Hash, PartialEq)]
pub enum Hashable<T>
where
    T: Sized + Eq + Hash + PartialEq, {
    /// char object
    Char(Char),
    /// int32 object
    Int(Int<T>),
    /// String object
    String(_String),
    /// Bool object
    Bool(Bool),
}


impl<T> fmt::Display for Hashable<T>
where
    T: Sized + fmt::Display + Eq + Hash + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // why there is &* ?
        // it works without both..
        match &*self {
            Hashable::Char(_char) => write!(f, "{}", _char),
            Hashable::Int(_int) => write!(f, "{}", _int),
            Hashable::String(_string) => write!(f, "{}", _string),
            Hashable::Bool(_bool) => write!(f, "{}", _bool),
        }
    }
}

impl<T> fmt::Debug for Hashable<T>
where
    T: Sized + fmt::Display + fmt::Debug + Eq + Hash + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // why there is &* ?
        // it works without both..
        match &*self {
            Hashable::Char(_char) => write!(f, "{:?}", _char),
            Hashable::Int(_int) => write!(f, "{:?}", _int),
            Hashable::String(_string) => write!(f, "{:?}", _string),
            Hashable::Bool(_bool) => write!(f, "{:?}", _bool),
        }
    }
}
