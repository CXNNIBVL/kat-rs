use kat::{
    global, impl_deserialize_from_toml_int, impl_deserialize_from_toml_string, kat_cfg, run, test,
    impl_deserialize_from_deserializable
};

struct IntData(usize);

impl_deserialize_from_toml_int!(|i| -> IntData { IntData(i as usize) });

struct StringData(pub String);

impl_deserialize_from_toml_string!(|s| -> StringData { StringData(s) });

struct CustomFromDeserialize(String);

impl_deserialize_from_deserializable!(
    |string: StringData| -> CustomFromDeserialize { 
        CustomFromDeserialize(string.0) 
    }
);


kat_cfg!(tests / data / data_custom_de_struct);
global! {
    int_data: IntData
}

test! {
    string_data: CustomFromDeserialize
}

run! {
    |global, test| -> {
        assert_eq!(global.int_data.0, 22);
        assert_eq!(test.string_data.0, "DATA");
    }
}
