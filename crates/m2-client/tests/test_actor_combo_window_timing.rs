use m2_client::actor_combo_window_timing::{ActorComboState, ComboInputData, RaceMotionData};

#[test]
fn test_pre_input_triggered_within_start_time_bounds() {
    let mut state = ActorComboState::new();
    state.cur_combo_index = 1;
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    state.cur_race_motion_data = Some(motion_data);
    
    // Simulate attacking elapsed time within the pre-input window (> start, < next)
    state.attacking_elapsed_time = 0.3;
    
    let result = state.input_combo_attack_command();
    
    assert_eq!(result, false);
    assert_eq!(state.is_pre_input, true);
    assert_eq!(state.cur_combo_index, 1); // Combo index shouldn't change yet
}

#[test]
fn test_immediate_combo_continuation_exceeding_next_combo_time() {
    let mut state = ActorComboState::new();
    state.cur_combo_index = 1;
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    state.cur_race_motion_data = Some(motion_data);
    
    // Simulate attacking elapsed time > next_combo_time, but < input_end_time
    state.attacking_elapsed_time = 0.6;
    
    let result = state.input_combo_attack_command();
    
    assert_eq!(result, true);
    assert_eq!(state.is_pre_input, false);
    assert_eq!(state.cur_combo_index, 2); // Combo index increments immediately
}

#[test]
fn test_expiration_logic_past_input_end_time() {
    let mut state = ActorComboState::new();
    state.cur_combo_index = 1;
    state.is_bow_mode = true;
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    state.cur_race_motion_data = Some(motion_data);
    
    // Simulate elapsed time > input_end_time
    state.attacking_elapsed_time = 0.9;
    
    let result = state.input_combo_attack_command();
    
    assert_eq!(result, false);
    assert_eq!(state.is_pre_input, false);
    assert_eq!(state.is_next_pre_input, true); // Bow mode buffer
    assert_eq!(state.cur_combo_index, 1); // Combo index doesn't change
}

#[test]
fn test_fallback_duration_logic() {
    let mut state = ActorComboState::new();
    state.cur_combo_index = 1;
    
    let motion_data = RaceMotionData {
        combo_input_data: None, // No combo data
        motion_duration: 1.0,
    };
    state.cur_race_motion_data = Some(motion_data);
    
    // Simulate elapsed time < motion_duration * 0.9 (0.9)
    state.attacking_elapsed_time = 0.8;
    let result = state.input_combo_attack_command();
    assert_eq!(result, false);
    assert_eq!(state.cur_combo_index, 1);
    
    // Simulate elapsed time > motion_duration * 0.9 (0.9)
    state.attacking_elapsed_time = 0.95;
    let result = state.input_combo_attack_command();
    assert_eq!(result, true);
    assert_eq!(state.cur_combo_index, 2);
}

#[test]
fn test_combo_process_trigger_after_pre_input() {
    let mut state = ActorComboState::new();
    state.cur_combo_index = 1;
    state.is_pre_input = true;
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    state.cur_race_motion_data = Some(motion_data.clone());
    
    // Simulate elapsed time < next_combo_time
    state.attacking_elapsed_time = 0.4;
    state.combo_process();
    
    assert_eq!(state.cur_combo_index, 1);
    assert_eq!(state.is_pre_input, true);
    
    // Simulate elapsed time > next_combo_time
    state.attacking_elapsed_time = 0.6;
    state.combo_process();
    
    assert_eq!(state.cur_combo_index, 2);
    assert_eq!(state.is_pre_input, false);
}
