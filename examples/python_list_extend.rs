
#![allow(unused_imports)]

use python::*;

fn main() {
    let mut list = List::new();

    let from_vec = vec![123i32, 123, 123, 123];

    list.extend("from str");
    list.extend(String::from("from String"));
    list.extend(List::from("extend from list"));

    list.extend(from_vec);
    list.extend(vec![123i64, 123, 123, 123]);

    print(&list);
    printd(list);
}