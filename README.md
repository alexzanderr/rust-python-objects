
![logo](https://github.com/alexzanderr/rust-python-objects/blob/main/static/img/logo/rust-python-objects-logo.png?raw=True)

<p align="center">
    <a href="https://crates.io/crates/python-objects">
        <img src="https://img.shields.io/crates/v/python-objects.svg" alt="Crates.io">
    </a>
    <a href="https://choosealicense.com/licenses/mit/" alt="License: MIT">
        <img src="https://img.shields.io/badge/license-MIT-green.svg" />
    </a>
</p>



# Rust Python Objects
use python like objects in rust, such as `list`, for now.

have the `simplicity` and `performance` together.

be happy.


# Table of Contents
- [Table of Contents](#table-of-contents)
    - [Showcases](#showcases)
    - [Install](#install)
    - [Documentation](#documentation)
    - [TODO](#todo)
    - [Contributing](#contributing)
    - [Changelog](#changelog)
    - [NOTE](#note)


# Showcases
first you need to see the showcases to convince you to install it.

1. [`python-list`](https://github.com/alexzanderr/rust-python-objects/blob/main/docs/python_list/showcase.md)
2. [`python-dict`] -> incoming

# Install
`1. the old way`

just copy the crate name and the version you want to use:
```toml
python-objects = "0.0.5"
```
to your `Cargo.toml` and then write some code and build it.

`2. the modern and simple way`

just run this command
```shell
cargo add python-objects
```
and this will add the `latest version` from `crates.io` to your `Cargo.toml`, just like the old way, but automatically.

- what is `cargo add` ? -> its a cargo sub command
- how to install it ? -> `cargo install cargo-edit`
- what is [`cargo-edit`](https://github.com/killercup/cargo-edit) ? -> its a rust package that adds useful sub commands for cargo that are not `built-in`

# Documentation
- [`docs.rs/python-objects`](https://docs.rs/python-objects/latest/python/)
- [`the-book`](https://alexzanderr.github.io/rust-python-objects/index.html)

# why ?
why not. because python concepts are very cool, but python is slow..

so what if we could use python objects in rust and have the same experience as in python (almost) ?

and things are getting much better. rust has `zero-cost abstractions`, meaning that every struct and trait will be deleted at `compile time` and they will be gone at `runtime`.

so we are getting python objects in rust with `performance` and `simplity`.


# TODO
check [`TODO.md`](https://github.com/alexzanderr/rust-python-objects/blob/main/TODO.md)


# Contributing
check [`CONTRIBUTING.md`](https://github.com/alexzanderr/rust-python-objects/blob/main/CONTRIBUTING.md
)

# Changelog
check [`CHANGELOG.md`](https://github.com/alexzanderr/rust-python-objects/blob/main/changelog/CHANGELOG.md)

# Crates that use Python-Objects
incoming, would be nice.

# NOTE
note for the user and developer

if you find `issues` go ahead and make an
[`issue`](https://github.com/alexzanderr/rust-python-objects/issues/new)
or a
[`pull request`](https://github.com/alexzanderr/rust-python-objects/compare),
cant wait to take a look into them.


right now this project is very minimal, it will grow, the idea just came to my mind some days ago (on 21.02.2022) and the first implementation was done at 4 AM (23.02.2022) with some adrenaline.

so for now dont expect too much of this crate

peace to you!