use m2_client_granny::motlist_parser::MotlistParser;

#[test]
fn test_integration_motlist_parser() {
    let instance = MotlistParser::new();
    assert!(instance.is_ready());
}
