use std::collections::HashMap;

#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Key {
    _key: i32,
}

impl std::fmt::Debug for Key {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self._key)
    }
}

struct Value {
    _value: i32,
}

impl std::fmt::Debug for Value {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self._value)
    }
}

fn main() {
    let key = Key {
        _key: 123
    };
    let val = Value {
        _value: 5990
    };
    let mut _dict: HashMap<Key, Value> = HashMap::new();
    _dict.insert(key, val);
    println!("{:?}", _dict);
}
