use m2_client::ui_window_target_board_complete::UiWindowTargetBoardComplete;

#[test]
fn test_target_board_init_and_select() {
    let mut board = UiWindowTargetBoardComplete::new();
    assert!(!board.is_visible());
    assert_eq!(board.target_vid(), None);

    board.set_target(1234, "Test Boss", 99, "Boss", 5000, 10000);
    assert!(board.is_visible());
    assert_eq!(board.target_vid(), Some(1234));
    
    let target = board.current_target.as_ref().unwrap();
    assert_eq!(target.name, "Test Boss");
    assert_eq!(target.level, 99);
    assert_eq!(target.race_type, "Boss");
    assert_eq!(target.hp, 5000);
    assert_eq!(target.max_hp, 10000);
    assert_eq!(target.hp_percent, 50);
}

#[test]
fn test_target_board_hp_update() {
    let mut board = UiWindowTargetBoardComplete::new();
    board.set_target(5678, "Player1", 50, "Player", 2000, 2000);

    // Take damage
    board.update_hp(1000, 2000);
    let target = board.current_target.as_ref().unwrap();
    assert_eq!(target.hp, 1000);
    assert_eq!(target.hp_percent, 50);

    // Die
    board.update_hp(0, 2000);
    assert!(!board.is_visible());
    assert_eq!(board.target_vid(), None);
}

#[test]
fn test_target_board_status_icons() {
    let mut board = UiWindowTargetBoardComplete::new();
    board.set_target(999, "Mob", 10, "Animal", 100, 100);

    // Initial state
    let target = board.current_target.as_ref().unwrap();
    assert!(!target.is_poisoned);
    assert!(!target.is_bleeding);
    assert!(!target.is_slowed);

    // Apply poison and slow
    board.set_status_effects(true, false, true);
    let target = board.current_target.as_ref().unwrap();
    assert!(target.is_poisoned);
    assert!(!target.is_bleeding);
    assert!(target.is_slowed);

    // Clear all
    board.set_status_effects(false, false, false);
    let target = board.current_target.as_ref().unwrap();
    assert!(!target.is_poisoned);
    assert!(!target.is_bleeding);
    assert!(!target.is_slowed);
}

#[test]
fn test_target_board_buttons() {
    let board = UiWindowTargetBoardComplete::new();
    // Verify all interaction buttons have valid handles
    assert!(board.btn_whisper > 0);
    assert!(board.btn_trade > 0);
    assert!(board.btn_party > 0);
    assert!(board.btn_guild > 0);
    assert!(board.btn_duel > 0);
    assert!(board.close_btn > 0);
}
