
#![doc = include_str!("../docs/python.md")]
#![doc = include_str!("../docs/python_list/showcase.md")]

#![warn(missing_docs)]



pub mod int;
pub mod float;
pub mod string;
pub mod character;
pub mod object;
pub mod list;
pub mod dict;
pub mod builtins;
pub mod boolean;

pub use int::Int;
pub use float::Float;
pub use string::_String;
pub use character::Char;
pub use list::List;
pub use object::Object;
pub use object::_Object;
pub use dict::Dict;
pub use builtins::print;
pub use builtins::len;
pub use builtins::repr;
pub use boolean::Bool;