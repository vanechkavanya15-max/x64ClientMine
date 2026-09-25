use m2_client_granny::gr2_morph_targets::Gr2MorphTargets;

#[test]
fn test_integration_gr2_morph_targets() {
    let instance = Gr2MorphTargets::new();
    assert!(instance.is_ready());
}
