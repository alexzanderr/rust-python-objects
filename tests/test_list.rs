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

use pretty_assertions::assert_eq;
use rstest::rstest;

// crate
use python::*;


#[test]
fn test_initialization() {
    let python_list = List::new();
    print(type_of(&python_list));
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

#[test]
fn test_extend() {
    let mut python_list = List::new();
    python_list.extend(123i32);
    python_list.extend(List::from("from list"));

    // from vec i32
    python_list.extend(vec![123, 123, 123]);

    let result = len(&python_list);
    assert_eq!(result, 1 + "from list".len() + 3)
}


#[test]
fn test_repr() {
    let mut python_list = List::new();
    python_list.extend(123);
    python_list.extend(123);
    let result = repr(&python_list);

    assert_eq!(result, "'[123, 123]'");

    let mut contains_strings = List::new();
    contains_strings.append_back("rust");
    contains_strings.append_back("is");
    contains_strings.append_back("great");

    let result = repr(&contains_strings);

    assert_eq!(result, "\"['rust', 'is', 'great']\"");
}

#[test]
fn test_str() {
    let mut python_list = List::new();
    python_list.extend(123);
    python_list.extend(123);
    let result = _str(&python_list);

    assert_eq!(result, "[123, 123]");

    let mut contains_strings = List::new();
    contains_strings.append_back("rust");
    contains_strings.append_back("is");
    contains_strings.append_back("great");

    let result = _str(&contains_strings);

    assert_eq!(result, "['rust', 'is', 'great']");
}
