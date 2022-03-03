

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use
)]

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use python::List;
use python::Object;
use python::Int;
use python::Float;
use python::Bool;
use python::_String;

use python::list::Append;
use python::list::AppendFront;
// builtins.rs
use python::print;
use python::len;
use python::repr;


fn main() {
    // example of what i want

    let mut python_list =
        List::from(String::from("123123"));

    python_list.append(123);

    python_list.append(123);

    python_list.append(List::from(List::from(String::from("working"))));

    python_list.append(123.123);
    python_list.append(123.123);
    python_list.append(123.123);
    python_list.append(String::from("asdasd"));

    python_list.append(
        List::from("andrew".to_string()));
    python_list.append(
        _String::from_string(
            String::from("python string")));

    python_list.append(Bool::new(true));
    python_list.append(Bool::new(false));
    python_list.append(false);

    print(&python_list);
    print(len(&python_list));


    python_list.append_front("salutare");

    for o in python_list.iter() {
        print(o)
    }



    // let oo = Object::Int(Int::new(123));
    // println!("{}", oo);

    // let oo = Object::Float(Float::new(123.123));
    // println!("{}", oo);


    // let empty_list = List::new();
    // println!("{}", empty_list);

    // // i would like to to this inline stuff
    // // let one_elem = List::new().append_int(123);
    // let mut one_elem = List::new();
    // one_elem
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123);

    // print(&one_elem);
    // print(len(one_elem));

    // let mut one_elem = List::new();
    // one_elem
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123);

    // print(repr(one_elem));

    // let python_string = _String::from_string(String::from("asd"));

    // print(python_string);

    // let list_from_integer = List::from(123);
    // // [1, 2, 3]
    // let list_from_string = List::from(String::from("abc"));
    // // ['a', 'b', 'c']
    // let list_from_static_string = List::from("asd");
    // // ['a', 's', 'd']

    // let python_list = List::new();
    // python_list.append(123);
    // python_list.append(123.123);
    // python_list.append(List::new());
    // python_list.append("static string");
    // python_list.append("String object".to_string());

    // python_list.extend(List::from(123));

    // print(python_list);
    // print(repr(python_list));

    // let integer = Int { value: 123i32};
    // let floater = Float { value: 123.123f32};


    // println!("{}", len(&integer));
    // println!("{}", len(integer));
}
