//! Testing framework for known answer tests.
//!
//! This crate aims to drastically reduce the boilerplate code
//! associated with rust tests, as well as to make known-answer tests easier
//! to write and extend.
//!
//! This framework splits the tests into the test implementation
//! and data, which is stored in .toml files.
//! 
//! Under the hood, Kat uses [Serde](https://docs.rs/serde/latest/serde/index.html)
//! and [Toml-rs](https://docs.rs/toml/latest/toml/) to deserialize test data.
//! Both need to be added as dependencies to your crate.
//!
//! ## Getting Started
//! ## Toml file layout
//! The toml file must contain two sections, the **global section**
//! and the **test section** (or sections).
//! ```no_run
//! // In this section global variables are defined.
//! [global]
//! my_global_var = "This is a global variable"
//!
//! // In these sections we define test cases.
//! // Each test owns its own data. 
//! // Though every test must have the same
//! // data signature
//! [[test]]
//! id = 0  // int
//! data = "This is data for test 0" // string
//! input = "INPUT" // string
//! expected = "INPUT" // string
//! 
//! // Multiple tests can be defined with
//! // consecutive "test" tables
//! [[test]]
//! id = 1  // int
//! data = "This is data for test 1" // string
//! input = "INPUT" // string
//! expected = "INPUT" // string
//! ```
//! If you'd like a comprehensive list of types, that you can
//! include in your toml file, then visit the 
//! [Toml Website](https://toml.io/en/v1.0.0)
//! 
//! ## Writing the tests
//! Writing the tests is just as straight forward as writing the data.
//! This tutorial will go step by step, in order of definition, and is based
//! on the earlier demonstrated toml file layout.
//! 
//! Import the kat crate
//! ---
//! This can be done, either in your test files global namespace
//! (e.g tests/my_test.rs), or in a submodule 
//! (e.g tests/my_test.rs::my_submodule).
//! ```no_run
//! // Import Kat
//! use kat::*;
//!```
//! Configure the test file path
//! ---
//! The [kat_cfg] macro configures the filepath of your
//! test file. The path will be interpreted, relative to the
//! workspace root. The file extension can be ommited, since we only support toml.
//! String quotes around the path are not needed, since kat will
//! directly interpolate the path from the macro expression.
//! 
//! ```no_run
//! // "WORKSPACE_ROOT/tests/data/my_data.toml"
//! kat_cfg!(tests/data/my_data);
//! ```
//! Global and Test variables
//! ---
//! Now we define the layout for our global and test variables.
//! Define the variables, just like you would in a normal
//! Rust struct.
//! 
//! Since Kat, internally uses Serde to deserialize the variables,
//! every type in [global] and [test] must derive Deserialize.
//! 
//! More to deserialization of types, in the
//! [Deserializing Types section](./#deserializing-types)
//! 
//! The [global] and [test] macros will generate structs 
//! which will later be parsed as the test files content.
//! ```no_run
//! // Define global variables
//! global! {
//! // The name of the variable must match
//! // the one defined in your data file.
//!  my_global_var: String
//! }
//!
//! // Define our test specific variables.
//! // The same conventions, as in the global!
//! // macro apply.
//! test! {
//!   id: usize,
//!   data: String,
//!   input: String,
//!   expected: String,
//! }
//!```
//! Running the tests
//! ---
//! And finally we provide the runner for our tests.
//! 
//! Depending on your IDE, you can see 
//! a "Run tests" hint (VS Code for example).
//! 
//! The tests will be run in the module 
//! "YOUR_MODULE::kat_tests", and the main test function
//! is simply called "tests".
//! 
//! Inside the [run] macro, you get access to your global and
//! test variables, inside the here named variables `globals` and `test_case`.
//! Both can be named like you would any other variable.
//! 
//! On top of that you can execute any statements inside the macro.
//! Though, mutating `globals` and `test_case` is not possible, since
//! they're internally defined as immutable aka read-only.
//! 
//! ```no_run
//! // Test Runner
//! run! {
//!     // Test Runner
//!     //
//!     // Note the lambda like invocation syntax.
//!     // It's specified in the macro as a match, for
//!     // easier readability and familiarity. 
//!     |globals, test_case| -> {
//!
//!         // Now pass the statements you want to run
//! 
//!         // We can access the global variable.
//!         println!("{}", global.my_global_var);
//!         
//!         // In similar fashion, the test case.
//!         println!("{}", test_case.id);
//!         
//!         // Any statements can be executed
//! 
//!         // Assertions
//!         assert_eq!(test_case.input, test_case.expected);    
//! 
//!         // Function call which is defined somewhere...
//!         my_super_expensive_function();
//!         
//!         // Also from other modules
//!         mymod::my_function();
//! 
//!         // Variables
//!         let x = 25;
//! 
//!         // Macros
//!         my_crate::some_macro!();
//!     }
//!  }
//!
//! ```
//! ### Panics
//! The runner panics, if the test file wasn't found,
//! an IO Error occured (e.g File open unsuccessful),
//! or if toml parsing was erroneous.
//! 
//! ---
//! All in all, we end up with a structure like this:
//! ```no_run
//! // Path configuration
//! kat_cfg(...);
//! 
//! // Define global variables
//! global! {
//!   ...
//! }
//! 
//! // Define Test variables
//! test! {
//!  ...
//! }
//! 
//! // Implement Test Runner
//! run! {
//!   |global, test| -> {
//!     ...
//!   }
//! 
//! }
//! ```
//! Runner attributes
//! ---
//! As per usual rust tests, you can annotate the [run] macro with
//! [test attributes](https://doc.rust-lang.org/reference/attributes/testing.html). 
//! The initial `#[test]` attribute is already being added for you internally.
//! ```no_run
//! // Ignore tests
//! run! {
//!   #[ignore = "not yet implemented"]
//!   |global, test| -> {
//!      ...  
//!   }   
//! }
//! ```
//! ```no_run
//! // Should panic
//! //
//! // Note, that when running the tests from
//! // the 'run' hint in your IDE, the test will
//! // still be logged as fail. The test will
//! // only accept the panic, when run with
//! // "Cargo test" 
//! run! {
//!   #[should_panic(expected = "values don't match")]
//!   |global, test| -> {
//!      assert_eq!(1, 2, "values don't match");
//!   }   
//! }
//! ```
//! Type Attributes
//! ---
//! Kat supports type attributes for both, types defined
//! in the [global] and [test] macro.
//! ```no_run
//! // Global macro as an example
//! global! {
//!   my_type: String,
//!   
//!   #[my_attribute]
//!   my_attributed_type: usize
//! }
//! ```
//! Deserializing Types
//! ---
//! ### Common Types
//! Kat provides the major toml types in its [types] module.
//! However, Kat does not support deserialization of multi-type
//! arrays. For this case it is encouraged to deserialize an array
//! of tables.
//! ```no_run
//! use kat::{types, DeriveTable};
//! 
//! // Kat provides a "DeriveTable" attribute,
//! // which actually is an alias for Serde's 
//! // Deserialize proc-macro.
//! //
//! // This is how you define a table
//! #[derive(DeriveTable)]
//! struct MyTable {
//!     value: types::TomlInt
//! }
//! 
//! global! {
//!     toml_string: types::TomlString,
//!     toml_int: types::TomlInt,
//!     toml_float: types::TomlFloat,
//!     toml_date: types::TomlDate,
//!     toml_bool: types::TomlBool,
//!     toml_int_array: types::TomlArray<types::TomlInt>,
//!     toml_table: MyTable,
//! }
//! 
//! ...
//! ```
//! The test file would look something like this:
//! ```no_run
//! [global]
//! toml_string = "Toml String"
//! toml_int = 10
//! toml_float = 3.1415
//! toml_date = 1979-05-27
//! toml_bool = true
//! toml_int_array = [1, 2, 3, 4, 5]
//! [global.toml_table]
//! value = 22
//! 
//! ...
//! ```
//! 
//! ### Deserializing Custom Types
//! Since Kat internally deserializes its types with the help of Serde and Toml-rs,
//! primitive types like `String` or `usize` can be parsed directly from toml, without
//! any macro magic, because Serde or Toml-rs provide internal deserialization implementations.
//! So technically you could deserialize custom types with serde attributes.
//! ```no_run
//! // Your custom type
//! struct StringHolder(String);
//! impl From<String> for StringHolder {
//!    fn from(s: String) -> Self {
//!         Self(s)
//!    }
//! }
//! 
//! // Generic String deserializer
//! // Deserialize a [T] if it's String constructable
//! fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
//!     where 
//!         D: Deserializer<'de>,
//!         T: From<String>
//! {
//!     let s = String::deserialize(deserializer)?;
//!     Ok(T::from(s))
//! }
//! 
//! global! {
//!     // Use Serde attribute
//!     #[serde(deserialize_with = "deserialize_from_string")]
//!     string_holder: StringHolder
//! }
//! 
//! ...
//! ```
//! However, this results in a lot of boilerplate code.
//! 
//! Luckily, Kat provides you with streamlined ways, in which you can
//! focus on the From implementation, and let Kat handle the code generation:
//! ### Deserialize Custom Types: The Kat way
//! Kat provides macros that generate the code needed to deserialize your value.
//! ```no_run
//! struct StringHolder(String);
//! 
//! // Note again the lambda syntax for
//! // familiarity and readability
//! impl_deserialize_from_toml_string!(
//!      |s| -> StringHolder {
//!        StringHolder(s)
//!      }       
//! );
//! 
//! // Now use it
//! global! {
//!     string_holder: StringHolder
//! }
//! ```
//! Inside Toml file
//! ```no_run
//! [global]
//! string_holder = "Hey Ho!"
//! 
//! ```
//! Here, `s` denotes the variable name for the passed 
//! [TomlString](types::TomlString), you can name it whatever
//! you wish for. Then follows an arrow with the type, the code is
//! to be generated for, here `StringHolder`. And finally the function
//! body. 
//! 
//! The function body, is essentially the body of the
//! `impl From<TomlString> for StringHolder` implementation,
//! this macro generates. The macro also generates a deserialize
//! implementation.
//! 
//! Macros like this exist for all types in the [types] module, but Table and Array.
//! For these two, you will need to call the [impl_deserialize_from_deserializable] macro.
//! 
//! The [impl_deserialize_from_deserializable] macro can deserialize a custom type
//! from any type that implements Serde's Deserialize trait.
//! ```no_run
//! 
//! #[derive(DeriveTable)]
//! struct MyTable {
//!     value: TomlInt
//! }
//! 
//! struct MyTableHolder(MyTable)
//! 
//! // Denote the input type being typed.
//! // As stated earlier, this macro
//! // generates `impls` from any type that
//! // is deserializable, so it needs the type
//! // annotation.
//! // The rest stays exactly the same.
//! // Note, that MyTableHolder doesn't
//! // have to be a tuple, this still is
//! // simply a From<T> implementation
//! impl_deserialize_from_deserializable!(
//!     |table: MyTable| -> MyTableHolder { 
//!         MyTableHolder(table)
//!     }      
//! );
//! 
//! // From Array
//! struct MyArrayHolder(TomlArray<usize>);
//! impl_deserialize_from_deserializable!(
//!     |array: TomlArray<usize>| -> MyArrayHolder {
//!         MyArrayHolder(array)
//!     }
//! );
//! ```
//! With this macro, it's also possible to chain your custom types.
//! ```no_run
//! // MyArrayHolder from previous example
//! 
//! struct HoldsArrayHolder(MyArrayHolder);
//! impl_deserialize_from_deserializable!(
//!     |holder: MyArrayHolder| -> HoldsArrayHolder {
//!         HoldsArrayHolder(holder)
//!     }
//! );
//! 
//! ```
//! This is possible, since the macro generated 
//! the code for the Deserialize trait for `MyArrayHolder`
//! 
//! ## Final Notes
//! It is discouraged to rename the crate, since many macros
//! inside the crate use the `kat::` module namespace 
//! in order to directly depent on a type, thus not cluttering the global namespace.
//! 
//! On top of that, many exported traits and macros use the `__XXX` prefix.
//! These items typically abstract the code generation away, thus, are private.
//! They should **not** be used directly.

