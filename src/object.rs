
// std imports
use std::fmt;

// crate imports
use crate::List;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;


/// the supreme _Object trait
/// that its derived types should
/// implement like all the __functions__ from python
pub trait _Object {
    /// python repr(object)
    fn __repr__(&self) -> String;
    /// python str(object)
    fn __str__(&self) -> String;
}


// use int::Int;
// use float::Float;
// use string::SString;
// use _char::Char;
// use list::Lits


/// supreme enum
pub enum Object {
    /// char object
    Char(Char),
    /// int32 object
    Int32(Int<i32>),
    /// int64 object
    Int64(Int<i64>),
    /// int128 object
    // Int128(Int<i128>),
    /// float32 object
    Float32(Float<f32>),
    /// float64 object
    Float64(Float<f64>),
    /// String object
    String(_String),
    /// List object
    List(List),
    /// Bool object
    Bool(Bool),
}


impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // why there is &* ?
        // it works without both..
        match &*self {
            Object::Char(_char) => write!(f, "{}", _char),
            Object::Int32(_int) => write!(f, "{}", _int),
            Object::Int64(_int) => write!(f, "{}", _int),
            Object::Float32(_float) => write!(f, "{}", _float),
            Object::Float64(_float) => write!(f, "{}", _float),
            Object::String(_string) => write!(f, "{}", _string),
            Object::List(_list) => write!(f, "{}", _list),
            Object::Bool(_bool) => write!(f, "{}", _bool),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // why there is &* ?
        // it works without both..
        match &*self {
            Object::Char(_char) => write!(f, "{:?}", _char),
            Object::Int32(_int) => write!(f, "{:?}", _int),
            Object::Int64(_int) => write!(f, "{:?}", _int),
            Object::Float32(_float) => write!(f, "{:?}", _float),
            Object::Float64(_float) => write!(f, "{:?}", _float),
            Object::String(_string) => write!(f, "{:?}", _string),
            Object::List(_list) => write!(f, "{:?}", _list),
            Object::Bool(_bool) => write!(f, "{:?}", _bool),
        }
    }
}
