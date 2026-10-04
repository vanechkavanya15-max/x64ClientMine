use m2_client::ui_window_character_select_screen::{CharacterSelectScreen, CharacterSlotData};
use m2_client::character_create_delete_flow::Job;

#[test]
fn test_char_select_screen_slots() {
    let mut screen = CharacterSelectScreen::new();
    
    screen.set_slot(0, CharacterSlotData {
        name: "Test1".to_string(),
        level: 10,
        job: Job::WarriorM,
        str: 10, dex: 10, vit: 10, int: 10,
        play_time: 100,
        armor: 11209,
        weapon: 19,
        is_empty: false,
    });
    
    assert_eq!(screen.slots[0].is_empty, false);
    assert_eq!(screen.slots[0].name, "Test1");
    assert_eq!(screen.slots[0].armor, 11209);
    assert_eq!(screen.slots[0].weapon, 19);
    assert_eq!(screen.slots[1].is_empty, true);
    
    screen.select_slot(0);
    assert_eq!(screen.selected_slot, Some(0));
    
    screen.select_slot(1);
    assert_eq!(screen.selected_slot, Some(0));
}

#[test]
fn test_char_select_screen_packet() {
    let mut screen = CharacterSelectScreen::new();
    
    screen.set_slot(2, CharacterSlotData {
        name: "Mage".to_string(),
        level: 40,
        job: Job::ShamanF,
        str: 10, dex: 10, vit: 10, int: 90,
        play_time: 1000,
        armor: 11209,
        weapon: 19,
        is_empty: false,
    });
    
    screen.select_slot(2);
    let packet = screen.get_selected_character_packet().unwrap();
    
    assert_eq!(packet.player_index, 2);
    let length = { packet.length };
    assert_eq!(length, 5);
}

#[test]
fn test_char_select_camera_rotation() {
    let mut screen = CharacterSelectScreen::new();
    
    screen.set_slot(0, CharacterSlotData {
        name: "Test".to_string(),
        level: 1,
        job: Job::WarriorM,
        str: 1, dex: 1, vit: 1, int: 1,
        play_time: 1,
        armor: 11209,
        weapon: 19,
        is_empty: false,
    });
    
    screen.select_slot(0);
    assert_eq!(screen.camera_rotation, 0.0);
    
    screen.update(1.0);
    assert_eq!(screen.camera_rotation, 10.0);
}

#[test]
fn test_char_select_handle_button_click() {
    let mut screen = CharacterSelectScreen::new();
    
    // Nothing selected
    assert_eq!(screen.handle_button_click("StartGame"), false);
    assert_eq!(screen.handle_button_click("DeleteCharacter"), false);
    
    screen.set_slot(0, CharacterSlotData {
        name: "Test1".to_string(),
        level: 10,
        job: Job::WarriorM,
        str: 10, dex: 10, vit: 10, int: 10,
        play_time: 100,
        armor: 11209,
        weapon: 19,
        is_empty: false,
    });
    
    screen.select_slot(0);
    
    // Something selected
    // Note: StartGame attempts a socket write. In tests ACTIVE_GAME_SOCKET is empty,
    // but the branch still returns true.
    assert_eq!(screen.handle_button_click("StartGame"), true);
    
    // DeleteCharacter calls modal function
    assert_eq!(screen.handle_button_click("DeleteCharacter"), true);
    
    // Always true
    assert_eq!(screen.handle_button_click("CreateCharacter"), true);
    
    // Unknown button
    assert_eq!(screen.handle_button_click("UnknownButton"), false);
}
