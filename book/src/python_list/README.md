

# Showcase - python list
```rust
// the crate name is 'python-objects'
// because there is another crate out there with `python` name
// but the lib.rs (library crate of this crate) its called `python`
// so you can import like this
extern crate python;
// actually 'extern crate' is useless
// just use only 'use python::'

// use everything from python
use python::*;

fn main() {
    // create a new python list
    let mut python_list =
        List::from(String::from("123123"));
    // at this point the list will look like this
    // ['1', '2', '3', '1', '2', '3']

    // append an integer
    python_list.append_back(123);
    // append a rust static string
    python_list.append_front("hello");
    python_list.append_back(123);
    // append a list
    python_list.append_back(List::from(String::from("working")));
    // note that the python list supports another python list inside

    // append a float
    python_list.append_back(123.123);
    python_list.append_back(123.123);
    python_list.append_back(123.123);
    // append a rust String
    python_list.append_back(String::from("asdasd"));

    python_list.append_back(
        List::from("something".to_string()));

    // append a python string
    // note that this _String is from this crate
    // its the struct that handles the String and &str data types
    python_list.append_back(
        _String::from(
            String::from("python string")));

    // append a python bool
    // note that Bool is the python struct that handles rust's bool
    python_list.append_back(Bool::new(true));
    python_list.append_back(Bool::new(false));
    // append a rust bool
    python_list.append_back(false);

    // print just like in python
    print(&python_list);

    // use len just like in python
    print(len(&python_list));


    // python_list.append_front("salutare");

    // iterate over the list just like in python
    // there are plans for future to remove the .iter()
    // so you can use for o in python_list { ... }, just that simple
    for o in python_list.iter() {
        print(o)
    }

    // create a python from parsing a static string
    let list_from_str = "123123".parse::<List>().unwrap();
    print(&list_from_str);


    let iter = (0..5).into_iter();
    // let list_from_iterator: List = iter.collect();

    // create a python list from rust iterator
    let list_from_iterator = iter.collect::<List>();
    print(&list_from_iterator);
}
```

output
```shell
# the list
['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]
# the length
19
hello
1
2
3
1
2
3
123
123
['w', 'o', 'r', 'k', 'i', 'n', 'g']
123.123
123.123
123.123
asdasd
['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g']
python string
True
False
False
['1', '2', '3', '1', '2', '3']
[0, 1, 2, 3, 4]
```

as you can see the list contains `char`, `float (f32)`, `integer (i32)`, `a rust string`, `another python list`, a `python string` and many more data types.

this is just `bare bones` and `experimental`, more features will come soon. stay still!