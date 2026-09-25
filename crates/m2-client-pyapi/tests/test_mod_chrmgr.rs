use m2_client_pyapi::mod_chrmgr::ModChrmgr;

#[test]
fn test_integration_mod_chrmgr() {
    let instance = ModChrmgr::new();
    assert!(instance.is_ready());
}
