#[path = "../src/actor_combat_combo_chain_v2.rs"]
mod actor_combat_combo_chain_v2;

#[cfg(test)]
mod tests {
    use super::actor_combat_combo_chain_v2::{ActorCombatComboChainV2, RaceMotionData};

    #[test]
    fn test_normal_combo_input() {
        let mut chain = ActorCombatComboChainV2::new();
        chain.max_combo_count = 4;
        
        let accepted = chain.input_combo_attack_command();
        assert!(accepted, "First combo hit should be accepted");
        assert_eq!(chain.cur_combo_index, 1);
    }

    #[test]
    fn test_pre_input_buffering() {
        let mut chain = ActorCombatComboChainV2::new();
        chain.max_combo_count = 4;
        chain.cur_combo_index = 1;
        chain.attacking_elapsed_time = 0.5;

        chain.cur_race_motion_data = Some(RaceMotionData {
            has_combo_input_time_data: true,
            combo_input_start_time: 0.4,
            next_combo_time: 0.8,
            combo_input_end_time: 1.0,
            motion_duration: 1.0,
        });

        let accepted = chain.input_combo_attack_command();
        assert!(!accepted, "Should not immediately accept within pre-input window");
        assert!(chain.is_pre_input, "Pre-input flag should be set");
        
        chain.attacking_elapsed_time = 0.9;
        chain.combo_process();
        
        assert_eq!(chain.cur_combo_index, 2, "Combo index should increment when time reaches next_combo_time and pre-input was buffered");
        assert!(!chain.is_pre_input, "Pre-input should be cleared after hit");
    }

    #[test]
    fn test_timeout_combo() {
        let mut chain = ActorCombatComboChainV2::new();
        chain.max_combo_count = 4;
        chain.cur_combo_index = 1;
        chain.attacking_elapsed_time = 1.2;

        chain.cur_race_motion_data = Some(RaceMotionData {
            has_combo_input_time_data: true,
            combo_input_start_time: 0.4,
            next_combo_time: 0.8,
            combo_input_end_time: 1.0,
            motion_duration: 1.0,
        });

        let accepted = chain.input_combo_attack_command();
        assert!(!accepted, "Combo should be rejected when timeout is reached");
        assert!(!chain.is_pre_input);
    }

    #[test]
    fn test_horse_mounting_loop() {
        let mut chain = ActorCombatComboChainV2::new();
        chain.is_mounting_horse = true;
        chain.max_combo_count = 3;
        chain.cur_combo_index = 2; // last combo hit
        
        chain.run_next_combo();
        assert_eq!(chain.cur_combo_index, 1, "On horse, combo should loop back to 1 instead of stopping");
    }

    #[test]
    fn test_bow_mode_next_pre_input() {
        let mut chain = ActorCombatComboChainV2::new();
        chain.is_bow_mode = true;
        chain.max_combo_count = 4;
        chain.cur_combo_index = 1;
        chain.attacking_elapsed_time = 1.2;

        chain.cur_race_motion_data = Some(RaceMotionData {
            has_combo_input_time_data: true,
            combo_input_start_time: 0.4,
            next_combo_time: 0.8,
            combo_input_end_time: 1.0,
            motion_duration: 1.0,
        });

        let accepted = chain.input_combo_attack_command();
        assert!(!accepted);
        assert!(chain.is_next_pre_input, "Bow mode should set is_next_pre_input if time exceeds end_time");

        chain.cur_combo_index = 0;
        chain.combo_process();
        assert_eq!(chain.cur_combo_index, 1, "Next pre-input should trigger first combo hit automatically");
        assert!(!chain.is_next_pre_input);
    }
}
