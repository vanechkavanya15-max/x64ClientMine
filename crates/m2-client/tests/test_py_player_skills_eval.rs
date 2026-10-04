use m2_client::py_bridge_player_skills_eval::PlayerSkillEvaluator;

#[test]
fn test_player_can_use_skill_evaluation() {
    let mut eval = PlayerSkillEvaluator::new();
    eval.player_level = 10;
    eval.current_sp = 50;
    eval.has_weapon_equipped = true;
    eval.is_stunned = false;

    // Powinien moc uzyc skilla 1 (koszt 30 SP, min lvl 5)
    assert!(eval.can_use_skill(1));
    assert_eq!(eval.get_skill_need_sp(1), 30);

    // Po uzyciu: SP maleje, cooltime aktywny
    eval.set_skill_used(1);
    assert_eq!(eval.current_sp, 20);
    assert!(!eval.can_use_skill(1)); // cooltime blokuje

    // Skill 4 wymaga 50 SP, a zostalo 20 -> brak SP
    assert!(!eval.can_use_skill(4));

    // Stun blokuje wszystko
    eval.is_stunned = true;
    assert!(!eval.can_use_skill(1));
    assert!(!eval.can_use_skill(4));
}
