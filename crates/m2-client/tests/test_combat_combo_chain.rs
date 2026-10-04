use m2_client::combat_combo_engine::{HEADER_CG_ATTACK, WeaponType};
use m2_client::combat_combo_chain::CombatComboChain;

#[test]
fn test_combo_progression_and_timeout() {
    let mut chain = CombatComboChain::new(WeaponType::Sword, false);
    assert_eq!(chain.combo_index, 0);

    // Initial attack
    assert!(chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 1);

    // Too early
    chain.update(0.2);
    assert!(!chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 1);

    // Just right for 2nd combo
    chain.update(0.4); // total 0.6
    assert!(chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 2);

    // Just right for 3rd combo
    chain.update(0.6); // total 0.6 since last reset
    assert!(chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 3);

    // Just right for 4th combo
    chain.update(0.6); 
    assert!(chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 4);

    // Max combo reached, should not advance further unless reset
    chain.update(0.6); 
    assert!(!chain.input_combo_attack(1.0, 0.5)); // returns false
    assert_eq!(chain.combo_index, 4); 

    // Reset via timeout
    chain.update(1.2); 
    assert!(!chain.input_combo_attack(1.0, 0.5)); // Elapsed > 1.0, so it resets and returns false
    assert_eq!(chain.combo_index, 0);

    // Can start over
    assert!(chain.input_combo_attack(1.0, 0.5));
    assert_eq!(chain.combo_index, 1);
}

#[test]
fn test_attack_geometry_reach_and_cone() {
    let chain = CombatComboChain::new(WeaponType::Sword, false); // Sword uses 0.5 (120 degree cone)
    
    // Attacker at 0,0 looking at 0 degrees (rot = 0 -> sin(0)=0, cos(0)=1 -> vector (0, 1))
    let hit1 = chain.calculate_attack_geometry(0.0, 0.0, 0.0, 0.0, 100.0, 50.0);
    assert!(hit1, "Should hit target straight ahead");

    // Target slightly to side, within 60 deg (half-angle)
    let hit2 = chain.calculate_attack_geometry(0.0, 0.0, 0.0, 80.0, 100.0, 50.0); // angle is atan(80/100) ~ 38 deg. cos(38) ~ 0.78 >= 0.5
    assert!(hit2, "Should hit target within cone");

    // Target far outside cone (behind)
    let hit3 = chain.calculate_attack_geometry(0.0, 0.0, 0.0, 0.0, -100.0, 50.0);
    assert!(!hit3, "Should not hit target behind attacker");

    // Target outside reach
    // Sword reach = 150 + victim_radius 50 = 200. Distance = 250
    let hit4 = chain.calculate_attack_geometry(0.0, 0.0, 0.0, 0.0, 250.0, 50.0);
    assert!(!hit4, "Should not hit target outside reach");

    // Point-blank target (dist_sq < 625), should hit even if behind
    let hit5 = chain.calculate_attack_geometry(0.0, 0.0, 0.0, 0.0, -20.0, 50.0);
    assert!(hit5, "Should hit point-blank target regardless of angle");
}

#[test]
fn test_packet_creation() {
    let mut chain = CombatComboChain::new(WeaponType::Sword, false);
    
    // First attack
    chain.input_combo_attack(1.0, 0.5);
    
    let packet = chain.create_attack_packet(12345);
    
    // Avoid unaligned reference errors by unpacking fields directly for assertion or block scopes
    let header = { packet.header };
    let length = { packet.length };
    let b_type = { packet.b_type };
    let dw_victim_vid = { packet.dw_victim_vid };
    
    assert_eq!(header, HEADER_CG_ATTACK);
    assert_eq!(length, std::mem::size_of::<m2_client::combat_combo_engine::TPacketCGAttack>() as u16);
    assert_eq!(b_type, 1);
    assert_eq!(dw_victim_vid, 12345);
    
    // Second attack
    chain.update(0.6);
    chain.input_combo_attack(1.0, 0.5);
    
    let packet2 = chain.create_attack_packet(12345);
    let b_type2 = { packet2.b_type };
    assert_eq!(b_type2, 2);
}
