

/// implement iterable for List for example
/// i need this to use with max min builtins
pub trait Iterable {
    /// returns u size
    fn __iter__(&self) -> usize;
}