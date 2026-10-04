#[path = "../src/party_sync_coordinator.rs"]
mod party_sync_coordinator;

use party_sync_coordinator::{
    PartySyncCoordinator, PartyRole, PartyMinimapMarker, PARTY_AFFECT_SLOT_MAX_NUM
};

#[test]
fn test_party_add_member_limit() {
    let mut coordinator = PartySyncCoordinator::new();

    // Dodaj 8 członków (limit)
    for i in 1..=8 {
        let added = coordinator.handle_party_add(i, format!("Player{}", i));
        assert!(added, "Powinno dodać gracza {}", i);
    }

    assert_eq!(coordinator.member_count(), 8);

    // Próba dodania 9-go członka powinna się nie powieść
    let added = coordinator.handle_party_add(9, "Player9".to_string());
    assert!(!added, "Nie powinno dodać 9-go gracza");
    assert_eq!(coordinator.member_count(), 8);
}

#[test]
fn test_party_update_hp_sp_role() {
    let mut coordinator = PartySyncCoordinator::new();
    let pid = 100;

    coordinator.handle_party_add(pid, "LeaderPlayer".to_string());
    let affects = [0; PARTY_AFFECT_SLOT_MAX_NUM];

    // Aktualizuj stan: Leader, 75% HP, 50% SP
    let updated = coordinator.handle_party_update(pid, 1, 75, 50, affects);
    assert!(updated);

    let member = coordinator.get_member(pid).unwrap();
    assert_eq!(member.hp_percent, 75);
    assert_eq!(member.sp_percent, 50);
    assert_eq!(member.role, PartyRole::Leader);
}

#[test]
fn test_party_remove_member() {
    let mut coordinator = PartySyncCoordinator::new();
    let pid = 200;

    coordinator.handle_party_add(pid, "Player200".to_string());
    assert_eq!(coordinator.member_count(), 1);

    let removed = coordinator.handle_party_remove(pid);
    assert!(removed);
    assert_eq!(coordinator.member_count(), 0);

    let removed_again = coordinator.handle_party_remove(pid);
    assert!(!removed_again);
}

#[test]
fn test_party_minimap_markers() {
    let mut coordinator = PartySyncCoordinator::new();
    let pid1 = 1;
    let pid2 = 2;

    coordinator.handle_party_add(pid1, "P1".to_string());
    coordinator.handle_party_add(pid2, "P2".to_string());

    coordinator.handle_party_link(pid1, 1001);
    coordinator.handle_party_link(pid2, 1002);

    let mut markers = coordinator.get_minimap_markers();
    markers.sort_by_key(|m| m.vid);

    assert_eq!(markers.len(), 2);
    assert_eq!(markers[0].vid, 1001);
    assert_eq!(markers[0].marker_type, "TYPE_PARTY");
    assert_eq!(markers[1].vid, 1002);
    assert_eq!(markers[1].marker_type, "TYPE_PARTY");

    coordinator.handle_party_unlink(pid1);

    let markers = coordinator.get_minimap_markers();
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].vid, 1002);
}

#[test]
fn test_party_buffs_and_parameters() {
    let mut coordinator = PartySyncCoordinator::new();
    let pid = 300;

    coordinator.handle_party_add(pid, "BuffedPlayer".to_string());
    
    let affects = [1, 2, 3, 4, 5, 6, 7];
    coordinator.handle_party_update(pid, 2, 100, 100, affects);
    
    let member = coordinator.get_member(pid).unwrap();
    assert_eq!(member.affects, affects);
    assert_eq!(member.role, PartyRole::Attacker);

    coordinator.handle_party_parameter(1);
    assert_eq!(coordinator.get_distribute_mode(), 1);
}
