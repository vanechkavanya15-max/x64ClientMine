#[path = "../src/full_3d_pipe_07.rs"]
mod full_3d_pipe_07;

use full_3d_pipe_07::wgpu_fog_manager::{FogMode, WgpuFogManager};

#[test]
fn test_default_fog_manager() {
    let manager = WgpuFogManager::new();
    assert_eq!(manager.get_mode(), FogMode::None);
    assert_eq!(manager.get_color(), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(manager.get_min_fog(), 2400.0);
    assert_eq!(manager.get_sight_range(), 25600.0);
    assert_eq!(manager.get_density(), 0.001);
    
    let shader_params = manager.get_shader_params();
    assert_eq!(shader_params, [2400.0, 25600.0, 0.001, 0.0]);
}

#[test]
fn test_setters_and_getters() {
    let mut manager = WgpuFogManager::new();
    
    manager.set_mode(FogMode::Linear);
    assert_eq!(manager.get_mode(), FogMode::Linear);
    
    manager.set_color(0.5, 0.6, 0.7, 0.8);
    assert_eq!(manager.get_color(), [0.5, 0.6, 0.7, 0.8]);
    
    manager.set_density(0.005);
    assert_eq!(manager.get_density(), 0.005);
}

#[test]
fn test_fog_distance_constraints() {
    let mut manager = WgpuFogManager::new();
    
    manager.set_sight_range(5000.0);
    assert_eq!(manager.get_sight_range(), 5000.0);
    
    // Test setting min_fog greater than sight_range
    manager.set_min_fog(6000.0);
    assert_eq!(manager.get_min_fog(), 5000.0);
    
    manager.set_min_fog(2000.0);
    assert_eq!(manager.get_min_fog(), 2000.0);
    
    // Test setting sight_range smaller than min_fog
    manager.set_sight_range(1000.0);
    assert_eq!(manager.get_sight_range(), 1000.0);
    assert_eq!(manager.get_min_fog(), 1000.0);
}

#[test]
fn test_shader_params_output() {
    let mut manager = WgpuFogManager::new();
    
    manager.set_mode(FogMode::Exp2);
    manager.set_sight_range(10000.0);
    manager.set_min_fog(5000.0);
    manager.set_density(0.02);
    
    let shader_params = manager.get_shader_params();
    assert_eq!(shader_params, [5000.0, 10000.0, 0.02, 2.0]);
}
