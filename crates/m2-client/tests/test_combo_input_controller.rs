use m2_client::actor_combo_window_timing::{ComboInputData, RaceMotionData};
use m2_client::engine_combo_input_controller::EngineComboInputController;

#[test]
fn test_hold_attack_key_continuous_combo() {
    let mut controller = EngineComboInputController::new();
    controller.set_attack_key_state(true);
    
    // Initial attack
    assert_eq!(controller.update_frame(0.1), true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Setup motion data for combo 1
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    // Advance time to pre-input window (0.3s)
    assert_eq!(controller.update_frame(0.3), false);
    assert_eq!(controller.combo_state.is_pre_input, true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Advance time past next_combo_time (0.3 + 0.3 = 0.6s > 0.5s)
    assert_eq!(controller.update_frame(0.3), true); // Triggered next combo
    assert_eq!(controller.combo_state.cur_combo_index, 2);
    assert_eq!(controller.combo_state.is_pre_input, false);
    assert_eq!(controller.combo_state.attacking_elapsed_time, 0.0); // Reset for combo 2
}

#[test]
fn test_single_click_pre_input_buffering() {
    let mut controller = EngineComboInputController::new();
    
    // Initial manual attack
    assert_eq!(controller.trigger_single_attack(), true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Setup motion data
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    // Advance time to pre-input window without holding key
    controller.update_frame(0.3);
    assert_eq!(controller.combo_state.is_pre_input, false);
    
    // Single click within pre-input window
    assert_eq!(controller.trigger_single_attack(), false);
    assert_eq!(controller.combo_state.is_pre_input, true);
    
    // Advance time past next_combo_time
    assert_eq!(controller.update_frame(0.3), true);
    assert_eq!(controller.combo_state.cur_combo_index, 2);
    assert_eq!(controller.combo_state.is_pre_input, false);
}

#[test]
fn test_single_click_too_late_expiration() {
    let mut controller = EngineComboInputController::new();
    
    // Initial attack
    controller.trigger_single_attack();
    
    // Setup motion data
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    // Advance time past input_end_time (0.9s > 0.8s)
    controller.update_frame(0.9);
    
    // Single click too late
    assert_eq!(controller.trigger_single_attack(), true); // Now correctly triggers as a new combo
    assert_eq!(controller.combo_state.is_pre_input, false);
    assert_eq!(controller.combo_state.cur_combo_index, 1); // Started fresh at 1
}

#[test]
fn test_full_four_hit_combo_sequence() {
    let mut controller = EngineComboInputController::new();
    controller.set_attack_key_state(true);

    // 1) Trigger combo 1
    assert_eq!(controller.update_frame(0.1), true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);

    // Motion data for combo 1 -> 2
    let motion_data_1 = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data_1);

    // Enter pre-input for combo 2
    assert_eq!(controller.update_frame(0.3), false);
    assert_eq!(controller.combo_state.is_pre_input, true);

    // Trigger combo 2
    assert_eq!(controller.update_frame(0.3), true);
    assert_eq!(controller.combo_state.cur_combo_index, 2);
    assert_eq!(controller.combo_state.is_pre_input, false);

    // Motion data for combo 2 -> 3
    let motion_data_2 = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.3,
            next_combo_time: 0.6,
            input_end_time: 0.9,
        }),
        motion_duration: 1.2,
    };
    controller.set_motion_data(motion_data_2);

    // Enter pre-input for combo 3
    assert_eq!(controller.update_frame(0.4), false);
    assert_eq!(controller.combo_state.is_pre_input, true);

    // Trigger combo 3
    assert_eq!(controller.update_frame(0.3), true);
    assert_eq!(controller.combo_state.cur_combo_index, 3);
    assert_eq!(controller.combo_state.is_pre_input, false);

    // Motion data for combo 3 -> 4
    let motion_data_3 = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.25,
            next_combo_time: 0.55,
            input_end_time: 0.85,
        }),
        motion_duration: 1.1,
    };
    controller.set_motion_data(motion_data_3);

    // Enter pre-input for combo 4
    assert_eq!(controller.update_frame(0.3), false);
    assert_eq!(controller.combo_state.is_pre_input, true);

    // Trigger combo 4
    assert_eq!(controller.update_frame(0.3), true);
    assert_eq!(controller.combo_state.cur_combo_index, 4);
    assert_eq!(controller.combo_state.is_pre_input, false);
}

#[test]
fn test_clear_combo_resets_state() {
    let mut controller = EngineComboInputController::new();
    controller.set_attack_key_state(true);
    controller.update_frame(0.1);
    
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    controller.clear_combo();
    
    assert_eq!(controller.combo_state.cur_combo_index, 0);
    assert_eq!(controller.is_atk_key_pressed, false);
    assert_eq!(controller.combo_state.is_pre_input, false);
}

#[test]
fn test_single_click_instant_combo_transition() {
    let mut controller = EngineComboInputController::new();
    
    // Initial attack
    assert_eq!(controller.trigger_single_attack(), true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Setup motion data
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    // Advance time EXACTLY past next_combo_time (0.6s)
    // In this state, is_pre_input is currently false, but the time is ripe for the next combo.
    controller.update_frame(0.6);
    assert_eq!(controller.combo_state.is_pre_input, false);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Single click should instantly transition the combo
    // because input_combo_attack_command returns false (sets is_pre_input),
    // and then combo_process sees is_pre_input and time > next_combo_time,
    // thereby instantly advancing the combo index and returning true.
    assert_eq!(controller.trigger_single_attack(), true);
    assert_eq!(controller.combo_state.cur_combo_index, 2);
    assert_eq!(controller.combo_state.is_pre_input, false);
}

#[test]
fn test_single_click_deferred_pre_input_buffering() {
    let mut controller = EngineComboInputController::new();
    
    // Initial attack
    assert_eq!(controller.trigger_single_attack(), true);
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.6,
            input_end_time: 1.0,
        }),
        motion_duration: 1.2,
    };
    controller.set_motion_data(motion_data);
    
    // Advance into pre-input window (0.4 > 0.2 but < 0.6)
    controller.update_frame(0.4);
    
    // Player clicks in the window
    assert_eq!(controller.trigger_single_attack(), false);
    assert_eq!(controller.combo_state.is_pre_input, true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Engine automatically transitions frame later when time > 0.6
    assert_eq!(controller.update_frame(0.3), true); 
    assert_eq!(controller.combo_state.cur_combo_index, 2);
    assert_eq!(controller.combo_state.is_pre_input, false);
}

#[test]
fn test_bow_mode_combo_input_expiration() {
    let mut controller = EngineComboInputController::new();
    controller.combo_state.is_bow_mode = true;
    
    // Initial attack
    assert_eq!(controller.trigger_single_attack(), true);
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    // Advance time past input_end_time (0.9s > 0.8s)
    controller.update_frame(0.9);
    
    // In bow mode, expiration does not instantly clear but sets is_next_pre_input
    assert_eq!(controller.trigger_single_attack(), false);
    assert_eq!(controller.combo_state.is_next_pre_input, true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
}

#[test]
fn test_combo_process_clear_on_missing_motion_data() {
    let mut controller = EngineComboInputController::new();
    
    // Initial attack
    assert_eq!(controller.trigger_single_attack(), true);
    assert_eq!(controller.combo_state.cur_combo_index, 1);
    
    // Simulate motion data unexpectedly being null mid-combo
    controller.combo_state.cur_race_motion_data = None;
    
    // Process should clear
    controller.update_frame(0.1);
    assert_eq!(controller.combo_state.cur_combo_index, 0);
}
