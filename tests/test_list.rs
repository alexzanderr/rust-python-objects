use pretty_assertions::assert_eq;
use rstest::rstest;

// crate
use python::*;


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
    python_list.append_back(123);
    python_list.append_back(123);
    python_list.append_back(123);
    python_list.append_back(123);
    python_list.append_back(123);
    print(&python_list);
    let result = len(&python_list);
    assert_eq!(result, 5);
    // incoming
    // assert_eq!(repr(&python_list), 5);
}


#[test]
fn test_append_bool() {
    let mut python_list = List::new();
    python_list.append_back(true);
    python_list.append_back(false);
    let result = len(&python_list);
    assert_eq!(result, 2)
}
