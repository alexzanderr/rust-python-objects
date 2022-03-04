
use python::*;

use std::any::type_name;


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let float_32 = Float::from(123.123f32);
    print(float_32);

    let float_64 = Float::from(123.123f64);
    print(float_64);

    let float = Float::from(123.123f32);
    print(float);
    print(repr(&float));

    let mut float_list = List::new();
    float_list.append_back(float_32);
    float_list.append_back(float_64);
    float_list.append_back(float);
    print(float_list);

    println!("{:#?}", float);
    println!("{:?}", float);
    println!("{:#}", float);

    println!("{:#?}", float_64);
    println!("{:?}", float_64);
    println!("{:#}", float_64);

}