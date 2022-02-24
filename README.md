
![logo](https://github.com/alexzanderr/rust-python-objects/blob/main/static/img/logo/rust-python-objects-logo.png?raw=True)

<p align="center">
    <a href="https://choosealicense.com/licenses/mit/" alt="License: MIT">
        <img src="https://img.shields.io/badge/license-MIT-green.svg" />
    </a>
</p>



# Rust Python Objects
use python like objects in rust, such as `list`, for now.

have the `simplicity` and `performance` together.

be happy.

# Showcase (example code)
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

# why ?
why not. because python concepts are very cool, but python is slow..

so what if we could use python objects in rust and have the same experience as in python (almost) ?

and things are getting much better. rust has `zero-cost abstractions`, meaning that every struct and trait will be deleted at `compile time` and they will be gone at `runtime`.

so we are getting python objects in rust with `performance` and `simplity`.


# TODO
check [`TODO.md`](https://github.com/alexzanderr/rust-python-objects/blob/main/TODO.md)




# CONTRIBUTING

1. fork the repo on github

2. clone your repo to local
```shell
git clone https://github.com/<your-username>/rust-python-objects
```

3. make your branch
```shell
git checkout -b <username>-<branch-name>
```
note: the branch name can be whatever you want, but would be more suggestive to be a username and feature name


4. make some changes in the source code

5. add + commit + push changes to your forked repo

add
```shell
git add .
```

commit
```shell
git commit -m 'created the best feature every. feels so good to contribute to open source'
```

push
```shell
git push github <your-branch-name>
```

6. go to github to your forked repo and send a pull request

7. wait for approval :)


# NOTE

note for the user and developer

right now this project is very minimal, it will grow, the idea just came to my mind some days ago (on 21.02.2022) and the first implementation was done at 4 AM (23.02.2022) with some adrenaline.

so for now dont expect too much of this crate

peace to you!