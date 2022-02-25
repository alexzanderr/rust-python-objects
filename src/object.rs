
// std imports
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// crate imports
use crate::List;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;


pub trait _Object {
    fn __repr__(&self) -> String;
    fn __len__(&self) -> usize;
    fn __str__(&self) -> String;
}


// use int::Int;
// use float::Float;
// use string::SString;
// use _char::Char;
// use list::Lits


// #[derive(Eq, Hash, PartialEq)]
pub enum Object {
    Char(Char),
    Int(Int),
    Float(Float),
    String(_String),
    List(List),
    Bool(Bool),
}


impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // why there is &* ?
        // it works without both..
        match &*self {
            Object::Char(_char) => write!(f, "{}", _char),
            Object::Int(_int) => write!(f, "{}", _int),
            Object::Float(_float) => write!(f, "{}", _float),
            Object::String(_string) => write!(f, "{}", _string),
            Object::List(_list) => write!(f, "{}", _list),
            Object::Bool(_bool) => write!(f, "{}", _bool),
            // _ => Ok(())
        }
    }
}
