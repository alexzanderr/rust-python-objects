#![doc = include_str!("../docs/python.md")]
#![doc = include_str!("../docs/python_list/showcase.md")]
//#![deny(missing_docs)]
//#![deny(dead_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::useless_format)]
#![allow(clippy::derive_hash_xor_eq)]


mod integer;
mod float;
mod string;
mod character;
mod object;
mod boolean;
mod iterable;


pub use integer::Int;
pub use float::Float;
pub use string::_String;
pub use character::Char;
pub use object::Object;
pub use object::_Object;
pub use boolean::Bool;
pub use iterable::Iterable;


// builtins.rs
mod builtins;
// simple print
pub use builtins::print;
// print with debug
pub use builtins::printd;
// debug print dbg! macro
pub use builtins::dprint;
pub use builtins::repr;
pub use builtins::type_of;
pub use builtins::len;
pub use builtins::_str;
// pub use builtins::max;


// python list
mod list;
pub use list::List;
pub use list::Append;
pub use list::AppendFront;
pub use list::Extend;


// python dict
mod dict;
pub use dict::Dict;
pub use dict::SetItem;


// traits
mod util_traits;
pub use util_traits::_Hashable;
