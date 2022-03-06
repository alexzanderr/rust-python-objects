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


use std::collections::HashMap;


use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


// use crate::Int;
// use crate::Float;
// use crate::SString;
// use crate::Char;
use crate::Object;
// use crate::_Object;


// TODO implement hash trait for object
/// Dict struct that represents the python dict
pub struct Dict {
    /// hashmap of object and object
    _dict: HashMap<Object, Object>,
}

impl Dict {
    /// creates a new empty python dict
    pub fn new() -> Dict {
        Dict {
            _dict: HashMap::new(),
        }
    }

    // fn set(&mut self, key: Object, value: Object) {
    //     self._dict.insert(key, value)
    // }
}

impl Default for Dict {
    fn default() -> Self {
        Dict::new()
    }
}

impl Display for Dict {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        for (key, value) in self._dict.iter() {
            write!(f, "{}: {}", key, value);
        }
        writeln!(f)
    }
}
