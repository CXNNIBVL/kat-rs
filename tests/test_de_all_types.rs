use kat::*;

kat_cfg!(tests/data/data_de_all_types);

test! {
    v: usize,
}

struct DateString(String);
impl_deserialize_from_toml_date!(
    |date| -> DateString {
        DateString(date.to_string())
    }
);

struct BoolHolder(bool);
impl_deserialize_from_toml_bool!(
    |b| -> BoolHolder {
        BoolHolder(b)
    }
);

struct FloatHolder(f32);
impl_deserialize_from_toml_float!(
    |f| -> FloatHolder {
        FloatHolder(f as f32)
    }
);

struct IntHolder(i64);
impl_deserialize_from_toml_int!(
    |i| -> IntHolder {
        IntHolder(i)
    }
);

struct StringHolder(String);
impl_deserialize_from_toml_string!(
    |s| -> StringHolder {
        StringHolder(s)
    }
);

struct Deserializable(StringHolder);
impl_deserialize_from_deserializable!(
    |sh: StringHolder| -> Deserializable {
        Deserializable(sh)
    }
);

#[derive(DeriveTable)]
struct Table {
    value: usize
}

#[derive(DeriveTable)]
struct InlineTable {
    value: String,
}

global! {
    string: types::TomlString,
    int: types::TomlInt,
    float: types::TomlFloat,
    toml_bool: types::TomlBool,
    date: types::TomlDate,
    toml_array: types::TomlArray<usize>,
    from_date: DateString,
    from_bool: BoolHolder,
    from_float: FloatHolder,
    from_int: IntHolder,
    from_string: StringHolder,
    from_de: Deserializable,
    table: Table,
    inline_table: InlineTable
}

run! {
    |global, test| -> {

        assert_eq!(global.string, "STRING");
        assert_eq!(global.int, 69);
        assert_eq!(global.float, 3.1415);
        assert_eq!(global.toml_bool, true);
        assert_eq!(global.date.to_string(), "1979-05-27T07:32:00");
        assert_eq!(global.toml_array.array, [0,1,2,3]);
        assert_eq!(global.from_date.0, "1979-05-27T07:32:00");
        assert_eq!(global.from_bool.0, false);
        assert_eq!(global.from_float.0, 69.69);
        assert_eq!(global.from_int.0, 187);
        assert_eq!(global.from_string.0, "STRING HOLDER");
        assert_eq!(global.from_de.0.0, "DYN");
        assert_eq!(global.table.value, 33);
        assert_eq!(global.inline_table.value, "33er");

        assert_eq!(test.v, 420);
    }
}