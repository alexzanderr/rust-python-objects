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
    unused_must_use
)]


use std::collections::VecDeque;

use std::fmt;

use std::error;

use std::ops::Add;
use std::ops::Deref;

use std::str::FromStr;


use crate::_Object;
use crate::Object;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;
use crate::type_of;
use crate::print;


use super::Append;
use crate::Iterable;

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

/// if you want to use max(&list) you need the impl for &List
/// how comes that for Object work by default
impl Iterable for List {
    fn __len__(&self) -> usize {
        self._list.len()
    }
}

// impl Iterable for &List {
//     fn __len__(&self) -> usize {
//         self._list.len()
//     }
// }


// TODO implement PartialEq for List
// impl PartialEq for List {
//     fn eq(&self, _List: &Self) -> bool {
//         for (o1, o2) in _List._list.iter().zip() {
//             if o1 == o2 {
//                 true
//             }
//         }
//         false
//     }

//     fn ne(&self, other: &Self) -> bool {
//         for (o1, o2) in _List._list.iter().zip() {
//             if o1 != o2 {
//                 true
//             }
//         }
//         false
//     }
// }


/// its implementation
impl List {
    /// new function
    pub fn new() -> List {
        List {
            _list: VecDeque::new(),
        }
    }

    /// check whether or not its a char or string type inside list
    /// designed and created for __repr__
    fn _contains_char_or_string(&self) -> bool {
        for _object in &self._list {
            match _object {
                Object::String(_object) => {
                    return true;
                },
                Object::Char(_object) => {
                    return true;
                },
                _ => {},
            }
        }
        false
    }
}


impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}



impl _Object for List {
    // >>>  x = ['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n',
    // ... 'g'], 'python string', True, False, False]
    // >>> x
    // ['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]
    // >>> repr(x)
    // "['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]"
    // >>> repr(["asd"])
    // "['asd']"
    // >>> repr(['asd'])
    // "['asd']"
    // >>> repr([123, 123])
    // '[123, 123]'

    /// rules for __repr__
    /// daca nu ai string or char -> '[]' (single quotes outsiode)
    /// daca ai string or char -> "[]" (double quotes outside)
    fn __repr__(&self) -> String {
        if self._contains_char_or_string() {
            format!("\"{}\"", self.__str__())
        } else {
            format!("'{}'", self.__str__())
        }
    }

    /// __str__ documentation in List
    fn __str__(&self) -> String {
        // if list is empty then we print '[]'
        if self._list.is_empty() {
            return String::from("[]");
        }


        let mut string_representation = String::new();
        let zero = self._list.get(0).unwrap();
        // print the first element of the list
        match zero {
            Object::String(_object) => {
                string_representation.push_str(
                    format!("[{}", _object.__repr__()).as_str(),
                );
            },
            Object::Char(_object) => {
                string_representation.push_str(
                    format!("[{}", _object.__repr__()).as_str(),
                );
            },
            _ => {
                string_representation
                    .push_str(format!("[{}", zero).as_str());
            },
        };

        // print the other elements of the list but from index = 1
        for _object in self._list.iter().skip(1) {
            match _object {
                Object::String(_object) => {
                    string_representation.push_str(
                        format!(", {}", _object.__repr__()).as_str(),
                    );
                },
                Object::Char(_object) => {
                    string_representation.push_str(
                        format!(", {}", _object.__repr__()).as_str(),
                    );
                },
                _ => {
                    string_representation
                        .push_str(format!(", {}", _object).as_str());
                },
            };
        }
        string_representation.push(']');
        string_representation
    }
}


impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // if list is empty then we print '[]'
        if self._list.is_empty() {
            write!(f, "[]");
            return Ok(());
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
                Object::String(obj) => {
                    write!(f, ", {}", obj.__repr__())
                },
                Object::Char(obj) => {
                    write!(f, ", {}", obj.__repr__())
                },
                _ => write!(f, ", {}", _object),
            };

            // ostream.push_str(&_object.fmt(f).unwrap());
        }
        write!(f, "]")
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // if list is empty then we print '[]'
        if self._list.is_empty() {
            write!(f, "[]");
            return Ok(());
        }


        let zero = self._list.get(0).unwrap();
        // print the first element of the list
        match zero {
            Object::String(obj) => write!(f, "[{:?}", obj.__repr__()),
            Object::Char(obj) => write!(f, "[{:?}", obj.__repr__()),
            _ => write!(f, "[{:?}", zero),
        };

        // print the other elements of the list but from index = 1
        for _object in self._list.iter().skip(1) {
            match _object {
                Object::String(obj) => {
                    write!(f, ", {}", obj.__repr__())
                },
                Object::Char(obj) => {
                    write!(f, ", {}", obj.__repr__())
                },
                _ => write!(f, ", {}", _object),
            };

            // ostream.push_str(&_object.fmt(f).unwrap());
        }
        write!(f, "]")
    }
}


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
            _list: _static_string
                .chars()
                .map(|c| Object::Char(Char::from(c)))
                .collect(),
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
            _list: _string
                .chars()
                .map(|c| Object::Char(Char::from(c)))
                .collect(),
        }
    }
}

impl From<String> for List {
    fn from(_string: String) -> List {
        List {
            _list: _string
                .chars()
                .map(|c| Object::Char(Char::from(c)))
                .collect(),
        }
    }
}

impl From<i32> for List {
    // include markdown file as doc comment for this function
    #[doc = include_str!("../../docs/python_list/showcase.md")]
    fn from(_integer: i32) -> List {
        let mut vector_deque = VecDeque::new();
        vector_deque.push_back(Object::Int32(Int::<i32>::new(_integer)));
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
            _list: temp_list
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
        _integer_iterator: T,
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
        _string_iterator: T,
    ) -> Self {
        let mut string_list = List::new();
        for _string in _string_iterator {
            string_list.append_back(_string);
        }
        string_list
    }
}


/// list concatenation
impl Add for List {
    type Output = List;

    /// list1 + list2 == list3
    fn add(self, rhs: Self) -> Self::Output {
        let mut new_list = List::from(self);
        for _object in rhs._list {
            new_list.append_back(_object);
        }
        new_list
    }
}
