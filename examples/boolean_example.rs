use python::*;
use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;

fn main() {
    let b1 = Bool::new(true);
    let b2 = Bool::new(false);

    if b1 != b2 {
        println!("yes")
    } else {
        println!("no")
    }

    if b1 == true {
        println!("yes")
    } else {
        println!("no")
    }

    // not available yet
    // if b1 {
    //     println!("yes")
    // } else {
    //     println!("no")
    // }

    assert_ne!(b1, b2);
    assert_eq!(b1, true);
    assert_eq!(b2, false);
}
