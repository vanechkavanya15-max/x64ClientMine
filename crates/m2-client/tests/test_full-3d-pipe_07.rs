#[path = "../src/full_3d_pipe_07.rs"]
mod full_3d_pipe_07;

use full_3d_pipe_07::wgpu_fog_manager::{FogMode, WgpuFogManager};

#[test]
fn test_default_parameters() {
    let fog_mgr = WgpuFogManager::new();
    // Default values expected by standard client
    assert_eq!(fog_mgr.get_min_fog(), 2400.0);
    assert_eq!(fog_mgr.get_sight_range(), 25600.0);
    assert_eq!(fog_mgr.get_density(), 0.001);
    assert_eq!(fog_mgr.get_mode(), FogMode::None);
    assert_eq!(fog_mgr.get_color(), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_set_min_fog_constrained() {
    let mut fog_mgr = WgpuFogManager::new();
    fog_mgr.set_sight_range(10000.0);
    
    // Setting min_fog normally
    fog_mgr.set_min_fog(5000.0);
    assert_eq!(fog_mgr.get_min_fog(), 5000.0);
    
    // Setting min_fog higher than sight_range should clamp to sight_range
    fog_mgr.set_min_fog(15000.0);
    assert_eq!(fog_mgr.get_min_fog(), 10000.0);
}

#[test]
fn test_set_sight_range_constrained() {
    let mut fog_mgr = WgpuFogManager::new();
    fog_mgr.set_min_fog(8000.0);
    fog_mgr.set_sight_range(12000.0);
    assert_eq!(fog_mgr.get_sight_range(), 12000.0);
    assert_eq!(fog_mgr.get_min_fog(), 8000.0);
    
    // Setting sight_range lower than min_fog should clamp min_fog to sight_range
    fog_mgr.set_sight_range(4000.0);
    assert_eq!(fog_mgr.get_sight_range(), 4000.0);
    assert_eq!(fog_mgr.get_min_fog(), 4000.0);
}

#[test]
fn test_fog_mode_change() {
    let mut fog_mgr = WgpuFogManager::new();
    
    fog_mgr.set_mode(FogMode::Linear);
    assert_eq!(fog_mgr.get_mode(), FogMode::Linear);
    
    fog_mgr.set_mode(FogMode::Exp2);
    assert_eq!(fog_mgr.get_mode(), FogMode::Exp2);
    
    fog_mgr.set_mode(FogMode::None);
    assert_eq!(fog_mgr.get_mode(), FogMode::None);
}

#[test]
fn test_terrain_uniforms_export() {
    let mut fog_mgr = WgpuFogManager::new();
    fog_mgr.set_mode(FogMode::Linear);
    fog_mgr.set_min_fog(2000.0);
    fog_mgr.set_sight_range(8000.0);
    fog_mgr.set_density(0.005);
    
    let params = fog_mgr.get_shader_params();
    // Requirements for TerrainUniforms: [fog_start, fog_end, density, mode]
    assert_eq!(params[0], 2000.0);
    assert_eq!(params[1], 8000.0);
    assert_eq!(params[2], 0.005);
    assert_eq!(params[3], 1.0); // Linear
    
    fog_mgr.set_mode(FogMode::Exp2);
    let params2 = fog_mgr.get_shader_params();
    assert_eq!(params2[3], 2.0); // Exp2
}
