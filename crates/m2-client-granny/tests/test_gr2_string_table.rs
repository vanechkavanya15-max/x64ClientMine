use m2_client_granny::gr2_string_table::Gr2StringTable;

#[test]
fn test_integration_gr2_string_table() {
    let instance = Gr2StringTable::new();
    assert!(!instance.is_ready());
}

#[test]
fn test_valid_string() {
    let mut data = b"hello".to_vec();
    data.push(0);
    data.extend_from_slice(b"world");
    data.push(0);

    let table = Gr2StringTable::from_buffer(data);
    assert!(table.is_ready());
    assert_eq!(table.get_string(0), Some("hello"));
    assert_eq!(table.get_string(6), Some("world"));
}

#[test]
fn test_invalid_offset() {
    let mut data = b"test".to_vec();
    data.push(0);
    
    let table = Gr2StringTable::from_buffer(data);
    assert_eq!(table.get_string(10), None);
}

#[test]
fn test_missing_null() {
    let data = b"test".to_vec();
    let table = Gr2StringTable::from_buffer(data);
    assert_eq!(table.get_string(0), None);
}

#[test]
fn test_invalid_utf8() {
    let data = vec![0xff, 0xff, 0];
    let table = Gr2StringTable::from_buffer(data);
    assert_eq!(table.get_string(0), None);
}
