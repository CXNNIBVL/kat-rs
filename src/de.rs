//! Helper functions to use in serde deserialize_with field attribute
//! 
//! Implementing a custom Serde-Deserializer for every custom type is
//! quite tedious. This module contains helper functions with which toml 
//! values can be deserialized.
//! 
//! ## Example
//! ```no_run
//! struct MyCustomStruct(u8);
//! 
//! // Implement your from type
//! impl From<i64> for MyCustomStruct {
//!     fn from(v: i64) -> Self {
//!        MyCustomStruct(v as u8)
//!     }      
//! }
//! 
//! kat_cfg(...);
//! 
//! globals! {
//!  ...
//! }
//! 
//! // Now in test macro (also applies for global macro)
//! test! {
//!    #[serde(deserialize_with = "kat::from_i64")]
//!     my_struct: MyCustomStruct
//! }
//! 
//! run! {
//!  ...
//! }
//! ```

use serde::{Deserialize, Deserializer};
use toml::value::{Value, Datetime, Array, Table};

/// Deserialize T from a string
pub fn from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<String>
{
    let s = String::deserialize(deserializer)?;
    Ok(T::from(s))
}

/// Deserialize T from an integer
pub fn from_i64<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<i64>
{
    let i = i64::deserialize(deserializer)?;
    Ok(T::from(i))
}

/// Deserialize T from a float
pub fn from_f64<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<f64>
{
    let f = f64::deserialize(deserializer)?;
    Ok(T::from(f))
}

/// Deserialize T from a boolean
pub fn from_bool<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<bool>
{
    let b = bool::deserialize(deserializer)?;
    Ok(T::from(b))
}

/// Deserialize T from a toml datetime
pub fn from_datetime<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<Datetime>
{
    let d = Datetime::deserialize(deserializer)?;
    Ok(T::from(d))
}

/// Deserialize T from a toml array
pub fn from_array<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<Array>
{
    let a = Array::deserialize(deserializer)?;
    Ok(T::from(a))
}

/// Deserialize T from a toml table
pub fn from_table<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<Table>
{
    let t = Table::deserialize(deserializer)?;
    Ok(T::from(t))
}

/// Deserialize T from a toml value
pub fn from_value<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where 
        D: Deserializer<'de>, 
        T: From<Value>
{
    let v = Value::deserialize(deserializer)?;
    Ok(T::from(v))
}