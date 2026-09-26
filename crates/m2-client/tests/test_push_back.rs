#[path = "../src/push_back.rs"]
mod push_back;

use push_back::{MobClass, Position, PushBackState};

#[test]
fn test_push_back_target_position() {
    let mut state = PushBackState::new(Position { x: 0.0, y: 0.0 });
    let res = state.start_pushback(1.0, 0.0, 150.0, 200.0, MobClass::Normal);
    
    assert!(res.is_ok());
    assert!(state.active);
    assert_eq!(state.dest_pos.x, 150.0);
    assert_eq!(state.dest_pos.y, 0.0);
}

#[test]
fn test_no_push_back_for_boss_and_huge() {
    let mut state_boss = PushBackState::new(Position { x: 10.0, y: 10.0 });
    let res_boss = state_boss.start_pushback(1.0, 0.0, 150.0, 200.0, MobClass::Boss);
    
    assert!(res_boss.is_ok());
    assert!(!state_boss.active);
    assert_eq!(state_boss.current_pos.x, 10.0);
    assert_eq!(state_boss.current_pos.y, 10.0);

    let mut state_huge = PushBackState::new(Position { x: 10.0, y: 10.0 });
    let res_huge = state_huge.start_pushback(1.0, 0.0, 150.0, 200.0, MobClass::Huge);
    
    assert!(res_huge.is_ok());
    assert!(!state_huge.active);
    assert_eq!(state_huge.current_pos.x, 10.0);
    assert_eq!(state_huge.current_pos.y, 10.0);
}

#[test]
fn test_push_back_interpolation() {
    let mut state = PushBackState::new(Position { x: 0.0, y: 0.0 });
    state.start_pushback(0.0, 1.0, 100.0, 10.0, MobClass::Normal).unwrap();

    assert!(state.active);
    assert_eq!(state.current_pos.x, 0.0);
    assert_eq!(state.current_pos.y, 0.0);

    state.update(5.0);
    assert!(state.active);
    assert_eq!(state.current_pos.x, 0.0);
    assert_eq!(state.current_pos.y, 50.0); // 50% z 100.0

    state.update(5.0);
    assert!(!state.active);
    assert_eq!(state.current_pos.x, 0.0);
    assert_eq!(state.current_pos.y, 100.0); // 100% z 100.0

    // Po zakonczeniu odrzutu aktualizacja nic nie zmienia
    state.update(10.0);
    assert!(!state.active);
    assert_eq!(state.current_pos.x, 0.0);
    assert_eq!(state.current_pos.y, 100.0); 
}

#[test]
fn test_push_back_zero_vector() {
    let mut state = PushBackState::new(Position { x: 0.0, y: 0.0 });
    let res = state.start_pushback(0.0, 0.0, 150.0, 200.0, MobClass::Normal);
    
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Wektor ciosu nie moze miec dlugosci 0");
    assert!(!state.active);
}

#[test]
fn test_push_back_zero_duration() {
    let mut state = PushBackState::new(Position { x: 0.0, y: 0.0 });
    let res = state.start_pushback(1.0, 0.0, 150.0, 0.0, MobClass::Normal);
    
    assert!(res.is_ok());
    assert!(!state.active);
    assert_eq!(state.current_pos.x, 150.0);
    assert_eq!(state.current_pos.y, 0.0);
}
