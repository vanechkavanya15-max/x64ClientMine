use m2_client_granny::gr2_string_table::Gr2StringTable;

#[test]
fn test_integration_gr2_string_table() {
    let instance = Gr2StringTable::new();
    assert!(instance.is_ready());
}
