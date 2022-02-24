
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// mod int;
// mod float;
// mod string;
// mod _char;
// mod object;
// mod list;

use crate::List;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;


pub trait _Object {
    // fn __str__(&self) -> {}
    fn __repr__(&self) -> String;
    fn __len__(&self) -> usize;
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
            // _ => Ok(())
        }
    }
}
