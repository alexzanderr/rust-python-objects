//!
//!
//! docs
//!
//!
//! ```rust
//! let mut python_list =
//!     List::from_string(String::from("123123"));
//! python_list.append_int(123);
//! python_list.append_float(123.123);
//! python_list.append_float(123.123);
//! python_list.append_float(123.123);
//! python_list.append_string(String::from("asdasd"));
//! python_list.append_list(
//!     List::from_string("andrew".to_string()));
//! python_list.append_pstring(
//!     _String::from_string(
//!         String::from("python string")));
//! print(&python_list);
//! print(len(&python_list));
//! ```
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
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

pub struct List {
    pub _list: Vec<Object>,
}

impl List {
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


// impl Iterator for List {
//     type Item = Object;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.iter_index >= self._list.len() {
//             // Obviously, there isn't any more data to read so let's stop here.
//             None
//         } else {
//             // We increment the position of our iterator.
//             self.iter_index += 1;
//             // We return the current value pointed by our iterator.
//             // self._list.get(self.iter_index - 1)
//         }
//     }
// }