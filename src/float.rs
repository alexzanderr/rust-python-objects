


use std::{fmt::{self, Display}};
use unindent::{unindent, Unindent};

use crate::_Object;
use crate::type_of;

#[derive(Copy)]
#[derive(Clone)]
pub struct Float<T: Sized> {
    _float: T
}

// impl<T> Float<T>
// where T: Sized
// {
//     pub fn new() -> Float<T> {
//         Float {
//             _float: 0.0f32
//         }
//     }
// }

// impl Float<f64>
// {
//     pub fn new() -> Float<f64> {
//         Float {
//             _float: 0.0f64
//         }
//     }
// }

impl From<f32> for Float<f32> {
    fn from(_float: f32) -> Self {
        Float {
            _float
        }
    }
}

impl From<f64> for Float<f64> {
    fn from(_float: f64) -> Self {
        Float {
            _float
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


impl fmt::Display for Float<f32> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _type = type_of(&self._float);
        if formatter.alternate() {
            write!(formatter, "{} -> <{}>", self.__str__(), _type)
        } else {
            write!(formatter, "{}", self.__str__())
        }
    }
}

impl fmt::Display for Float<f64> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _type = type_of(&self._float);
        if formatter.alternate() {
            write!(formatter, "{} -> <{}>", self.__str__(), _type)
        } else {
            write!(formatter, "{}", self.__str__())
        }
    }
}







// https://doc.rust-lang.org/std/fmt/struct.Formatter.html
// TODO make something like rich inspect with colors and stuff
impl<T> fmt::Debug for Float<T>
where T: Sized + Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.alternate
        let _type = type_of(&self._float);

        if f.alternate() {
            let _fmt = format!("Float<{}> {{
                _float: {}
            }}", _type, self._float);
            let _fmt = _fmt.unindent();
            write!(f,  "{}", _fmt)
        } else {
            write!(f, "Float<{}> {{ _float: {} }}", _type, self._float)
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