



use std::fmt;
use std::any::type_name;

// use crate::Object;
use crate::_Object;

/// print(object);
pub fn print<T: fmt::Display>(arg: T) {
    println!("{}", arg);
}

/// print(object);
// pub fn printd<T: Display>(arg: T) {
//     println!("{:?}", arg);
// }

/// len(object);
pub fn len<T: _Object>(_object: &T) -> usize {
    _object.__len__()
}

/// repr(object);
pub fn repr<T: _Object>(_object: &T) -> String {
    _object.__repr__()
}

pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}