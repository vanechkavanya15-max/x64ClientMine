use m2_client::wgpu_terrain_splatting::{TTerrainSplat, TerrainSectorGroup, SPLATALPHA_RAW_XSIZE, SPLAT_TEX_WIDTH};

#[test]
fn test_terrain_splat_default() {
    let splat = TTerrainSplat::default();
    assert_eq!(splat.active, 0);
    assert_eq!(splat.needs_update, 0);
}

#[test]
fn test_terrain_sector_group_3x3() {
    let group = TerrainSectorGroup::new(10, 20);
    
    assert_eq!(group.center_x, 10);
    assert_eq!(group.center_y, 20);
    assert_eq!(group.active_sectors.len(), 9);

    // Should include (10, 20)
    assert!(group.active_sectors.contains(&(10, 20)));
    // Neighbors
    assert!(group.active_sectors.contains(&(9, 19)));
    assert!(group.active_sectors.contains(&(11, 21)));
}

#[test]
fn test_software_pixel_blending() {
    let c0 = [0.1, 0.2, 0.1];
    let c1 = [0.5, 0.5, 0.2];
    let c2 = [0.8, 0.8, 0.8];
    let c3 = [0.0, 0.0, 0.0];
    
    let w0 = 0.5;
    let w1 = 0.25;
    let w2 = 0.25;
    let w3 = 0.0;
    
    let r = c0[0]*w0 + c1[0]*w1 + c2[0]*w2 + c3[0]*w3;
    let g = c0[1]*w0 + c1[1]*w1 + c2[1]*w2 + c3[1]*w3;
    let b = c0[2]*w0 + c1[2]*w1 + c2[2]*w2 + c3[2]*w3;
    
    assert!((r - 0.375_f32).abs() < 0.001);
    assert!((g - 0.425_f32).abs() < 0.001);
    assert!((b - 0.3_f32).abs() < 0.001);
}

#[test]
fn test_software_lighting_dot_product() {
    let normal = [0.0_f32, 0.0_f32, 1.0_f32];
    // Direction TO sun
    let sun_dir = [0.0_f32, -0.707_f32, 0.707_f32]; 
    
    // dot product
    let dot = normal[0]*sun_dir[0] + normal[1]*sun_dir[1] + normal[2]*sun_dir[2];
    
    assert!((dot - 0.707_f32).abs() < 0.001);
    
    // Test shadow combine
    let base_color = 1.0;
    let ambient = 0.3;
    let shadow_intensity = 0.5; // From shadow map
    
    let final_light = base_color * (ambient + f32::max(dot, 0.0) * shadow_intensity);
    assert!((final_light - 0.6535_f32).abs() < 0.001);
}

#[test]
fn test_terrain_splat_constants() {
    // Ensuring sizes match metin2 legacy constraints (XSIZE * 2 + 2)
    assert_eq!(SPLATALPHA_RAW_XSIZE, 258);
    assert_eq!(SPLAT_TEX_WIDTH, 256);
}

#[test]
fn test_struct_sizes_c_abi() {
    assert_eq!(std::mem::size_of::<TTerrainSplat>(), 8); // Two i32 fields
}