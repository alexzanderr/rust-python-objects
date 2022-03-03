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
use std::collections::vec_deque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;

use crate::_Object;
use crate::Object;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;


// mod object;
// use object::Object;

/// append function for List
/// can append any data type
pub trait Append<T>: Sized {
    /// Performs append
    fn append(&mut self, _: T) -> &mut Self;
}

/// appends to front of the list
/// meaning: list.insert(o, item)
pub trait AppendFront<T>: Sized {
    /// performs the append front
    fn append_front(&mut self, _: T) -> &mut Self;
}

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
impl Append<i32> for List {
    fn append(&mut self, _integer: i32) -> &mut Self {
        self._list.push_back(Object::Int(Int::new(_integer)));
        self
    }
}

impl Append<char> for List {
    fn append(&mut self, _char: char) -> &mut Self {
        self._list.push_back(Object::Char(Char::new(_char)));
        self
    }
}

impl Append<f32> for List {
    fn append(&mut self, _float: f32) -> &mut Self {
        self._list.push_back(Object::Float(Float::new(_float)));
        self
    }
}


impl Append<String> for List {
    fn append(&mut self, string: String) -> &mut Self {
        self._list.push_back(Object::String(_String::from_string(string)));
        self
    }
}

impl Append<_String> for List {
    fn append(&mut self, _string: _String) -> &mut Self {
        self._list.push_back(Object::String(_string));
        self
    }
}

impl Append<bool> for List {
    fn append(&mut self, _bool: bool) ->&mut Self {
        self._list.push_back(Object::Bool(Bool::new(_bool)));
        self
    }
}


impl Append<Bool> for List {

    #[doc = include_str!("../docs/python_list/append_pbool.md")]
    fn append(&mut self, _bool: Bool) ->&mut Self {
        self._list.push_back(Object::Bool(_bool));
        self
    }
}

impl Append<List> for List {
    fn append(&mut self, _list: List) -> &mut Self {
        self._list.push_back(Object::List(_list));
        self
    }
}




impl AppendFront<i32> for List {
    fn append_front(&mut self, _integer: i32) -> &mut Self {
        self._list.push_front(Object::Int(Int::new(_integer)));
        self
    }
}


impl AppendFront<&str> for List {
    fn append_front(
        &mut self,
        _static_string: &str
    ) -> &mut Self {
        self._list.push_front(Object::String(_String::from_str(_static_string)));
        self
    }
}

impl AppendFront<char> for List {
    fn append_front(&mut self, _char: char) -> &mut Self {
        self._list.push_front(Object::Char(Char::new(_char)));
        self
    }
}

impl AppendFront<f32> for List {
    fn append_front(&mut self, _float: f32) -> &mut Self {
        self._list.push_front(Object::Float(Float::new(_float)));
        self
    }
}


impl AppendFront<String> for List {
    fn append_front(&mut self, string: String) -> &mut Self {
        self._list.push_front(Object::String(_String::from_string(string)));
        self
    }
}

impl AppendFront<_String> for List {
    fn append_front(&mut self, _string: _String) -> &mut Self {
        self._list.push_front(Object::String(_string));
        self
    }
}

impl AppendFront<bool> for List {
    fn append_front(&mut self, _bool: bool) ->&mut Self {
        self._list.push_front(Object::Bool(Bool::new(_bool)));
        self
    }
}


impl AppendFront<Bool> for List {

    #[doc = include_str!("../docs/python_list/append_pbool.md")]
    fn append_front(&mut self, _bool: Bool) ->&mut Self {
        self._list.push_front(Object::Bool(_bool));
        self
    }
}

impl AppendFront<List> for List {
    fn append_front(&mut self, _list: List) -> &mut Self {
        self._list.push_front(Object::List(_list));
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

    fn __str__(&self) -> String {
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
    #[doc = include_str!("../docs/python_list/showcase.md")]
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
