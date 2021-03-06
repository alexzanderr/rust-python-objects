use std::fmt;
use std::any::type_name;


// use crate::Object;
use crate::_Object;
use crate::Iterable;

/// print(object);
pub fn print<T: fmt::Display>(arg: T) {
    println!("{}", arg);
}

/// print(object);
pub fn printd<T: fmt::Debug>(arg: T) {
    println!("{:?}", arg);
}


/// print(object);
pub fn dprint<T: fmt::Debug>(arg: T) {
    dbg!("{}", arg);
}

/// len(object);
pub fn len<T: Iterable>(_object: &T) -> usize {
    _object.__len__()
}

/// repr(object);
pub fn repr<T: _Object>(_object: &T) -> String {
    _object.__repr__()
}

/// _str(object);
/// sorry but i cant name this 'str'
/// because there is a rust data type called string slice which is called
/// guess: str
/// that remains the convension
pub fn _str<T: _Object>(_object: &T) -> String {
    _object.__str__()
}

/// get the type of an object
pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

// / the maximum from an iterable
// pub fn max<T: Iterable>(
//     _iterable: T,
// ) -> Object {
//     Object::Int32(Int::<i32>::new(123))
// }
