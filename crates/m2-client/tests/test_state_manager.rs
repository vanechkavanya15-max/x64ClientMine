#[path = "../src/state_manager.rs"]
mod state_manager;

use state_manager::{BlendMode, CullMode, DepthTest, StateManager};

#[test]
fn test_initial_state() {
    let sm = StateManager::new();
    assert_eq!(sm.get_blend_mode(), BlendMode::Opaque);
    assert_eq!(sm.get_depth_test(), DepthTest::ReadWrite);
    assert_eq!(sm.get_cull_mode(), CullMode::Back);
    assert_eq!(sm.is_wireframe(), false);
}

#[test]
fn test_set_blend_mode() {
    let mut sm = StateManager::new();
    
    // Setting to same state should return false (redundant)
    assert_eq!(sm.set_blend_mode(BlendMode::Opaque), false);
    
    // Setting to new state should return true
    assert_eq!(sm.set_blend_mode(BlendMode::AlphaBlend), true);
    assert_eq!(sm.get_blend_mode(), BlendMode::AlphaBlend);
    
    // Setting back
    assert_eq!(sm.set_blend_mode(BlendMode::Opaque), true);
    assert_eq!(sm.get_blend_mode(), BlendMode::Opaque);

    // Additive
    assert_eq!(sm.set_blend_mode(BlendMode::Additive), true);
    assert_eq!(sm.get_blend_mode(), BlendMode::Additive);
}

#[test]
fn test_set_depth_test() {
    let mut sm = StateManager::new();
    
    // Default is ReadWrite, so setting it should return false
    assert_eq!(sm.set_depth_test(DepthTest::ReadWrite), false);
    
    assert_eq!(sm.set_depth_test(DepthTest::ReadOnly), true);
    assert_eq!(sm.get_depth_test(), DepthTest::ReadOnly);
    
    assert_eq!(sm.set_depth_test(DepthTest::None), true);
    assert_eq!(sm.get_depth_test(), DepthTest::None);
    
    assert_eq!(sm.set_depth_test(DepthTest::None), false);
}

#[test]
fn test_set_cull_mode() {
    let mut sm = StateManager::new();
    
    // Default is Back
    assert_eq!(sm.set_cull_mode(CullMode::Back), false);
    
    assert_eq!(sm.set_cull_mode(CullMode::Front), true);
    assert_eq!(sm.get_cull_mode(), CullMode::Front);
    
    assert_eq!(sm.set_cull_mode(CullMode::None), true);
    assert_eq!(sm.get_cull_mode(), CullMode::None);
}

#[test]
fn test_set_wireframe() {
    let mut sm = StateManager::new();
    
    // Default is false
    assert_eq!(sm.set_wireframe(false), false);
    
    assert_eq!(sm.set_wireframe(true), true);
    assert_eq!(sm.is_wireframe(), true);
    
    assert_eq!(sm.set_wireframe(true), false);
}

#[test]
fn test_reset_to_defaults() {
    let mut sm = StateManager::new();
    
    sm.set_blend_mode(BlendMode::AlphaBlend);
    sm.set_depth_test(DepthTest::None);
    sm.set_cull_mode(CullMode::Front);
    sm.set_wireframe(true);
    
    sm.reset_to_defaults();
    
    assert_eq!(sm.get_blend_mode(), BlendMode::Opaque);
    assert_eq!(sm.get_depth_test(), DepthTest::ReadWrite);
    assert_eq!(sm.get_cull_mode(), CullMode::Back);
    assert_eq!(sm.is_wireframe(), false);
}
