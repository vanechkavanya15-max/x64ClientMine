#[path = "../src/actor_state_machine_core_v2.rs"]
mod actor_state_machine_core_v2;

use actor_state_machine_core_v2::{ActorState, ActorStateMachine};

#[test]
fn test_default_state() {
    let state = ActorState::default();
    assert_eq!(state, ActorState::Wait);
}

#[test]
fn test_state_machine_initialization() {
    let mut sm = ActorStateMachine::new();
    assert_eq!(sm.current_state(), ActorState::Wait);
    assert!(sm.can_move());
    assert!(sm.can_attack());
    
    // Check set_state and reset
    sm.set_state(ActorState::Knockdown);
    assert_eq!(sm.current_state(), ActorState::Knockdown);
    sm.set_main_actor(true);
    assert!(sm.is_main_actor());

    sm.change_state(ActorState::Stun).unwrap();
    assert_eq!(sm.current_state(), ActorState::Stun);
    assert!(!sm.can_move());
    
    sm.initialize_state_data();
    assert_eq!(sm.current_state(), ActorState::Wait);
    assert!(sm.can_move());
    assert!(!sm.is_main_actor());
}

#[test]
fn test_state_transitions() {
    let mut sm = ActorStateMachine::new();
    
    // Normal transitions
    assert!(sm.change_state(ActorState::Walk).is_ok());
    assert_eq!(sm.current_state(), ActorState::Walk);
    
    assert!(sm.change_state(ActorState::Run).is_ok());
    assert_eq!(sm.current_state(), ActorState::Run);
    
    assert!(sm.change_state(ActorState::Attack).is_ok());
    assert_eq!(sm.current_state(), ActorState::Attack);

    assert!(sm.change_state(ActorState::Fishing).is_ok());
    assert_eq!(sm.current_state(), ActorState::Fishing);

    assert!(sm.change_state(ActorState::Mining).is_ok());
    assert_eq!(sm.current_state(), ActorState::Mining);
    
    assert!(sm.change_state(ActorState::Dead).is_ok());
    assert_eq!(sm.current_state(), ActorState::Dead);
    
    // Dead characters cannot move or attack
    assert!(sm.change_state(ActorState::Walk).is_err());
    assert!(sm.change_state(ActorState::Attack).is_err());
}

#[test]
fn test_stun_condition() {
    let mut sm = ActorStateMachine::new();
    
    assert!(sm.change_state(ActorState::Stun).is_ok());
    assert_eq!(sm.current_state(), ActorState::Stun);
    
    // Stunned characters cannot move or attack
    assert!(sm.change_state(ActorState::Walk).is_err());
    assert!(sm.change_state(ActorState::Run).is_err());
    assert!(sm.change_state(ActorState::Attack).is_err());
    assert!(sm.change_state(ActorState::Combo).is_err());
    assert!(sm.change_state(ActorState::Skill).is_err());
    
    // But they can transition to damage or dead
    assert!(sm.change_state(ActorState::Damage).is_ok());
    assert!(sm.change_state(ActorState::Dead).is_ok());
}
