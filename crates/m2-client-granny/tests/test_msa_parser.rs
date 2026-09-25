use m2_client_granny::msa_parser::MsaParser;

#[test]
fn test_integration_msa_parser() {
    let instance = MsaParser::new();
    assert!(instance.is_ready());
}
