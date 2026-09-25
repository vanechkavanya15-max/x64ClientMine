use m2_client_pyapi::mod_player::ModPlayer;

#[test]
fn test_integration_mod_player() {
    let instance = ModPlayer::new();
    assert!(instance.is_ready());
}

#[test]
fn test_player_status() {
    let mut player = ModPlayer::new();
    
    // Domyslne wartosci dla nieistniejacych statusow
    assert_eq!(player.get_status(1), 0);
    assert_eq!(player.get_status(2), 0);
    
    // Ustawienie statusow
    player.set_status(1, 100);
    player.set_status(2, -50);
    
    // Sprawdzenie ustawionych wartosci
    assert_eq!(player.get_status(1), 100);
    assert_eq!(player.get_status(2), -50);
    assert_eq!(player.get_status(3), 0); // Sprawdzenie nieistniejacego po dodaniu innych
}

#[test]
fn test_player_inventory() {
    let mut player = ModPlayer::new();
    
    // Domyslne wartosci dla nieistniejacych przedmiotow
    assert_eq!(player.get_item_index(0), 0);
    assert_eq!(player.get_item_count(0), 0);
    
    // Ustawienie przedmiotow na slotach
    player.set_item(0, 1000, 5); // slot 0, vnum 1000, 5 sztuk
    player.set_item(10, 2000, 1); // slot 10, vnum 2000, 1 sztuka
    
    // Sprawdzenie ustawionych wartosci
    assert_eq!(player.get_item_index(0), 1000);
    assert_eq!(player.get_item_count(0), 5);
    
    assert_eq!(player.get_item_index(10), 2000);
    assert_eq!(player.get_item_count(10), 1);
    
    // Sprawdzenie nieistniejacego slota
    assert_eq!(player.get_item_index(1), 0);
    assert_eq!(player.get_item_count(1), 0);
    
    // Nadpisanie slota
    player.set_item(0, 1001, 10);
    assert_eq!(player.get_item_index(0), 1001);
    assert_eq!(player.get_item_count(0), 10);
}

#[test]
fn test_player_skills() {
    let mut player = ModPlayer::new();
    
    // Domyslne wartosci dla nieistniejacych umiejetnosci
    assert_eq!(player.get_skill_level(1), 0);
    assert_eq!(player.get_skill_level(2), 0);
    
    // Ustawienie poziomow umiejetnosci
    player.set_skill_level(1, 15);
    player.set_skill_level(2, 20); // Master
    
    // Sprawdzenie ustawionych wartosci
    assert_eq!(player.get_skill_level(1), 15);
    assert_eq!(player.get_skill_level(2), 20);
    
    // Sprawdzenie nieistniejacej umiejetnosci
    assert_eq!(player.get_skill_level(3), 0);
    
    // Zmiana poziomu umiejetnosci
    player.set_skill_level(1, 16);
    assert_eq!(player.get_skill_level(1), 16);
}
