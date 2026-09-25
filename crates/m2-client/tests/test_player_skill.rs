use m2_client::player_skill::PlayerSkill;

#[test]
fn test_integration_player_skill() {
    let instance = PlayerSkill::new();
    assert!(instance.is_ready());
}
