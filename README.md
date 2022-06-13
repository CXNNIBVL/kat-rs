# kat-rs
Testing framework for known answer tests.

This crate aims to drastically reduce the boilerplate code
associated with rust tests, as well as to make known-answer tests easier
to write and extend.

This framework splits the tests into the test implementation
and data, which is stored in .toml files.

Under the hood, Kat uses [Serde](https://docs.rs/serde/latest/serde/index.html)
and [Toml-rs](https://docs.rs/toml/latest/toml/) to deserialize test data.

## kat-rs in action
### Toml file layout
File `WORKSPACE_ROOT/tests/data/my_data.toml`
```toml
# In this section global variables are defined.
[global]
my_global_var = "This is a global variable"
my_custom_string_holder = "String Holder"

# In these sections we define test cases.
# Each test owns its own data. 
# Though every test must have the same
# data signature
[[test]]
id = 0
data = "This is data for test 0"
input = 24.8
expected = 24.8

# Multiple tests can be defined with
# consecutive "test" tables
[[test]]
id = 1
data = "This is data for test 1"
input = 3.1415
expected = 3.1415
```

### Writing the tests
```rust
use kat::{
    types,
    kat_cfg,
    global,
    test,
    run,
};

// Some custom struct
struct CustomStringHolder {
    s: String,
    v: usize
}

impl_deserialize_from_toml_string!(
    |s| -> CustomStringHolder {
        CustomStringHolder {
            s, 
            v: 12
        }
    }
); 

// Path configuration relative to WORKSPACE_ROOT
kat_cfg(tests/data/my_data.toml);

// Define global variables
global! {
  my_global_var: types::TomlString,
  my_custom_string_holder: CustomStringHolder
}

// Define Test variables
test! {
    id: types::TomlInt,
    data = types::TomlString,
    input = types::TomlFloat
    expected = types::TomlFloat
}

// Implement Test Runner
run! {
  #[should_panic(expected = "x, y, not equal")]  
  |global, test| -> {
    
    println!("{}", global.my_global_var);
    println!("{}", global.my_custom_string_holder.s);
    assert_eq!(test.input, test.expected);

    let (x, y) = (10, 15);
    assert_eq!(x, y, "x, y, not equal");

  }

}
