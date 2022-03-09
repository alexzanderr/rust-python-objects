
use python::*;


fn main() {
    let some1 = List::from("123");
    let some2 = List::from(123);
    let some3 = some1 + some2;
    print(some3);

    // error, they are consumed
    // print(some1);
    // print(some2);
}