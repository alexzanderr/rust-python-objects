
#![doc = include_str!("../docs/python.md")]
#![doc = include_str!("../docs/python_list/showcase.md")]

#![deny(missing_docs)]
#![deny(dead_code)]

#![allow(clippy::module_inception)]
#![allow(clippy::useless_format)]


mod integer;
mod float;
mod string;
mod character;
mod object;
mod dict;
mod builtins;
mod boolean;
mod list;
mod iterable;


pub use integer::Int;
pub use float::Float;
pub use string::_String;
pub use character::Char;
pub use object::Object;
pub use object::_Object;
pub use boolean::Bool;
pub use dict::Dict;
pub use iterable::Iterable;


// builtins.rs
pub use builtins::print;
pub use builtins::repr;
pub use builtins::type_of;
pub use builtins::len;
pub use builtins::_str;
pub use builtins::max;


pub use list::List;
pub use list::Append;
pub use list::AppendFront;
