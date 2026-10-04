#[path = "../src/horse_combat_skill_engine.rs"]
mod horse_combat_skill_engine;

use horse_combat_skill_engine::*;
use std::mem::size_of;

#[test]
fn test_can_use_horse_skill_military_vs_combat() {
    // Combat horse (level 11-20)
    assert_eq!(HorseCombatSkillEngine::can_use_horse_skill(11, HORSE_SKILL_SLASH), false);
    assert_eq!(HorseCombatSkillEngine::can_use_horse_skill(20, HORSE_SKILL_STOMP), false);
    
    // Military horse (level 21+)
    assert_eq!(HorseCombatSkillEngine::can_use_horse_skill(21, HORSE_SKILL_CHARGE), true);
    assert_eq!(HorseCombatSkillEngine::can_use_horse_skill(30, HORSE_SKILL_ARROW_SHOWER), true);

    // Invalid skill
    assert_eq!(HorseCombatSkillEngine::can_use_horse_skill(25, 999), false);
}

#[test]
fn test_calculate_charge_knockback_inside_cone() {
    let attacker_x = 0.0;
    let attacker_y = 0.0;
    let attacker_rot = 270.0; // facing along X axis (since 270-270 = 0 -> cos(0)=1, sin(0)=0)
    
    let target_x = 10.0;
    let target_y = 0.0;

    let charge_distance = 15.0;
    let cone_angle = 90.0;
    let impulse = 5.0;

    let result = HorseCombatSkillEngine::calculate_charge_knockback(
        attacker_x, attacker_y, attacker_rot,
        target_x, target_y,
        charge_distance, cone_angle, impulse
    );

    assert!(result.is_some());
    let (dist, kx, ky) = result.unwrap();
    assert!((dist - 10.0).abs() < 0.001);
    assert!((kx - 5.0).abs() < 0.001); // full impulse on X
    assert!((ky - 0.0).abs() < 0.001); // no impulse on Y
}

#[test]
fn test_calculate_charge_knockback_outside_cone() {
    let attacker_x = 0.0;
    let attacker_y = 0.0;
    let attacker_rot = 270.0; // facing along X axis (0 rad)
    
    // Target is behind the attacker
    let target_x = -10.0;
    let target_y = 0.0;

    let charge_distance = 15.0;
    let cone_angle = 90.0; // +/- 45 degrees
    let impulse = 5.0;

    let result = HorseCombatSkillEngine::calculate_charge_knockback(
        attacker_x, attacker_y, attacker_rot,
        target_x, target_y,
        charge_distance, cone_angle, impulse
    );

    assert!(result.is_none());
}

#[test]
fn test_calculate_charge_knockback_out_of_range() {
    let attacker_x = 0.0;
    let attacker_y = 0.0;
    let attacker_rot = 270.0;
    
    let target_x = 20.0;
    let target_y = 0.0;

    let charge_distance = 15.0;
    let cone_angle = 90.0;
    let impulse = 5.0;

    let result = HorseCombatSkillEngine::calculate_charge_knockback(
        attacker_x, attacker_y, attacker_rot,
        target_x, target_y,
        charge_distance, cone_angle, impulse
    );

    assert!(result.is_none());
}

#[test]
fn test_build_horse_skill_packet() {
    let target_vid = 12345;
    let bytes = HorseCombatSkillEngine::build_horse_skill_packet(HORSE_SKILL_SLASH, target_vid);
    
    assert!(bytes.is_some());
    let bytes = bytes.unwrap();

    assert_eq!(bytes.len(), size_of::<TPacketCGUseSkill>());

    // Verify unaligned fields properly
    let packet: TPacketCGUseSkill = unsafe { std::ptr::read(bytes.as_ptr() as *const _) };
    
    let header = { packet.header };
    let length = { packet.length };
    let vnum = { packet.dw_vnum };
    let vid = { packet.dw_target_vid };

    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(length, size_of::<TPacketCGUseSkill>() as u16);
    assert_eq!(vnum, HORSE_SKILL_SLASH);
    assert_eq!(vid, target_vid);
}

#[test]
fn test_build_horse_skill_packet_invalid_skill() {
    let target_vid = 12345;
    let bytes = HorseCombatSkillEngine::build_horse_skill_packet(999, target_vid);
    
    assert!(bytes.is_none());
}
