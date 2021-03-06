use crate::Object;
use crate::Int;
use crate::Float;
// use crate::_Object;
use crate::_String;
use crate::Char;
use crate::Bool;


use super::List;


/// append function for List
/// can append any data type
pub trait Append<T>: Sized {
    /// Performs append
    fn append_back(&mut self, _: T) -> &mut Self;
}


/// inline append for integer
/// example
///
/// let mut one_elem = List::new();
/// one_elem
///     .append_back(123)
///     .append_back(123)
///     .append_back(123)
///     .append_back(123)
///     .append_back(123)
///     .append_back(123)
///     .append_back(123);
/// println!("{}", one_elem);
///
/// [123, 123, 123, 123, 123, 123, 123]
impl Append<i32> for List {
    fn append_back(&mut self, _integer: i32) -> &mut Self {
        self._list.push_back(Object::Int32(Int::new(_integer)));
        self
    }
}

impl Append<i64> for List {
    fn append_back(&mut self, _integer: i64) -> &mut Self {
        self._list.push_back(Object::Int64(Int::new(_integer)));
        self
    }
}


impl Append<Int<i32>> for List {
    fn append_back(&mut self, _integer: Int<i32>) -> &mut Self {
        self._list.push_back(Object::Int32(_integer));
        self
    }
}

impl Append<Int<i64>> for List {
    fn append_back(&mut self, _integer: Int<i64>) -> &mut Self {
        self._list.push_back(Object::Int64(_integer));
        self
    }
}

// impl Append<Vec<i32>> for List {
//     fn append_back(&mut self, _int_vec: Vec<i32>) -> &mut Self {
//         for _int in _int_vec {
//             self._list.push_back(Object::Int32(Int::from(_int)));
//         }
//         self
//     }
// }


// impl<T> Append<T> for List
// where T: Sized {
//     fn append_back(&mut self, _integer: i32) -> &mut Self {
//         self._list.push_back(Object::Int32(Int::new(_integer)));
//         self
//     }
// }


impl Append<Float<f32>> for List {
    fn append_back(&mut self, _float: Float<f32>) -> &mut Self {
        self._list.push_back(Object::Float32(_float));
        self
    }
}

impl Append<Float<f64>> for List {
    fn append_back(&mut self, _float: Float<f64>) -> &mut Self {
        self._list.push_back(Object::Float64(_float));
        self
    }
}

impl Append<f64> for List {
    fn append_back(&mut self, _float: f64) -> &mut Self {
        self._list.push_back(Object::Float64(Float::new(_float)));
        self
    }
}


impl Append<&str> for List {
    fn append_back(&mut self, _str: &str) -> &mut Self {
        self._list.push_back(Object::String(_String::from(_str)));
        self
    }
}


impl Append<char> for List {
    fn append_back(&mut self, _char: char) -> &mut Self {
        self._list.push_back(Object::Char(Char::from(_char)));
        self
    }
}

impl Append<f32> for List {
    fn append_back(&mut self, _float: f32) -> &mut Self {
        self._list.push_back(Object::Float32(Float::from(_float)));
        self
    }
}


impl Append<String> for List {
    fn append_back(&mut self, string: String) -> &mut Self {
        self._list.push_back(Object::String(_String::from(string)));
        self
    }
}

impl Append<_String> for List {
    fn append_back(&mut self, _string: _String) -> &mut Self {
        self._list.push_back(Object::String(_string));
        self
    }
}

impl Append<bool> for List {
    fn append_back(&mut self, _bool: bool) -> &mut Self {
        self._list.push_back(Object::Bool(Bool::new(_bool)));
        self
    }
}


impl Append<Bool> for List {
    #[doc = include_str!("../../docs/python_list/append_pbool.md")]
    fn append_back(&mut self, _bool: Bool) -> &mut Self {
        self._list.push_back(Object::Bool(_bool));
        self
    }
}

impl Append<List> for List {
    fn append_back(&mut self, _list: List) -> &mut Self {
        self._list.push_back(Object::List(_list));
        self
    }
}

impl Append<Object> for List {
    fn append_back(&mut self, object: Object) -> &mut Self {
        self._list.push_back(object);
        self
    }
}
