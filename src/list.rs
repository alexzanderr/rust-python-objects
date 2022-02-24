//! NOT VISIBLE TO THE docs.rs
//! this is only for lib.rs
//! it wasnt visible because this module somehow was private
//! now its visible


#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
)]


use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Object;
use crate::_Object;


// mod object;
// use object::Object;

/// the main component
///
/// contens structure of good docs
/// [short sentence explaining what it is]
///
/// [more detailed explanation]
///
/// [at least one code example that users can copy/paste to try it]
///
/// [even more advanced explanations if necessary]
pub struct List {
    pub _list: Vec<Object>,
}


/// its implementation
impl List {
    /// new function
    pub fn new() -> List {
        List {
            _list: vec![],
        }
    }

    // TODO
    // make an iterator for List and extract object from it
    pub fn from_list(_list: List) -> List {
        let mut temp_list: Vec<Object> = vec![];
        for _object in _list._list {
            temp_list.push(_object);
        }
        List {
            _list: temp_list,
        }
    }

    // include markdown file as doc comment for this function
    #[doc = include_str!("../docs/python_list.md")]
    pub fn from_int(_integer: i32) -> List {
        List {
            _list: vec![Object::Int(Int::new(_integer))],
        }
    }

    pub fn from_string(_string: String) -> List {
        let mut _list: Vec<Object> =
            _string.chars()
            .map(|o| Object::Char(Char::new(o)))
            .collect();
        List {
            _list,
        }
    }



    /// inline append for integer
    /// example
    ///
    /// let mut one_elem = List::new();
    /// one_elem
    ///     .append_int(123)
    ///     .append_int(123)
    ///     .append_int(123)
    ///     .append_int(123)
    ///     .append_int(123)
    ///     .append_int(123)
    ///     .append_int(123);
    /// println!("{}", one_elem);
    ///
    /// [123, 123, 123, 123, 123, 123, 123]
    ///
    pub fn append_int(&mut self, _integer: i32) -> &mut Self {
        self._list.push(Object::Int(Int::new(_integer)));
        self
    }

    pub fn append_char(&mut self, _char: char) -> &mut Self {
        self._list.push(Object::Char(Char::new(_char)));
        self
    }

    pub fn append_float(&mut self, _float: f32) -> &mut Self {
        self._list.push(Object::Float(Float::new(_float)));
        self
    }

    pub fn append_string(&mut self, string: String) -> &mut Self {
        self._list.push(Object::String(_String::from_string(string)));
        self
    }

    pub fn append_pstring(&mut self, _string: _String) -> &mut Self {
        self._list.push(Object::String(_string));
        self
    }

    pub fn append_object(&mut self, _object: Object) -> &mut Self {
        self._list.push(_object);
        self
    }

    pub fn append_list(&mut self, _list: List) -> &mut Self {

        self._list.push(Object::List(_list));
        self
    }
}

impl _Object for List {
    fn __len__(&self) -> usize {
        self._list.len()
    }

    fn __repr__(&self) -> String {
        String::from("unimplemented")
    }
}


impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // if list is empty then we print '[]'
        if self._list.len() == 0 {
            write!(f, "[]");
            return Ok(())
        }


        let zero = self._list.get(0).unwrap();
        // print the first element of the list
        match zero {
            Object::String(obj) => write!(f, "[{}", obj.__repr__()),
            Object::Char(obj) => write!(f, "[{}", obj.__repr__()),
            _ => write!(f, "[{}", zero),
        };

        // print the other elements of the list but from index = 1
        for _object in self._list.iter().skip(1) {
            match _object {
                Object::String(obj) => write!(f, ", {}", obj.__repr__()),
                Object::Char(obj) => write!(f, ", {}", obj.__repr__()),
                _ => write!(f, ", {}", _object),
            };

            // ostream.push_str(&_object.fmt(f).unwrap());
        }
        write!(f, "]")
    }
}

