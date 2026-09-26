#[path = "../src/combo_state_machine.rs"]
mod combo_state_machine;

use combo_state_machine::{ComboState, ComboStateMachine, ComboTimeData};

#[test]
fn test_basic_combo_sequence() {
    let mut sm = ComboStateMachine::new(4);
    assert_eq!(sm.cur_combo_index, 0);
    assert_eq!(sm.state, ComboState::Idle);

    // Hit 1
    let triggered = sm.input_attack();
    assert!(triggered);
    assert_eq!(sm.cur_combo_index, 1);
    assert_eq!(sm.state, ComboState::Attacking);

    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // Wait until ComboWindowOpen
    sm.update(0.45);
    assert_eq!(sm.state, ComboState::ComboWindowOpen);

    // Hit 2
    let triggered = sm.input_attack();
    assert!(triggered);
    assert_eq!(sm.cur_combo_index, 2);
    assert_eq!(sm.elapsed_time, 0.0);

    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // Wait until ComboWindowOpen
    sm.update(0.45);

    // Hit 3
    let triggered = sm.input_attack();
    assert!(triggered);
    assert_eq!(sm.cur_combo_index, 3);

    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // Wait until ComboWindowOpen
    sm.update(0.45);

    // Hit 4 (Last hit)
    let triggered = sm.input_attack();
    assert!(triggered);
    assert_eq!(sm.cur_combo_index, 4);

    // Attempt hit 5, should be rejected for foot player (max combo reached)
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    sm.update(0.45);
    let triggered = sm.input_attack();
    assert!(!triggered, "Cios 5 na pieszo powinien zostac odrzucony");
    assert_eq!(sm.cur_combo_index, 4);
}

#[test]
fn test_pre_input_buffering() {
    let mut sm = ComboStateMachine::new(4);
    sm.input_attack(); // Hit 1
    
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // Move to PreInputWindow
    sm.update(0.3);
    assert_eq!(sm.state, ComboState::PreInputWindow);

    // Input attack in PreInputWindow
    let triggered = sm.input_attack();
    assert!(!triggered); // Doesn't run next combo instantly
    assert!(sm.is_pre_input);

    // Another update to push past next_combo_time, it should auto trigger
    sm.update(0.15); // total 0.45
    assert_eq!(sm.state, ComboState::Attacking);
    assert_eq!(sm.cur_combo_index, 2);
    assert!(!sm.is_pre_input);
}

#[test]
fn test_combo_timeout() {
    let mut sm = ComboStateMachine::new(4);
    sm.input_attack(); // Hit 1
    
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // Move past combo_window_end
    sm.update(0.7);
    
    assert_eq!(sm.state, ComboState::Idle);
    assert_eq!(sm.cur_combo_index, 0); // Expired and reset
}

#[test]
fn test_mounting_horse_combo_loop() {
    let mut sm = ComboStateMachine::new(4);
    sm.set_mounting_horse(true);
    
    // hit 1
    sm.input_attack();
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.1,
        next_combo_time: 0.2,
        combo_window_end: 0.3,
        motion_duration: 0.4,
        has_time_data: true,
    }));
    sm.update(0.25);
    
    // hit 2
    sm.input_attack();
    sm.update(0.25);
    
    // hit 3
    sm.input_attack();
    sm.update(0.25);
    
    // hit 4
    sm.input_attack();
    sm.update(0.25);
    
    // hit 5, triggers on_end_combo -> resets to 1
    sm.input_attack();
    assert_eq!(sm.cur_combo_index, 1);
}

#[test]
fn test_no_time_data_fallback() {
    let mut sm = ComboStateMachine::new(4);
    sm.input_attack(); // hit 1
    
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.0,
        next_combo_time: 0.0,
        combo_window_end: 0.0,
        motion_duration: 1.0,
        has_time_data: false, // fallback to motion_duration * 0.9
    }));
    
    sm.update(0.8);
    assert_eq!(sm.state, ComboState::Attacking);
    let triggered = sm.input_attack();
    assert!(!triggered); // not past 0.9
    
    sm.update(0.15); // total 0.95
    assert_eq!(sm.state, ComboState::ComboWindowOpen);
    let triggered = sm.input_attack();
    assert!(triggered);
    assert_eq!(sm.cur_combo_index, 2);
}

#[test]
fn test_late_input_rejection() {
    let mut sm = ComboStateMachine::new(4);
    sm.input_attack();
    
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // just before combo window end
    sm.update(0.55);
    // this is fine
    assert!(sm.input_attack());

    // try again with next combo
    sm.set_time_data(Some(ComboTimeData {
        combo_window_start: 0.2,
        next_combo_time: 0.4,
        combo_window_end: 0.6,
        motion_duration: 0.8,
        has_time_data: true,
    }));

    // move past window end, but don't call update yet. test input_attack specifically.
    sm.elapsed_time = 0.65;
    let triggered = sm.input_attack();
    assert!(!triggered); // Should return false as elapsed_time > combo_window_end
}
