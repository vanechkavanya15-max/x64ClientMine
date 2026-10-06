#[path = "../src/party_formation_manager_v2.rs"]
mod party_formation_manager_v2;

use party_formation_manager_v2::*;

#[test]
fn test_party_member_add_remove_link() {
    let mut manager = PartyFormationManager::new();
    
    assert!(manager.append_member(1, "Player1").is_ok());
    assert!(manager.append_member(2, "Player2").is_ok());
    
    // Add existing
    assert!(manager.append_member(1, "Player1Again").is_err());
    
    assert_eq!(manager.get_member_count(), 2);
    
    assert!(manager.link_member(1, 100));
    assert!(manager.link_member(2, 200));
    
    assert!(manager.is_party_member_by_vid(100));
    assert!(manager.is_party_member_by_name("Player2"));
    
    assert!(manager.unlink_member(1));
    assert!(!manager.is_party_member_by_vid(100));
    
    assert!(manager.remove_member(2));
    assert_eq!(manager.get_member_count(), 1);
}

#[test]
fn test_party_max_members() {
    let mut manager = PartyFormationManager::new();
    
    for i in 1..=8 {
        assert!(manager.append_member(i, &format!("Player{}", i)).is_ok());
    }
    
    assert!(manager.append_member(9, "Player9").is_err());
}

#[test]
fn test_party_hp_and_state() {
    let mut manager = PartyFormationManager::new();
    manager.append_member(1, "Player1").unwrap();
    
    // HP cap test
    manager.update_member_info(1, 0, 150);
    assert_eq!(manager.get_member_by_pid(1).unwrap().hp_percent, 100);
    
    // Role update
    manager.update_member_info(1, 2, 50); // Attacker
    assert_eq!(manager.get_member_by_pid(1).unwrap().role, PartyRole::Attacker);
    assert_eq!(manager.get_member_by_pid(1).unwrap().hp_percent, 50);
    
    // Invalid role update
    manager.update_member_info(1, 99, 100);
    assert_eq!(manager.get_member_by_pid(1).unwrap().role, PartyRole::Attacker); // Role should not change
}

#[test]
fn test_party_group_bonuses() {
    let mut manager = PartyFormationManager::new();
    
    manager.append_member(1, "P1").unwrap();
    assert_eq!(manager.calculate_group_bonuses().exp_bonus_percent, 0);
    
    manager.append_member(2, "P2").unwrap();
    assert_eq!(manager.calculate_group_bonuses().exp_bonus_percent, 12);
    
    manager.append_member(3, "P3").unwrap();
    assert_eq!(manager.calculate_group_bonuses().exp_bonus_percent, 15);
    
    manager.append_member(4, "P4").unwrap();
    manager.append_member(5, "P5").unwrap();
    manager.append_member(6, "P6").unwrap();
    manager.append_member(7, "P7").unwrap();
    manager.append_member(8, "P8").unwrap();
    assert_eq!(manager.calculate_group_bonuses().exp_bonus_percent, 100);
}

#[test]
fn test_party_role_bonuses() {
    let mut manager = PartyFormationManager::new();
    
    manager.append_member(1, "P1").unwrap();
    manager.update_member_info(1, 2, 100); // Attacker
    
    manager.append_member(2, "P2").unwrap();
    manager.update_member_info(2, 7, 100); // Defender
    
    manager.append_member(3, "P3").unwrap();
    manager.update_member_info(3, 4, 100); // Buffer
    
    let bonuses = manager.calculate_group_bonuses();
    assert_eq!(bonuses.attacker_bonus, 1);
    assert_eq!(bonuses.defender_bonus, 1);
    assert_eq!(bonuses.buffer_bonus, 1);
    assert_eq!(bonuses.skill_master_bonus, 0);
}

#[test]
fn test_party_distance_bonuses() {
    let mut manager = PartyFormationManager::new();
    
    manager.append_member(1, "P1").unwrap();
    manager.append_member(2, "P2").unwrap();
    manager.append_member(3, "P3").unwrap();
    
    manager.update_member_info(2, 2, 100); // Attacker
    manager.update_member_info(3, 4, 100); // Buffer
    
    assert_eq!(manager.calculate_group_bonuses().exp_bonus_percent, 15);
    
    // P3 is out of range
    manager.update_member_near(3, false);
    
    let bonuses = manager.calculate_group_bonuses();
    assert_eq!(bonuses.exp_bonus_percent, 12);
    assert_eq!(bonuses.buffer_bonus, 0); // Out of range buffer
    assert_eq!(bonuses.attacker_bonus, 1); // In range attacker
}
