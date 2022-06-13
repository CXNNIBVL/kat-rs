pub use serde::{
    Deserialize as __Deserialize, 
    Deserializer as __Deserializer, 
    de::DeserializeOwned as __DeserializeOwned,
};

/// private. should not be used directly
pub use serde_derive::Deserialize as __DeriveDeserialize;

/// Type to generate a table from a struct
pub use __DeriveDeserialize as DeriveTable;

/// Deserializable types
pub mod types {
    use super::*;
    /// Type to deserialize a toml String from
    pub type TomlString = String;

    /// Type to deserialize a toml Int from
    pub type TomlInt = i64;

    /// Type to deserialize a toml Float from
    pub type TomlFloat = f64;

    /// Type to deserialize a toml Bool from
    pub type TomlBool = bool;

    /// Type to deserialize a toml Datetime from
    pub type TomlDate = toml::value::Datetime;

    /// Type to deserialize a single value-type toml Array from
    pub struct TomlArray<T: __DeserializeOwned> {
        pub array: Vec<T>
    }

    impl<'de, T> __Deserialize<'de> for TomlArray<T>
        where T: __DeserializeOwned
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: __Deserializer<'de>,
        {
            let f = Vec::<T>::deserialize(deserializer)?;
            Ok(Self { array: f })
        }
    }

}

/// private. should not be used directly
#[macro_export]
macro_rules! __generate_impl_deserialize {
    ($var_name_from: ident, $from_ty: ty, $for_ty: ty, $($body: tt)*) => {
        impl From<$from_ty> for $for_ty {
            fn from($var_name_from: $from_ty) -> Self {
                { $($body)* }
            }
        }

        impl<'de> kat::__Deserialize<'de> for $for_ty {
            fn deserialize<D>(deserializer: D) -> Result<$for_ty, D::Error>
            where
                D: kat::__Deserializer<'de>,
            {
                let f = <$from_ty>::deserialize(deserializer)?;
                Ok(<$for_ty>::from(f))
            }
        }
    };
}

/// Generate Deserialize trait for any type that is
/// constructable from a type that implements Serde's Deserialize
/// trait
#[macro_export]
macro_rules! impl_deserialize_from_deserializable {
    (
        |$var_name_from: ident: $from_ty: ty| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, $from_ty, $for_ty, $($body)*); };
}

/// Generate Deserialize trait for any type that is
/// constructable from a TomlString
#[macro_export]
macro_rules! impl_deserialize_from_toml_string {
    (
        |$var_name_from: ident| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, kat::types::TomlString, $for_ty, $($body)*); };
}

/// Generate Deserialize trait for any type that is
/// constructable from a TomlInt
#[macro_export]
macro_rules! impl_deserialize_from_toml_int {
    (
        |$var_name_from: ident| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, kat::types::TomlInt, $for_ty, $($body)*); };
}

/// Generate Deserialize trait for any type that is
/// constructable from a TomlFloat
#[macro_export]
macro_rules! impl_deserialize_from_toml_float {
    (
        |$var_name_from: ident| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, kat::types::TomlFloat, $for_ty, $($body)*); };
}

/// Generate Deserialize trait for any type that is
/// constructable from a TomlBool
#[macro_export]
macro_rules! impl_deserialize_from_toml_bool {
    (
        |$var_name_from: ident| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, kat::types::TomlBool, $for_ty, $($body)*); };
}

/// Generate Deserialize trait for any type that is
/// constructable from a TomlDate
#[macro_export]
macro_rules! impl_deserialize_from_toml_date {
    (
        |$var_name_from: ident| -> $for_ty: ty {
            $($body: tt)*
        }
    ) => { kat::__generate_impl_deserialize!($var_name_from, kat::types::TomlDate, $for_ty, $($body)*); };
}
