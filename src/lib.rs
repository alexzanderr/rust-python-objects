
#![doc = include_str!("../docs/python.md")]
#![doc = include_str!("../docs/python_list/showcase.md")]

#![warn(missing_docs)]



mod int;
mod float;
mod string;
mod character;
mod object;
mod dict;
mod builtins;
mod boolean;
mod list;


pub use int::Int;
pub use float::Float;
pub use string::_String;
pub use character::Char;
pub use object::Object;
pub use object::_Object;
pub use boolean::Bool;
pub use dict::Dict;

// builtins.rs
pub use builtins::print;
pub use builtins::len;
pub use builtins::repr;


pub use list::List;
pub use list::Append;
pub use list::AppendFront;