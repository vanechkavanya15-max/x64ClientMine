use m2_client_granny::gr2_section::Gr2Section;

#[test]
fn test_integration_gr2_section() {
    let instance = Gr2Section::new();
    assert!(instance.is_ready());
}
