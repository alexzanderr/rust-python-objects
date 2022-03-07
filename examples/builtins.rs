
use python::*;

fn main() {

    let mut l = List::new();
    l.append_back(123);
    l.append_back(123);

    let l_repr = repr(&l);
    let l_str = _str(&l);
    let l_len = len(&l);
    let l_max = max(&l);

    print(l_repr);
    print(l_str);
    print(l_len);
    print(l_max);
}