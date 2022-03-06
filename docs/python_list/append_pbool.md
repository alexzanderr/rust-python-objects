

# append python bool: Bool struct

example
```rust
use python::*;
use pretty_assertions::assert_eq;

let python_bool = Bool::new(true);
let mut python_list = List::new();
python_list.append_back(python_bool);
print(python_list);
// assert_eq!(123, 1233);
```

output
```shell
[True]
```

