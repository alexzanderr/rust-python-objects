
use crate::_Object;

/// implement iterable for List for example
/// i need this to use with max min builtins
pub trait Iterable: _Object {
    /// return the length of the collection/container/iterable
    fn __len__(&self) -> usize;
}
