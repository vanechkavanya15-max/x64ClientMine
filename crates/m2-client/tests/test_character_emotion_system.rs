#[path = "../src/character_emotion_system.rs"]
mod character_emotion_system;

use character_emotion_system::*;

#[test]
fn test_single_emotion() {
    let mut state = CharacterEmotionState::new(1, 0, true);
    state.act_emotion(NAME_CLAP);
    
    assert_eq!(state.current_motion, NAME_CLAP);
    assert_eq!(state.motion_loop, false);
}

#[test]
fn test_is_act_emotion() {
    let mut state = CharacterEmotionState::new(1, 0, true);
    
    state.act_emotion(NAME_KISS_WITH_WARRIOR);
    assert!(state.is_acting_emotion());
    
    state.act_emotion(NAME_FRENCH_KISS_WITH_SHAMAN);
    assert!(state.is_acting_emotion());
    
    state.act_emotion(NAME_CLAP);
    assert!(!state.is_acting_emotion());
}

#[test]
fn test_dual_emotion_distance() {
    let mut main_state = CharacterEmotionState::new(1, 0, true);
    main_state.cur_pixel_position = PixelPosition { x: 500.0, y: 500.0, z: 0.0 };
    main_state.is_waiting = true;

    let mut target_state = CharacterEmotionState::new(2, 1, false);
    target_state.cur_pixel_position = PixelPosition { x: 500.0, y: 600.0, z: 0.0 };
    target_state.is_waiting = true;

    // act_dual_emotion moves main_state towards target_state so distance is exactly 100.0
    // Currently distance is 100.0 (600 - 500)
    main_state.act_dual_emotion(&mut target_state, NAME_KISS_START as u16, NAME_KISS_START as u16, 1000);

    let dst_pos = main_state.tcp_move_after_func.pos_dst;
    
    // Direction is main -> target = (0, -100) -> normalized (0, -1)
    // Destination = target + dir * 100 = (500, 600) + (0, -1) * 100 = (500, 500)
    assert_eq!(dst_pos.x, 500.0);
    assert_eq!(dst_pos.y, 500.0);
    
    assert_eq!(main_state.tcp_move_after_func.e_func, FUNC_EMOTION);
    let combined_motion = ((NAME_KISS_START as u32) << 16) | (NAME_KISS_START as u32);
    assert_eq!(main_state.tcp_move_after_func.u_arg, combined_motion);
    assert_eq!(main_state.tcp_move_after_func.u_arg_expanded, 2); // target vid
}

#[test]
fn test_process_function_emotion() {
    let mut main_state = CharacterEmotionState::new(1, 0, true); // race 0 -> job 0
    main_state.is_walking = true;
    
    let mut target_state = CharacterEmotionState::new(2, 3, false); // race 3 -> job 3
    target_state.is_walking = true;
    
    let pos_dst = PixelPosition { x: 100.0, y: 100.0, z: 0.0 };
    let combined_motion = ((NAME_KISS_START as u32) << 16) | (NAME_KISS_START as u32);
    
    main_state.process_function_emotion(combined_motion, 2, pos_dst, &mut target_state);
    
    assert_eq!(main_state.is_walking, false);
    assert_eq!(target_state.is_walking, false);
    assert_eq!(main_state.tcp_state_enabled, true);
    assert_eq!(target_state.tcp_state_enabled, true);
    assert_eq!(main_state.cur_pixel_position.x, 100.0);
    
    // Main state animates with target's job offset
    assert_eq!(main_state.current_motion, NAME_KISS_START + 3);
    
    // Target state animates with main's job offset
    assert_eq!(target_state.current_motion, NAME_KISS_START + 0);
}

#[test]
fn test_race_to_job() {
    assert_eq!(race_to_job(0), 0); // Warrior (M)
    assert_eq!(race_to_job(1), 1); // Assassin (W)
    assert_eq!(race_to_job(2), 2); // Sura (M)
    assert_eq!(race_to_job(3), 3); // Shaman (W)
    assert_eq!(race_to_job(4), 0); // Warrior (W)
    assert_eq!(race_to_job(5), 1); // Assassin (M)
    assert_eq!(race_to_job(6), 2); // Sura (W)
    assert_eq!(race_to_job(7), 3); // Shaman (M)
}
