use kat;
use kat::DeriveTable;

#[derive(DeriveTable)]
struct CustomTable {
    name: String,
    value: usize,
}

kat::kat_cfg!(tests / data / data_add_table_section);

kat::global! {
    global_value: String,
    table: CustomTable
}

kat::test! {
    test_value: String,
    table: CustomTable
}

kat::run! {
    |global, test| -> {
        assert_eq!(global.global_value, "GLOBAL");
        assert_eq!(global.table.name, "GLOBAL TABLE");
        assert_eq!(global.table.value, 69);

        assert_eq!(test.test_value, "TEST");
        assert_eq!(test.table.name, "TEST TABLE");
        assert_eq!(test.table.value, 69);
    }
}
