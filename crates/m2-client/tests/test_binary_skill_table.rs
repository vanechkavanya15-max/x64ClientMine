use m2_client::binary_skill_table_loader::{
    BinarySkillTableLoader, SkillGradeLevel,
};

#[test]
fn test_skill_table_default_loading() {
    let loader = BinarySkillTableLoader::new();
    assert!(loader.len() >= 2);

    let tripple = loader.get_skill(1).expect("Tripple Slash musi istniec");
    assert_eq!(tripple.name, "Tripple Slash");
    assert_eq!(tripple.splash_range, 250);
    assert_eq!(tripple.calculate_sp_cost(1), 33);
    assert_eq!(tripple.calculate_sp_cost(20), 90);

    // Stopnie zaawansowania
    assert_eq!(tripple.get_grade(10), SkillGradeLevel::Normal);
    assert_eq!(tripple.get_grade(17), SkillGradeLevel::Master);
    assert_eq!(tripple.get_grade(30), SkillGradeLevel::GrandMaster);
    assert_eq!(tripple.get_grade(40), SkillGradeLevel::PerfectMaster);

    let aura = loader.get_skill(4).expect("Aura musi istniec");
    assert_eq!(aura.name, "Aura of the Sword");
    assert_eq!(aura.calculate_cooltime(1), 60.0);
}
