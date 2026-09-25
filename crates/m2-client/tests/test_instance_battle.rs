use m2_client::instance_battle::InstanceBattle;

#[test]
fn test_integration_instance_battle() {
    let instance = InstanceBattle::new();
    assert!(instance.is_ready());
}

#[test]
fn test_combo_system_initialization() {
    let instance = InstanceBattle::new();
    assert_eq!(instance.cur_combo_index, 0);
    assert_eq!(instance.is_pre_input, false);
}

#[test]
fn test_input_combo_attack_command_first_hit() {
    let mut instance = InstanceBattle::new();
    let result = instance.input_combo_attack_command(45.0, true);
    
    assert!(result);
    assert_eq!(instance.atk_dir_rot, 45.0);
    assert_eq!(instance.cur_combo_index, 1);
    assert_eq!(instance.is_pre_input, false);
}

#[test]
fn test_input_combo_attack_command_pre_input() {
    let mut instance = InstanceBattle::new();
    
    // First hit -> starts combo
    instance.input_combo_attack_command(45.0, true);
    
    // Second hit with combo time data -> sets pre_input
    let result = instance.input_combo_attack_command(90.0, true);
    
    assert!(result);
    assert_eq!(instance.atk_dir_rot, 90.0);
    assert_eq!(instance.cur_combo_index, 1); // doesn't increment yet
    assert_eq!(instance.is_pre_input, true);
}

#[test]
fn test_combo_process_next_combo() {
    let mut instance = InstanceBattle::new();
    
    instance.input_combo_attack_command(0.0, true);
    instance.input_combo_attack_command(0.0, true); // sets pre_input
    
    // Process combo where elapsed time > next combo time
    instance.combo_process(2.0, 1.0, true, false);
    
    assert_eq!(instance.cur_combo_index, 2);
    assert_eq!(instance.is_pre_input, false);
}

#[test]
fn test_combo_sequence_end() {
    let mut instance = InstanceBattle::new();
    instance.combo_type = 0; // max_combo = 4
    
    instance.cur_combo_index = 4;
    instance.run_next_combo();
    
    // Should clear combo
    assert_eq!(instance.cur_combo_index, 0);
    assert_eq!(instance.is_pre_input, false);
}

#[test]
fn test_process_normal_attack_huge_race_in_range() {
    let instance = InstanceBattle::new();
    
    let result = instance.process_normal_attack(
        (0.0, 0.0, 0.0),
        0.0,
        (0.0, 400.0, 0.0),
        true, // is huge
        1.0,
        500.0,
        90.0
    );
    
    assert!(result);
}

#[test]
fn test_process_normal_attack_huge_race_out_of_range() {
    let instance = InstanceBattle::new();
    
    let result = instance.process_normal_attack(
        (0.0, 0.0, 0.0),
        0.0,
        (0.0, 600.0, 0.0),
        true, // is huge
        1.0,
        500.0,
        90.0
    );
    
    // Out of max distance limit (500 * 500)
    assert!(!result);
}

#[test]
fn test_process_normal_attack_normal_race_out_of_range() {
    let instance = InstanceBattle::new();
    
    let result = instance.process_normal_attack(
        (0.0, 0.0, 0.0),
        0.0,
        (0.0, 400.0, 0.0),
        false, // not huge
        1.0,
        500.0,
        90.0
    );
    
    // Out of max distance limit (300 * 300)
    assert!(!result);
}

#[test]
fn test_process_normal_attack_angle_miss() {
    let instance = InstanceBattle::new();
    
    let result = instance.process_normal_attack(
        (0.0, 0.0, 0.0),
        90.0, // facing X axis
        (0.0, 100.0, 0.0), // victim on Y axis
        false,
        1.0,
        200.0,
        45.0 // narrow hit cone
    );
    
    // Victim is outside the hit cone
    assert!(!result);
}

#[test]
fn test_process_normal_attack_reach_miss() {
    let instance = InstanceBattle::new();
    
    let result = instance.process_normal_attack(
        (0.0, 0.0, 0.0),
        0.0, // facing Y axis
        (0.0, 150.0, 0.0), // victim on Y axis at dist 150
        false,
        1.0,
        100.0, // attack radius only 100
        90.0
    );
    
    // Victim is outside the weapon reach
    assert!(!result);
}
