
use std::fmt;
use std::any::type_name;

/// get the type of an object
pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

/// the supreme _Object trait
/// that its derived types should
/// implement like all the __functions__ from python
pub trait _Object<T> {
    /// python repr(object)
    fn __repr__(&self) -> String;
    /// python len(object)
    fn __len__(&self) -> usize;
    /// python str(object)
    fn __str__(&self) -> String;
}

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



impl<T> _Object<T> for Float<T>
where T: Sized + fmt::Display
{
    #[inline]
    fn __repr__(&self) -> String {
        // you need fmt::Display for this
        format!("{}", self._float)
    }

    #[inline]
    fn __len__(&self) -> usize {
        8
    }

    #[inline]
    fn __str__(&self) -> String {
        // and for this; fmt::Display
        format!("{}", self._float)
    }
}




impl<T> fmt::Display for Float<T>
where T: Sized + fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _type = type_of(&self._float);
        if formatter.alternate() {
            write!(formatter, "{} -> <{}>", self.__str__(), _type)
        } else {
            write!(formatter, "{}", self.__str__())
        }
    }
}


/// print(object);
pub fn print<T: fmt::Display>(arg: T) {
    println!("{}", arg);
}

fn main() {
    let _floater = Float::new(123f32);
    let floater = Float::from(123f32);
    print(floater);
}
