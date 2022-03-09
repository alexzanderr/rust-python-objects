

use crate::{_Object, Append, Int};

/// extend the current list with anything
pub trait Extend<T>: Sized + _Object {

    #[doc = include_str!("../../docs/python_list/extend.md")]
    fn extend(&mut self, _: T) -> &mut Self;
}


use super::List;


impl Extend<List> for List {
    fn extend(&mut self, _container: List) -> &mut Self {
        for _object in _container._list {
            self.append_back(_object);
        }
        self
    }
}

impl Extend<&str> for List {
    fn extend(&mut self, _str: &str) -> &mut Self {
        for _char in _str.chars() {
            self.append_back(_char);
        }
        self
    }
}

impl Extend<String> for List {
    fn extend(&mut self, _string: String) -> &mut Self {
        for _char in _string.chars() {
            self.append_back(_char);
        }
        self
    }
}


impl<T> Extend<Vec<T>> for List
where T: Sized, List: Append<T>
{
    fn extend(&mut self, _vec: Vec<T>) -> &mut Self {
        for _item in _vec {
            self.append_back(_item);
        }
        self
    }
}


impl Extend<i32> for List {
    fn extend(&mut self, _int: i32) -> &mut Self {
        self.append_back(_int);
        self
    }
}


impl Extend<Int<i32>> for List {
    fn extend(&mut self, _int: Int<i32>) -> &mut Self {
        self.append_back(_int);
        self
    }
}


// too hard
// impl<T> Extend<&[T]> for List
// where T: Sized, List: Append<T>
// {
//     fn extend(&mut self, _vec: &[T]) -> &mut Self {
//         for _item in _vec {
//             self.append_back(_item);
//         }
//         self
//     }
// }

