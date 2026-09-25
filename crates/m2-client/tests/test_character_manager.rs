use m2_client::character_manager::CharacterManager;

#[test]
fn test_integration_character_manager() {
    let instance = CharacterManager::new();
    assert!(instance.is_ready());
}
