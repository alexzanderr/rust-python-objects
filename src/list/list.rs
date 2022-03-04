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


use std::collections::VecDeque;

use std::fmt;

use std::error;

use std::ops::Deref;
use std::str::FromStr;

use crate::_Object;
use crate::Object;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;


use super::Append;

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
// #[derive(Copy, Clone)]
pub struct List {
    /// _list which holds all the python objects together
    pub _list: VecDeque<Object>,
}


/// its implementation
impl List {
    /// new function
    pub fn new() -> List {
        List {
            _list: VecDeque::new(),
        }
    }

    // TODO
    // make an iterator for List and extract object from it
}




impl _Object for List {
    fn __len__(&self) -> usize {
        self._list.len()
    }

    fn __repr__(&self) -> String {
        String::from("unimplemented")
    }

    fn __str__(&self) -> String {
        String::from("unimplemented")
    }
}


impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

///
///
///
impl Deref for List {
    type Target = VecDeque<Object>;


    /// usage
    /// for o in python_list.iter() {
    ///     print(o)
    /// }
    fn deref(&self) -> &Self::Target {
        &self._list
    }
}


impl From<&str> for List {
    fn from(_static_string: &str) -> List {
        List {
            _list:
                _static_string.chars()
                .map(|c| Object::Char(Char::new(c)))
                .collect()
        }
    }
}

/// creates a list from string
/// example
/// let list = List::from("q23123123".to_string())
/// or
/// let list = List::from(String::from("q23123123"))
impl From<&String> for List {
    fn from(_string: &String) -> List {
        List {
            _list:
                _string.chars()
                .map(|c| Object::Char(Char::new(c)))
                .collect()
        }
    }
}

impl From<String> for List {
    fn from(_string: String) -> List {
        List {
            _list:
                _string.chars()
                .map(|c| Object::Char(Char::new(c)))
                .collect()
        }
    }
}

impl From<i32> for List {

    // include markdown file as doc comment for this function
    #[doc = include_str!("../../docs/python_list/showcase.md")]
    fn from(_integer: i32) -> List {
        let mut vector_deque = VecDeque::new();
        vector_deque.push_back(Object::Int(Int::new(_integer)));
        List {
            _list: vector_deque,
        }
    }
}


impl From<&List> for List {
    /// creates a list from another list; like copy constructor
    fn from(_list: &List) -> List {
        let mut temp_list: VecDeque<Object> = VecDeque::new();
        for _object in &_list._list {
            // temp_list.push_back(_object);
        }
        List {
            _list: temp_list,
        }
    }
}



impl FromStr for List {
    type Err = Box<dyn error::Error>;

    fn from_str(_static_str: &str) -> Result<Self, Self::Err> {
        Ok(List::from(_static_str))
    }
}


impl FromIterator<i32> for List {
    fn from_iter<T: IntoIterator<Item = i32>>(
        _integer_iterator: T
    ) -> Self {
        let mut integer_list = List::new();
        for integer in _integer_iterator {
            integer_list.append_back(integer);
        }
        integer_list
    }
}


impl FromIterator<String> for List {
    fn from_iter<T: IntoIterator<Item = String>>(
        _string_iterator: T
    ) -> Self {
        let mut string_list = List::new();
        for _string in _string_iterator {
            string_list.append_back(_string);
        }
        string_list
    }
}
