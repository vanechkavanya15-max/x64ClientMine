use m2_client_bridge::state::party::{PartyManager, PartyMember, m2_party_get_count, PARTY_MANAGER};

#[test]
fn test_add_member() {
    let mut pm = PartyManager::new();
    assert_eq!(pm.members.len(), 0);

    let p1 = PartyMember { vid: 100, name: "Player1".to_string(), hp_percent: 0, is_leader: true };

    // Dodanie czlonka
    assert!(pm.add_member(p1.clone()));
    assert_eq!(pm.members.len(), 1);

    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.vid, 100);
    assert_eq!(m.name, "Player1");
    assert_eq!(m.hp_percent, 0);
    assert!(m.is_leader);

    // Dodanie istniejacego czlonka
    assert!(!pm.add_member(p1));
    assert_eq!(pm.members.len(), 1);
}

#[test]
fn test_remove_member() {
    let mut pm = PartyManager::new();
    pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });
    pm.add_member(PartyMember { vid: 200, name: "B".to_string(), hp_percent: 100, is_leader: false });

    // Usuniecie nieistniejacego czlonka
    assert!(!pm.remove_member(300));
    assert_eq!(pm.members.len(), 2);

    // Usuniecie istniejacego czlonka
    assert!(pm.remove_member(100));
    assert_eq!(pm.members.len(), 1);
    assert!(pm.get_member_by_vid(100).is_none());
    assert!(pm.get_member_by_vid(200).is_some());
}

#[test]
fn test_update_member_hp() {
    let mut pm = PartyManager::new();
    pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });

    // Zmiana HP istniejacego czlonka
    assert!(pm.update_member_hp(100, 75));
    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.hp_percent, 75);

    // Zmiana HP nieistniejacego czlonka
    assert!(!pm.update_member_hp(200, 50));
}

#[test]
fn test_get_member_by_index() {
    let mut pm = PartyManager::new();
    pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });
    pm.add_member(PartyMember { vid: 200, name: "B".to_string(), hp_percent: 100, is_leader: false });

    // Pobranie poprawnego indeksu
    let m1 = pm.get_member_by_index(0).unwrap();
    assert_eq!(m1.vid, 100);
    let m2 = pm.get_member_by_index(1).unwrap();
    assert_eq!(m2.vid, 200);

    // Pobranie niepoprawnego indeksu
    assert!(pm.get_member_by_index(2).is_none());
}

#[test]
fn test_get_member_by_vid() {
    let mut pm = PartyManager::new();
    pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });

    // Pobranie istniejacego czlonka po VID
    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.vid, 100);

    // Pobranie nieistniejacego czlonka po VID
    assert!(pm.get_member_by_vid(200).is_none());
}

#[test]
fn test_clear_party() {
    let mut pm = PartyManager::new();
    pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });
    pm.add_member(PartyMember { vid: 200, name: "B".to_string(), hp_percent: 100, is_leader: false });
    
    assert_eq!(pm.members.len(), 2);
    pm.clear();
    assert_eq!(pm.members.len(), 0);
}

#[test]
fn test_m2_party_get_count() {
    // Upewniamy sie ze grupa jest pusta
    PARTY_MANAGER.write().unwrap().clear();
    
    // Sprawdzamy poczatkowy stan
    let count1 = unsafe { m2_party_get_count() };
    assert_eq!(count1, 0);
    
    // Dodajemy czlonkow do globalnego managera
    {
        let mut pm = PARTY_MANAGER.write().unwrap();
        pm.add_member(PartyMember { vid: 100, name: "A".to_string(), hp_percent: 100, is_leader: true });
        pm.add_member(PartyMember { vid: 200, name: "B".to_string(), hp_percent: 100, is_leader: false });
    }
    
    // Sprawdzamy stan po dodaniu
    let count2 = unsafe { m2_party_get_count() };
    assert_eq!(count2, 2);
}
