

# python list extend method

```rust
#![allow(unused_imports)]

use python::*;

fn main() {
    let mut list = List::new();

    let from_vec = vec![123i32, 123, 123, 123];

    list.extend("from str");
    list.extend(String::from("from String"));
    list.extend(List::from("extend from list"));

    list.extend(from_vec);
    list.extend(vec![123i64, 123, 123, 123]);

    // extend with i32, in this case, same as append
    list.extend(123i32);


    print(&list);
    printd(list);
}
```

output
```shell
['f', 'r', 'o', 'm', ' ', 's', 't', 'r', 'f', 'r', 'o', 'm', ' ', 'S', 't', 'r', 'i', 'n', 'g', 'e', 'x', 't', 'e', 'n', 'd', ' ', 'f', 'r', 'o', 'm', ' ', 'l', 'i', 's', 't', 123, 123, 123, 123, 123, 123, 123, 123]
["'f'", 'r', 'o', 'm', ' ', 's', 't', 'r', 'f', 'r', 'o', 'm', ' ', 'S', 't', 'r', 'i', 'n', 'g', 'e', 'x', 't', 'e', 'n', 'd', ' ', 'f', 'r', 'o', 'm', ' ', 'l', 'i', 's', 't', 123, 123, 123, 123, 123, 123, 123, 123]
```

`"'f'"` i know, there are some bugs, but im working on it