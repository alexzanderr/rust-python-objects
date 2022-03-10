#![allow(unused_imports)]

use python::*;


fn main() {
    let mut list = List::new();


    list.append_back("from str");
    list.append_back(String::from("from String"));
    list.append_back(List::from("extend from list"));

    list.append_back(123);
    list.append_back(123.123f32);
    list.append_back(123.123f64);


    print(list);
}
