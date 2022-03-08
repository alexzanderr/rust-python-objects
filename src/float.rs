#![allow(unused_imports)]

use std::fmt;

use unindent::{
    unindent,
    Unindent,
};

use crate::_Object;
use crate::type_of;

#[derive(Copy)]
#[derive(Clone)]
/// Float struct that handles f32 and f64
pub struct Float<T: Sized> {
    // this can be f32 or f64
    _float: T,
}


impl<T> Float<T>
where T: Sized
{
    /// constructor
    /// creates a Float object from any float (f32, f64)
    pub fn new(_float: T) -> Self {
        Float {
            _float
        }
    }
}

impl<T> From<T> for Float<T>
where T: Sized
{
    fn from(_float: T) -> Self {
        Float {
            _float,
        }
    }
}



impl Default for Float<f32> {
    fn default() -> Self {
        Float {
            _float: 0.0f32
        }
    }
}

impl Default for Float<f64> {
    fn default() -> Self {
        Float {
            _float: 0.0f64
        }
    }
}


impl _Object for Float<f32> {
    fn __repr__(&self) -> String {
        format!("{}", self._float)
    }

    fn __len__(&self) -> usize {
        self._float as usize
    }

    fn __str__(&self) -> String {
        format!("{}", self._float)
    }
}

impl _Object for Float<f64> {
    fn __repr__(&self) -> String {
        format!("{}", self._float)
    }

    fn __len__(&self) -> usize {
        self._float as usize
    }

    fn __str__(&self) -> String {
        format!("{}", self._float)
    }
}


/// T cannot be formatted with the default formatter
/// thats why there are implementations for every float possible
impl<T> fmt::Display for Float<T>
where T: Sized + fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _type = type_of(&self._float);
        if formatter.alternate() {
            write!(formatter, "{} -> <{}>", self._float, _type)
        } else {
            write!(formatter, "{}", self._float)
        }
    }
}


// https://doc.rust-lang.org/std/fmt/struct.Formatter.html
// TODO make something like rich inspect with colors and stuff
impl<T> fmt::Debug for Float<T>
where
    T: Sized + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.alternate
        let _type = type_of(&self._float);

        if f.alternate() {
            let _fmt = format!(
                "Float<{}> {{
                _float: {}
            }}",
                _type, self._float
            );
            let _fmt = _fmt.unindent();
            write!(f, "{}", _fmt)
        } else {
            write!(
                f,
                "Float<{}> {{ _float: {} }}",
                _type, self._float
            )
        }
    }
}

// impl fmt::Debug for Float<f64> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.alternate
//         let _type = "f64";

//         if f.alternate() {
//             let _fmt = format!("Float<{}> {{
//                 _float: {}
//             }}", _type, self._float);
//             let _fmt = _fmt.unindent();
//             write!(f,  "{}", _fmt)
//         } else {
//             write!(f, "Float<{}> {{ _float: {} }}", _type, self._float)
//         }
//     }
// }
