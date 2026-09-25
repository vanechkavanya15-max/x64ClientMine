use m2_client::ui_select_screen::{UiSelectScreen, SimplePlayerInformation, PLAYER_PER_ACCOUNT4, CHARACTER_NAME_MAX_LEN};

fn create_dummy_player(id: u32, name: &str) -> SimplePlayerInformation {
    SimplePlayerInformation {
        id,
        name: name.to_string(),
        job: 1,
        level: 1,
        play_minutes: 0,
        st: 10,
        ht: 12,
        dx: 8,
        iq: 6,
        main_part: 0,
        change_name: 0,
        hair_part: 0,
        x: 0,
        y: 0,
        addr: 0,
        port: 0,
        skill_group: 0,
    }
}

#[test]
fn test_integration_ui_select_screen_ready() {
    let instance = UiSelectScreen::new();
    assert!(instance.is_ready());
}

#[test]
fn test_select_empire() {
    let mut ui = UiSelectScreen::new();
    assert!(ui.send_select_empire_packet(2));
    assert_eq!(ui.empire_id, 2);
}

#[test]
fn test_create_and_select_character() {
    let mut ui = UiSelectScreen::new();
    
    // Simulate successful creation
    let player = create_dummy_player(100, "Wojownik");
    assert!(ui.recv_player_create_success_packet(0, player));
    
    // Select the character
    assert!(ui.select_slot(0));
    assert_eq!(ui.selected_slot, Some(0));
    
    // Verify preview and stats
    let preview = ui.get_selected_character_preview().unwrap();
    assert_eq!(preview.name, "Wojownik");
    
    let stats = ui.get_selected_character_stats().unwrap();
    assert_eq!(stats, (12, 6, 10, 8)); // (ht, iq, st, dx)
}

#[test]
fn test_select_empty_slot() {
    let mut ui = UiSelectScreen::new();
    assert!(!ui.select_slot(0));
    assert_eq!(ui.selected_slot, None);
}

#[test]
fn test_destroy_character() {
    let mut ui = UiSelectScreen::new();
    let player = create_dummy_player(100, "Wojownik");
    ui.recv_player_create_success_packet(0, player);
    
    ui.select_slot(0);
    assert_eq!(ui.selected_slot, Some(0));
    
    assert!(ui.recv_player_destroy_success_packet(0));
    assert!(ui.players[0].is_none());
    assert_eq!(ui.selected_slot, None);
}

#[test]
fn test_change_name() {
    let mut ui = UiSelectScreen::new();
    let mut player = create_dummy_player(100, "OldName");
    player.change_name = 1;
    ui.recv_player_create_success_packet(1, player);
    
    assert!(ui.recv_change_name(100, "NewName"));
    assert_eq!(ui.players[1].as_ref().unwrap().name, "NewName");
    assert_eq!(ui.players[1].as_ref().unwrap().change_name, 0);
}

#[test]
fn test_send_packets_validation() {
    let ui = UiSelectScreen::new();
    
    // Too long name
    let long_name = "a".repeat(CHARACTER_NAME_MAX_LEN + 1);
    assert!(!ui.send_create_character_packet(0, &long_name, 1, 1, 10, 10, 10, 10));
    assert!(!ui.send_change_name_packet(0, &long_name));
    
    // Invalid slot
    assert!(!ui.send_select_character_packet(PLAYER_PER_ACCOUNT4 as u8));
    assert!(!ui.send_destroy_character_packet(PLAYER_PER_ACCOUNT4 as u8, "1234567"));
}
