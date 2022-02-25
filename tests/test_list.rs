

use pretty_assertions::assert_eq;

// crate
use python::print;
use python::List;
use python::len;
use python::repr;


#[test]
fn test_initialization() {
    let python_list = List::new();
    // i know that there is not actual test
    // but more things need to be implemented
    // like some default traits for class list, like Copy, Debug, Eq, PartialEq ...
    // in order to have assert_eq!()
}


#[test]
fn test_append_int() {
    let mut python_list = List::new();
    python_list.append_int(123);
    python_list.append_int(123);
    python_list.append_int(123);
    python_list.append_int(123);
    python_list.append_int(123);
    print(python_list);
}


#[test]
fn test_pretty() {
    assert_eq!(5, 5);
}

#[test]
fn test_len() {
    let mut python_list = List::new();
    python_list.append_bool(true);
    python_list.append_bool(false);
    let result = len(&python_list);
    assert_eq!(result, 2)
}
