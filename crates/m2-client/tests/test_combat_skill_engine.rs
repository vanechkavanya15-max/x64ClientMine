#[path = "../src/combat_skill_engine.rs"]
mod combat_skill_engine;

use combat_skill_engine::*;

#[test]
fn test_state_machine_transitions() {
    let engine_ptr = m2_skill_engine_new(100, 0); // 0 casting speed
    let mut pkt = TPacketCGUseSkill {
        header: 0,
        length: 0,
        dw_vnum: 0,
        dw_target_vid: 0,
    };
    
    let can_use = m2_combat_skill_can_use(engine_ptr, 1, 10, 0.0);
    assert!(can_use);
    
    // Base cooldown 5.0, casting speed 0 (so spd=100) -> actual cooldown 10.0 (based on my revised calculate_cooldown)
    let triggered = m2_skill_trigger(engine_ptr, 1, 123, 10, 1.0, 5.0, 0.0, &mut pkt as *mut _);
    assert!(triggered);
    
    let pkt_vnum = pkt.dw_vnum;
    let pkt_target_vid = pkt.dw_target_vid;
    assert_eq!(pkt_vnum, 1);
    assert_eq!(pkt_target_vid, 123);
    
    let engine = unsafe { &mut *engine_ptr };
    assert_eq!(engine.current_sp, 90);
    
    let can_use_now = m2_combat_skill_can_use(engine_ptr, 1, 10, 0.5);
    assert!(!can_use_now);
    
    // While casting (time < 1.0), remaining cooltime should return full actual cooldown
    let rem_cool = m2_skill_get_remaining_cooltime(engine_ptr, 1, 0.5);
    assert_eq!(rem_cool, 10.0);
    
    // Cooldown phase (time >= 1.0)
    // Cooldown ends at 1.0 + 10.0 = 11.0
    // At time 1.5, remaining cooldown is 11.0 - 1.5 = 9.5
    let rem_cool2 = m2_skill_get_remaining_cooltime(engine_ptr, 1, 1.5);
    assert_eq!(rem_cool2, 9.5);
    
    // At time 11.1, cooldown is over
    let rem_cool3 = m2_skill_get_remaining_cooltime(engine_ptr, 1, 11.1);
    assert_eq!(rem_cool3, 0.0);
    
    let can_use_later = m2_combat_skill_can_use(engine_ptr, 1, 10, 11.1);
    assert!(can_use_later);

    m2_skill_engine_free(engine_ptr);
}

#[test]
fn test_not_enough_sp() {
    let engine_ptr = m2_skill_engine_new(5, 0);
    let can_use = m2_combat_skill_can_use(engine_ptr, 1, 10, 0.0);
    assert!(!can_use);
    m2_skill_engine_free(engine_ptr);
}

#[test]
fn test_cooldown_calculation() {
    let mut engine = CombatSkillEngine::new(100, 0);
    assert_eq!(engine.calculate_cooldown(10.0), 20.0);

    engine.casting_speed = -50;
    assert_eq!(engine.calculate_cooldown(10.0), 25.0);

    engine.casting_speed = 150;
    // 10.0 * 66 / 100 = 6.6
    assert_eq!(engine.calculate_cooldown(10.0), 6.6);
}
