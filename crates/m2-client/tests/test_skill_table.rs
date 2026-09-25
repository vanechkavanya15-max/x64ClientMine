use m2_client::skill_table::SkillTable;

#[test]
fn test_integration_skill_table() {
    let instance = SkillTable::new();
    assert!(instance.is_ready());
}
