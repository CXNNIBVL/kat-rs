//! Testing framework for known answer tests.
//! 
//! This crate aims to drastically reduce the boilerplate code
//! associated with rust tests, as well as to make tests easier
//! to write and extend.
//! 
//! This framework splits its implementation into the test implementation
//! and data, which is stored in .toml files.
//! 
//! ## Example
//! ### Toml file layout
//! The toml file must contain two sections, the global section
//! and the test section or sections.
//! ```no_run
//! [global] // In this section global variables are defined
//! my_global_var = "This is a global variable"
//! 
//! [[test]] // In these sections we define test cases
//! id = 0
//! data = "This is data for test 0"
//! 
//! [[test]] // Multiple tests can be defined like this
//! id = 1
//! data = "This is data for test 1"
//! ```
//! ### Writing the tests
//! Writing the tests is just as straight forward as writing the data
//! ```no_run
//! // In your test files global namespace (e.g tests/my_test.rs)
//! 
//! // This macro configures the filepath of your test file.
//! // The path will be interpreted, relative to the workspace root.
//! //
//! // In this case kat will look for the file:
//! // "WORKSPACE_ROOT/tests/data/my_data.toml"
//! //
//! // The file extension can be ommited, since we only support toml
//! kat_cfg!(tests/data/my_data);
//! 
//! // Now we define our global variables
//! global! {
//! 
//! // The name of the variable must be the same
//! // as the one defined in your data file.
//! // Kat internally uses serde to parse the key in
//! // your toml file as this variable.
//!  my_global_var: String
//! }
//! 
//! // We want to parse the data key
//! // in the test file, into this custom struct
//! struct MyData(String);
//! 
//! // Now we define our test specific variables
//! // The same conventions, as in the global! 
//! // macro do apply.
//! test! {
//!   id: usize,
//!   
//!   // To be able to parse the string,
//!   // we need to add a serde deserialize
//!   // attribute. See serde for more info on that.
//!   // Kat offers helper functions which can be used
//!   // in serdes deserialize_with attribute.
//!   #[serde(deserialize_with = "kat::from_string")]
//!   data: MyData
//! }
//! 
//! // Now we also need to implement From<String> for MyData,
//! // since the trait bounds on kat::from_string require us
//! // to do so
//! impl From<String> for MyData {
//!   fn from(s: String) -> Self {
//!     MyData(s)
//!   }
//! }
//! 
//! // And finally we provide the runner for our tests
//! run! {
//!     // The first two args of this macro are
//!     // the variable name for the global variables holder,
//!     // and the testcase variables holder.
//!     globals, test_case,
//! 
//!     // Now pass the statements you want to run
//!     println!("{}", global.my_global_var); // We can access the global variable, its behind a reference though
//!     println!("{}", test_case.id); // In similar fashion, the test case. Here you get owned access
//!     my_super_expensive_function(); // Any function can be called.
//!     mymod::my_function(); // Also from other modules
//!     let x = 25; // Any statement can be run!
//!  }
//! 
//! ```

pub mod de;

/// Configure the test files location.
/// 
/// Configures the location of the test file via a token stream.
/// The Path will be relative to the current workspaces cargo.toml.
/// Additionally, this macro will bring any dependencies required, into scope.
/// 
/// ### Invocation
/// ```no_run
/// // Where 'mydata' is a toml file inside the directory 'mydir',
/// // which itself is in the same directory as the top level
/// // cargo.toml
/// kat_cfg!(mydir/mydata)
/// ```
#[macro_export]
macro_rules! kat_cfg {
    ($path1: tt$(/$path2: tt)*) => {
        #[allow(dead_code)]
        const FILEPATH_SLICE: &'static [&'static str] = &[
            env!("CARGO_MANIFEST_DIR", "Cargo manifest directory environment variable is undefinded"),
            stringify!($path1),
            $(stringify!($path2),)*
        ];

        #[allow(unused_imports)]
        use serde as kat_serde;
        #[allow(unused_imports)]
        use toml as kat_toml;
        #[allow(unused_imports)]
        use kat_serde::{Deserialize};
    };
}

