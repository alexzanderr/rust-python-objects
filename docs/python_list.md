

# Showcase - python list
```rust
// the crate name is 'python-objects'
// because there is another crate out there with `python` name
// but the lib.rs (library crate of this crate) its called `python`
// so you can import like this
extern crate python;

// use everything from python
use python::*;

fn main() {
    // create a new python list from a string
    let mut python_list =
        List::from_string(String::from("123123"));
    // since rust doesnt have function overloading
    // we are stuck with different names
    // but i think its better because its more explicit
    // append some values
    python_list.append_int(123);
    python_list.append_float(123.123);
    python_list.append_float(123.123);
    python_list.append_float(123.123);
    python_list.append_string(String::from("asdasd"));
    // append a rust string
    python_list.append_list(
        List::from_string("rust's string".to_string()));
    // append a python string
    python_list.append_pstring(
        _String::from_string(
            String::from("python string")));

    // note the python-like print
    // print to stdout
    print(&python_list);
    // and len
    // print length of list
    print(len(&python_list));
}
```

output
```shell
# the python list
['1', '2', '3', '1', '2', '3', 123, 123.123, 123.123, 123.123, 'asdasd', ['a', 'n', 'd', 'r', 'e', 'w'], 'python string']
# the length
13
```

as you can see the list contains `char`, `float (f32)`, `integer (i32)`, `a rust string`, `another python list`, and a `python string`

this is just `bare bones` and `experimental`, more features will come soon. stay still!