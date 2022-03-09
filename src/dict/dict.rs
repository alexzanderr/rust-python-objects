



use std::collections::HashMap;
use std::fmt;


use crate::Int;
// use crate::Float;
use crate::_String;
// use crate::Char;
use crate::Object;
use crate::_Object;
use crate::_Hashable;
use super::Hashable;
use super::SetItem;
use std::hash::Hash;
// use crate::_Object;


// TODO implement hash trait for object
/// Dict struct that represents the python dict
pub struct Dict<T: Sized + Eq + Hash + PartialEq> {
    /// hashmap of object and object
    _dict: HashMap<Hashable<T>, Object>,
}

impl<T> Dict<T>
where T: Sized + Eq + Hash + PartialEq
{
    /// creates a new empty python dict
    pub fn new() -> Dict<T> {
        Dict {
            _dict: HashMap::new(),
        }
    }
}

// impl<K, V> SetItem<K, V> for Dict
// where K: _Hashable, V: _Object + Sized
// {
//     fn set(&mut self, key: K, value: V) {
//         self._dict.insert(key, value);
//     }
// }

impl<T> SetItem<Int<T>, _String> for Dict<T>
where T: Sized + Eq + Hash + PartialEq
{
    fn set(&mut self, key: Int<T>, value: _String) {
        self._dict.insert(Hashable::Int(key), Object::String(value));
    }
}

// impl<T> Default for Dict<T>
// where T: Sized
// {
//     fn default() -> Self {
//         Dict::new()
//     }
// }

impl<T> fmt::Display for Dict<T>
where T: Sized + fmt::Display + Eq + Hash + PartialEq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{");
        for (key, value) in self._dict.iter() {
            write!(f, "    {}: {}", key, value);
        }
        writeln!(f, "\n}}")
    }
}



