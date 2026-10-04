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
    assert_eq!(controller.trigger_single_attack(), false);
    assert_eq!(controller.combo_state.is_pre_input, false);
    assert_eq!(controller.combo_state.cur_combo_index, 1); // Stays at 1
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