mod de;
pub use de::*;

/// Configure the test files location.
#[macro_export]
macro_rules! kat_cfg {
    ($path1: tt$(/$path2: tt)*) => {
        #[allow(dead_code)]
        const __FILEPATH_SLICE: &'static [&'static str] = &[
            env!("CARGO_MANIFEST_DIR", "Cargo manifest directory environment variable is undefinded"),
            stringify!($path1),
            $(stringify!($path2),)*
        ];
    };
}

/// Defines the global variables inside the test file.
#[macro_export]
macro_rules! global {
    ($($data: tt)*) => {

        #[derive(kat::DeriveTable)]
        struct __KatGlobal {
            $($data)*
        }
    };
}

/// Defines the test specific variables inside the test file.
#[macro_export]
macro_rules! test {
    ($($data: tt)*) => {

        #[derive(kat::DeriveTable)]
        struct __KatTest {
            $($data)*
        }
    };
}

/// Runs the tests.
#[macro_export]
macro_rules! run {
    (
        $(#[$attr:meta])*
        |$global_data: ident, $test_data: ident| -> {
            $($body: tt)*
        }
    ) => {
        #[cfg(test)]
        mod kat_tests {

            use super::*;

            #[derive(kat::DeriveTable)]
            struct __KatFileLayout {
                global: __KatGlobal,

                #[serde(rename = "test")]
                tests: Vec<__KatTest>
            }

            pub fn __read_file_as_string() -> Result<String, String> {
                use std::path::PathBuf;
                use std::fs::File;
                use std::io::Read;

                let mut filepath: PathBuf = __FILEPATH_SLICE.iter().collect();
                filepath.set_extension("toml");

                if !filepath.is_file() {
                    return Err(format!(
                        "{} is not a file, or wasn't found",
                        filepath.display(),
                    ))
                }

                let mut file = match File::open(&filepath) {
                    Ok(file) => file,
                    Err(err) => return Err(format!(
                        "File {} was found, but could not be opened: {}",
                        filepath.display(), err.to_string()
                    ))
                };

                let mut content = String::new();

                if let Err(err) = file.read_to_string(&mut content) {
                    return Err(format!(
                        "File {} was found, but could not be read: {}",
                        filepath.display(), err.to_string()
                    ))
                }

                Ok(content)
            }

            #[test]
            $(#[$attr])*
            fn tests() {

                let file_content = match __read_file_as_string() {
                    Ok(content) => content,
                    Err(err) => { panic!("Error: {}", err) }
                };

                let ($global_data, kat_tests) = {
                    let layout: __KatFileLayout = match toml::from_str(&file_content) {
                        Ok(k) => k,
                        Err(err) => { panic!("Unable to parse toml: {}", err.to_string()) }
                    };

                    (layout.global, layout.tests)
                };

                kat_tests.into_iter()
                .for_each(|$test_data|{
                    { $($body)* }
                });
            }
        }
    };
}