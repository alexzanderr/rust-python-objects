
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
)]

use python::Int;
use python::print;
use python::printd;
use std::collections::HashMap;

fn main() {
    let int32: i32 = 123;
    let int64: i64 = 123;
    let int32 = Int::new(int32);
    let int64 = Int::new(int64);

    // note the constructor new is generic
    let int8 = Int::new(1i8);
    // and also the from method is generic
    let int8 = Int::from(1i8);
    let int16 = Int::from(1i16);
    print(int8);
    print(int16);

    printd(int8);

    print(int32);
    print(int64);

    let mut dict = HashMap::new();
    dict.insert(int32, int64);
    println!("{:?}", dict);


}