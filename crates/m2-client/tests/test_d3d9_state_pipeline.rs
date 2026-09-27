use m2_client::d3d9_state_pipeline::{StateManager, D3DRS_ALPHABLENDENABLE, D3DRS_ZENABLE};

#[test]
fn test_render_state_cache() {
    let mut state_manager = StateManager::new();

    // Set state that hasn't been set before -> should return true
    assert_eq!(state_manager.set_render_state(D3DRS_ZENABLE, 1), true);
    
    // Set same state -> should return false (no redundant call)
    assert_eq!(state_manager.set_render_state(D3DRS_ZENABLE, 1), false);

    // Set different value -> should return true
    assert_eq!(state_manager.set_render_state(D3DRS_ZENABLE, 0), true);
}

#[test]
fn test_render_state_out_of_bounds() {
    let mut state_manager = StateManager::new();

    // Out of bounds state index should be ignored and return false
    assert_eq!(state_manager.set_render_state(256, 1), false);
    assert_eq!(state_manager.set_render_state(1000, 1), false);
}

#[test]
fn test_sampler_state_cache() {
    let mut state_manager = StateManager::new();

    // Set state that hasn't been set before -> should return true
    assert_eq!(state_manager.set_sampler_state(0, 5, 2), true);
    
    // Set same state -> should return false (no redundant call)
    assert_eq!(state_manager.set_sampler_state(0, 5, 2), false);

    // Set different value -> should return true
    assert_eq!(state_manager.set_sampler_state(0, 5, 3), true);
}

#[test]
fn test_sampler_state_out_of_bounds() {
    let mut state_manager = StateManager::new();

    // Out of bounds stage
    assert_eq!(state_manager.set_sampler_state(8, 0, 1), false);
    
    // Out of bounds state
    assert_eq!(state_manager.set_sampler_state(0, 16, 1), false);
}

#[test]
fn test_texture_stage_state_cache() {
    let mut state_manager = StateManager::new();

    // Set state that hasn't been set before -> should return true
    assert_eq!(state_manager.set_texture_stage_state(1, 10, 4), true);
    
    // Set same state -> should return false (no redundant call)
    assert_eq!(state_manager.set_texture_stage_state(1, 10, 4), false);

    // Set different value -> should return true
    assert_eq!(state_manager.set_texture_stage_state(1, 10, 5), true);
}

#[test]
fn test_texture_stage_state_out_of_bounds() {
    let mut state_manager = StateManager::new();

    // Out of bounds stage
    assert_eq!(state_manager.set_texture_stage_state(8, 0, 1), false);
    
    // Out of bounds state
    assert_eq!(state_manager.set_texture_stage_state(0, 32, 1), false);
}

#[test]
fn test_default_values_are_zero() {
    let mut state_manager = StateManager::new();

    // Because default is 0, setting to 0 should return false (no change)
    assert_eq!(state_manager.set_render_state(D3DRS_ALPHABLENDENABLE, 0), false);
    
    // Changing to 1 should return true
    assert_eq!(state_manager.set_render_state(D3DRS_ALPHABLENDENABLE, 1), true);
}
