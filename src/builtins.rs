



use std::fmt::Display;

// use crate::Object;
use crate::_Object;


pub fn print<T: Display>(arg: T) {
    println!("{}", arg);
}


pub fn len<T: _Object>(_object: &T) -> usize {
    _object.__len__()
}

pub fn repr<T: _Object>(_object: &T) -> String {
    _object.__repr__()
}