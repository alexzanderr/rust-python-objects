use std::collections::VecDeque;
use std::fmt;
use std::any::type_name;


/// get the type of an object
pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

/// the supreme _Object trait
/// that its derived types should
/// implement like all the __functions__ from python
trait _Object {
    /// python len(object)
    fn __len__(&self) -> usize;
    fn __str__(&self) -> String;
}

trait Append<T>: Sized {
    fn append_back(&mut self, _: T) -> &mut Self;
}

#[derive(Copy)]
#[derive(Clone)]
/// Float struct that handles f32 and f64
pub struct Float<T: Sized> {
    // this can be f32 or f64
    _float: T,
}


impl<T> Float<T>
where
    T: Sized,
{
    /// constructor
    /// creates a Float object from any float (f32, f64)
    pub fn new(_float: T) -> Self {
        Float {
            _float,
        }
    }
}

impl<T> From<T> for Float<T>
where
    T: Sized,
{
    fn from(_float: T) -> Self {
        Float {
            _float,
        }
    }
}


impl<T> _Object for Float<T>
where
    T: Sized + fmt::Display,
{
    #[inline]
    fn __len__(&self) -> usize {
        8
    }

    fn __str__(&self) -> String {
        format!("{}", self._float)
    }
}


impl<T> fmt::Display for Float<T>
where
    T: Sized + fmt::Display,
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

struct List<T>
where
    T: _Object, {
    /// _list which holds all the python objects together
    pub _list: VecDeque<T>,
}

impl<T> List<T>
where
    T: _Object,
{
    pub fn new() -> Self {
        let _list = VecDeque::new();
        List {
            _list,
        }
    }
}


impl<T> _Object for List<T>
where
    T: _Object + fmt::Display,
{
    #[inline]
    fn __len__(&self) -> usize {
        self._list.len()
    }

    fn __str__(&self) -> String {
        format!("not implemented")
    }
}

impl<T> Append<T> for List<T>
where
    T: _Object,
{
    fn append_back(&mut self, _object: T) -> &mut Self {
        self._list.push_back(_object);
        self
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

    let mut lista = List::new();
    lista.append_back(Float::new(123f32));
    // lista.append_back(Float::new(123f64));
}
