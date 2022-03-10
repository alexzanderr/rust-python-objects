use python::*;

fn main() {
    let int = Int::new(123i32);
    let string = _String::from("salutare");

    let mut dict = Dict::new();
    dict.set(int, string);
    // dict.set(Int::new(65i64), _String::from("something"));

    print(dict);
}
