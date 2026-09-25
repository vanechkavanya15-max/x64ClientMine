use m2_client_bridge::state::party::PartyManager;

#[test]
fn test_add_member() {
    let mut pm = PartyManager::new();
    assert_eq!(pm.members.len(), 0);

    // Dodanie czlonka
    assert!(pm.add_member(100));
    assert_eq!(pm.members.len(), 1);

    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.vid, 100);
    assert_eq!(m.hp_percent, 0);
    assert_eq!(m.distance, 0);

    // Dodanie istniejacego czlonka
    assert!(!pm.add_member(100));
    assert_eq!(pm.members.len(), 1);
}

#[test]
fn test_remove_member() {
    let mut pm = PartyManager::new();
    pm.add_member(100);
    pm.add_member(200);

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
fn test_set_hp() {
    let mut pm = PartyManager::new();
    pm.add_member(100);

    // Zmiana HP istniejacego czlonka
    assert!(pm.set_hp(100, 75));
    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.hp_percent, 75);

    // Zmiana HP nieistniejacego czlonka
    assert!(!pm.set_hp(200, 50));
}

#[test]
fn test_set_distance() {
    let mut pm = PartyManager::new();
    pm.add_member(100);

    // Zmiana odleglosci istniejacego czlonka
    assert!(pm.set_distance(100, 1500));
    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.distance, 1500);

    // Zmiana odleglosci nieistniejacego czlonka
    assert!(!pm.set_distance(200, 1000));
}

#[test]
fn test_get_member_by_index() {
    let mut pm = PartyManager::new();
    pm.add_member(100);
    pm.add_member(200);

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
    pm.add_member(100);

    // Pobranie istniejacego czlonka po VID
    let m = pm.get_member_by_vid(100).unwrap();
    assert_eq!(m.vid, 100);

    // Pobranie nieistniejacego czlonka po VID
    assert!(pm.get_member_by_vid(200).is_none());
}

#[test]
fn test_clear_party() {
    let mut pm = PartyManager::new();
    pm.add_member(100);
    pm.add_member(200);
    
    assert_eq!(pm.members.len(), 2);
    pm.clear();
    assert_eq!(pm.members.len(), 0);
}
