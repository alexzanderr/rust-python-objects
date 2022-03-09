

use crate::_Hashable;
use crate::_Object;

pub trait SetItem<K, V>
where K: _Hashable, V: _Object + Sized
{
    fn set(&mut self, key: K, value: V);
}