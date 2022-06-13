use kat;

kat::kat_cfg!(tests / data / data);

kat::global! {
    name: String,
    value: usize
}

kat::test! {
    test_name: String,
    value: usize
}

kat::run! {
    |global, test| -> {
        assert_eq!(global.name, "GLOBAL");
        assert_eq!(global.value, 69);
        assert_eq!(test.test_name, "TEST");
        assert_eq!(test.value, 420);
    }
}
