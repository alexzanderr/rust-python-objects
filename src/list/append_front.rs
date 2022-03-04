

use crate::Object;
use crate::Int;
use crate::Float;
use crate::_String;
use crate::Char;
use crate::Bool;


use super::List;



/// appends to front of the list
/// meaning: list.insert(o, item)
pub trait AppendFront<T>: Sized {
    /// performs the append front
    fn append_front(&mut self, _: T) -> &mut Self;
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
        self._list.push_front(Object::Float32(Float::from(_float)));
        self
    }
}

impl AppendFront<f64> for List {
    fn append_front(&mut self, _float: f64) -> &mut Self {
        self._list.push_front(Object::Float64(Float::from(_float)));
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

    #[doc = include_str!("../../docs/python_list/append_pbool.md")]
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