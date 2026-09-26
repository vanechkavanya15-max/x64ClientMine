use m2_client::horse_combat_motion_v2::*;

#[test]
fn test_initialization_and_start() {
    let mut fsm = HorseCombatMotionV2::new();
    assert_eq!(fsm.cur_combo_index, 0);
    assert_eq!(fsm.max_combo_count, 5);
    assert_eq!(fsm.state, HorseComboState::Idle);

    // Initial attack input
    let started = fsm.input_attack();
    assert!(started);
    assert_eq!(fsm.cur_combo_index, 1);
    assert_eq!(fsm.state, HorseComboState::Attacking);
}

#[test]
fn test_combo_progression_and_looping() {
    let mut fsm = HorseCombatMotionV2::new();
    fsm.input_attack(); // combo 1
    
    // Without time data, combo progresses when elapsed time > duration * 0.9
    let mut time_data = HorseComboTimeData::default();
    time_data.motion_duration = 1.0;
    time_data.has_time_data = false;
    fsm.set_time_data(Some(time_data));

    // Progress to combo 2
    fsm.update(0.95);
    assert_eq!(fsm.state, HorseComboState::ComboWindowOpen);
    assert!(fsm.input_attack());
    assert_eq!(fsm.cur_combo_index, 2);

    // Progress to 3
    fsm.update(0.95);
    fsm.input_attack();
    assert_eq!(fsm.cur_combo_index, 3);

    // Progress to 4
    fsm.update(0.95);
    fsm.input_attack();
    assert_eq!(fsm.cur_combo_index, 4);

    // Progress to 5
    fsm.update(0.95);
    fsm.input_attack();
    assert_eq!(fsm.cur_combo_index, 5);

    // Loop back to 1
    fsm.update(0.95);
    fsm.input_attack();
    assert_eq!(fsm.cur_combo_index, 1);
}

#[test]
fn test_pre_input_mechanics() {
    let mut fsm = HorseCombatMotionV2::new();
    fsm.input_attack(); // combo 1

    let mut time_data = HorseComboTimeData::default();
    time_data.has_time_data = true;
    time_data.combo_window_start = 0.5;
    time_data.next_combo_time = 0.8;
    time_data.combo_window_end = 1.2;
    fsm.set_time_data(Some(time_data));

    // Wait until pre-input window
    fsm.update(0.6);
    assert_eq!(fsm.state, HorseComboState::PreInputWindow);
    
    // Input attack (gets queued)
    let triggered_now = fsm.input_attack();
    assert!(!triggered_now);
    assert!(fsm.is_pre_input);

    // Wait until next combo time
    fsm.update(0.3); // total elapsed 0.9
    assert_eq!(fsm.state, HorseComboState::Attacking); // it automatically ran next combo
    assert_eq!(fsm.cur_combo_index, 2);
    assert!(!fsm.is_pre_input);
}

#[test]
fn test_combo_expiration() {
    let mut fsm = HorseCombatMotionV2::new();
    fsm.input_attack(); // combo 1

    let mut time_data = HorseComboTimeData::default();
    time_data.has_time_data = true;
    time_data.combo_window_start = 0.5;
    time_data.next_combo_time = 0.8;
    time_data.combo_window_end = 1.2;
    fsm.set_time_data(Some(time_data));

    // Wait past combo end
    fsm.update(1.5);
    assert_eq!(fsm.state, HorseComboState::Idle); // got cleared
    assert_eq!(fsm.cur_combo_index, 0);
}

#[test]
fn test_ffi_hit_sphere_radii() {
    assert_eq!(m2_horse_combat_get_hit_sphere(1), (120.0, 150.0));
    assert_eq!(m2_horse_combat_get_hit_sphere(2), (120.0, 150.0));
    assert_eq!(m2_horse_combat_get_hit_sphere(3), (130.0, 160.0));
    assert_eq!(m2_horse_combat_get_hit_sphere(4), (130.0, 160.0));
    assert_eq!(m2_horse_combat_get_hit_sphere(5), (150.0, 200.0));
    assert_eq!(m2_horse_combat_get_hit_sphere(6), (100.0, 100.0));
}

#[test]
fn test_immediate_input_after_next_combo_time() {
    let mut fsm = HorseCombatMotionV2::new();
    fsm.input_attack(); // combo 1

    let mut time_data = HorseComboTimeData::default();
    time_data.has_time_data = true;
    time_data.combo_window_start = 0.5;
    time_data.next_combo_time = 0.8;
    time_data.combo_window_end = 1.2;
    fsm.set_time_data(Some(time_data));

    fsm.update(0.9); // Go past next_combo_time
    assert_eq!(fsm.state, HorseComboState::ComboWindowOpen);
    
    // Input attack immediately
    let triggered_now = fsm.input_attack();
    assert!(triggered_now);
    assert_eq!(fsm.cur_combo_index, 2);
    assert_eq!(fsm.state, HorseComboState::Attacking);
}
