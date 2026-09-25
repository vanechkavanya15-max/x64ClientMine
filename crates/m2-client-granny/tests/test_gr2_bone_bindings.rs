use m2_client_granny::gr2_bone_bindings::Gr2BoneBindings;

#[test]
fn test_integration_gr2_bone_bindings() {
    let instance = Gr2BoneBindings::new();
    assert!(instance.is_ready());
}