/// Defines the global variables inside the test file.
/// 
/// Variables are stored in a generated struct and 
/// can be accessed later in a test.
/// Variables should be declared like normal struct fields.
/// 
/// Since the library uses Serde for deserialization,
/// the fields can be annotated with serde's attributes.
/// 
/// ### Invocation
/// ```no_run
/// // Inside the test file
/// const _: &'static str = r#"
///     [global]
///     global1 = "Global 1"
///     global2 = 10
///     global3 = "Global De-Type to serialize from string"
/// "#;
/// 
/// //Invocation
/// global! {
///     pub global1: String,
///     pub global2: usize,
///     
///     #[serde(deserialize_with = "my_custom_de_function")]
///     pub global3: MyCustomType
/// }
/// 
/// ```
/// 
/// ### Invocation without global parameters
/// ```no_run
/// global!{}
/// ```
/// 
#[macro_export]
macro_rules! global {
    ($($data: tt)*) => {
        
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct KatGlobal {
            $($data)*
        }
    };
}

/// Defines the test specific variables inside the test file.
/// 
/// Variables are stored in a generated struct and 
/// can be accessed later in a test.
/// Variables should be declared like normal struct fields.
/// 
/// Since the library uses Serde for deserialization,
/// the fields can be annotated with serde's attributes.
/// 
/// ### Invocation
/// ```no_run
/// // Inside the test file
/// const _: &'static str = r#"
///     [global]
///     # ...
///     
///     [[test]]
///     value1 = 10,
///     value2 = "My Test Value"
///     value3 = "My Other Test Value"
///     
///     [[test]]
///     value1 = 20,
///     value2 = "My Test Value"
///     value3 = "My Other Test Value"
/// "#;
/// 
/// //Invocation
/// test! {
///     pub value1: usize,
///     pub value2: String,
///     
///     #[serde(deserialize_with = "my_custom_de_function")]
///     pub global3: MyCustomType
/// }
/// 
/// ```
/// 
#[macro_export]
macro_rules! test {
    ($($data: tt)*) => {
        
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct KatTest {
            $($data)*
        }
    };
}

/// Runs the tests.
/// 
/// This macro will create the module "kat_tests" and the test function "tests".
/// The macro will run any passed statements for every Test it could find. 
/// Inside the macro you get access to your global variables, as well as test specific
/// variables.
/// Depending on your IDE, you will see runnable highlighting above your macro invocation.
/// The macro depends on the invocation of `kat_cfg`, `global` and `test`.
/// 
/// ### Invocation
/// ```no_run
/// 
/// // Set the path to the test file
/// kat_cfg!(path/to/my/file);
/// 
/// // Define yout global variables
/// global!{
///     pub my_global: usize
/// }
/// 
/// // Define your test specific variables
/// test!{
///     pub my_test_var: usize
/// }
/// 
/// // Now run your tests
/// run! {
///     // First give a name to your variable, containing your globals
///     global_var,
///     
///     // Then give a name to your current testcase variable, containing
///     // specific test data
///     test_case,
/// 
///     // Now pass the statements you want to run
///     println!("{}", global_var.my_global); // We can access the global variable, its behind a reference tho
///     println!("{}", test_case.my_test_var); // In similar fashion, the test case. Here you get owned access
///     my_super_expensive_function(); // Any function can be called.
///     mymod::my_function(); // Also from other modules
///     let x = 25; // Any statement can be run!
/// }
/// 
/// ```
/// 
/// ## Panics
/// This macro will panic if:
/// * The file wasn't found, could not be opened or read
/// * Parsing from TOML was unsuccessful
#[macro_export]
macro_rules! run {
    (
        $global_data: ident, 
        $test_data: ident,
        $($data: tt)*
    ) => {
        
        #[cfg(test)]
        mod kat_tests {

            use super::*;

            #[derive(Deserialize)]
            struct KatFileLayout {
                global: KatGlobal,

                #[serde(rename = "test")]
                tests: Vec<KatTest>
            }

            pub fn read_file_as_string() -> Result<String, String> {
                use std::path::PathBuf;
                use std::fs::File;
                use std::io::Read;

                let mut filepath: PathBuf = FILEPATH_SLICE.iter().collect();
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
            fn tests() {

                let file_content = match read_file_as_string() {
                    Ok(content) => content,
                    Err(err) => { panic!("Error: {}", err) }
                };

                let (kat_global_data, kat_tests) = { 
                    let layout: KatFileLayout = match kat_toml::from_str(&file_content) {
                        Ok(k) => k,
                        Err(err) => { panic!("Unable to parse toml: {}", err.to_string()) }
                    };

                    (layout.global, layout.tests)
                };

                let $global_data = &kat_global_data;

                for $test_data in kat_tests {
                    { $($data)* }
                }
            }
        }
    };
}