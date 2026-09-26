#[path = "../src/atlas_map_scaler_v2.rs"]
mod atlas_map_scaler_v2;

use atlas_map_scaler_v2::{m2_atlas_calc_pos, AtlasMapScaler, AtlasPosition};

#[test]
fn test_ffi_m2_atlas_calc_pos() {
    // A 1024x1024 map on a 512x512 UI should scale by 0.5
    // Global point (512, 512) -> Atlas point (256, 256)
    let pos = m2_atlas_calc_pos(512.0, 512.0, 1024.0, 1024.0);
    assert_eq!(pos, AtlasPosition { x: 256.0, y: 256.0 });

    // A 2048x1024 map on a 512x512 UI
    // Uniform scale is min(512/2048, 512/1024) = min(0.25, 0.5) = 0.25
    // The scaled map is 2048*0.25 = 512, 1024*0.25 = 256
    // Offsets: X = (512 - 512)/2 = 0, Y = (512 - 256)/2 = 128
    // Global point (1024, 512) -> Atlas point (1024*0.25 + 0, 512*0.25 + 128) -> (256, 256)
    let pos = m2_atlas_calc_pos(1024.0, 512.0, 2048.0, 1024.0);
    assert_eq!(pos, AtlasPosition { x: 256.0, y: 256.0 });
}

#[test]
fn test_atlas_map_scaler_uniform_scale() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 1024.0, 2048.0);
    // uniform scale = min(512/1024, 512/2048) = min(0.5, 0.25) = 0.25
    assert_eq!(scaler.get_atlas_uniform_scale(), 0.25);
}

#[test]
fn test_atlas_map_scaler_offsets() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 1024.0, 2048.0);
    // uniform scale = 0.25
    // scaled_width = 1024 * 0.25 = 256
    // scaled_height = 2048 * 0.25 = 512
    // offset_x = (512 - 256) / 2 = 128
    // offset_y = (512 - 512) / 2 = 0
    assert_eq!(scaler.atlas_offset_x, 128.0);
    assert_eq!(scaler.atlas_offset_y, 0.0);
}

#[test]
fn test_global_to_atlas_position() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 1024.0, 2048.0);
    // uniform scale = 0.25, offset_x = 128, offset_y = 0
    
    let (ax, ay) = scaler.global_position_to_atlas_position(512.0, 1024.0);
    // ax = 512 * 0.25 + 128 = 128 + 128 = 256
    // ay = 1024 * 0.25 + 0 = 256
    assert_eq!(ax, 256.0);
    assert_eq!(ay, 256.0);
}

#[test]
fn test_atlas_to_global_position() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 1024.0, 2048.0);
    // uniform scale = 0.25, offset_x = 128, offset_y = 0
    // reverse scale = 4.0
    
    let (gx, gy) = scaler.atlas_position_to_global_position(256.0, 256.0);
    // gx = (256 - 128) * 4.0 = 128 * 4.0 = 512.0
    // gy = (256 - 0) * 4.0 = 256 * 4.0 = 1024.0
    assert_eq!(gx, 512.0);
    assert_eq!(gy, 1024.0);
}

#[test]
fn test_roundtrip_conversion() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 3141.0, 5926.0);
    let original_gx = 1234.0;
    let original_gy = 4321.0;
    
    let (ax, ay) = scaler.global_position_to_atlas_position(original_gx, original_gy);
    let (gx, gy) = scaler.atlas_position_to_global_position(ax, ay);
    
    assert!((original_gx - gx).abs() < 0.01);
    assert!((original_gy - gy).abs() < 0.01);
}

#[test]
fn test_zero_map_size() {
    let scaler = AtlasMapScaler::new(512.0, 512.0, 0.0, 0.0);
    assert_eq!(scaler.get_atlas_uniform_scale(), 1.0);
    assert_eq!(scaler.atlas_offset_x, 0.0);
    assert_eq!(scaler.atlas_offset_y, 0.0);
    
    let (ax, ay) = scaler.global_position_to_atlas_position(100.0, 200.0);
    assert_eq!(ax, 100.0);
    assert_eq!(ay, 200.0);
    
    let (gx, gy) = scaler.atlas_position_to_global_position(100.0, 200.0);
    assert_eq!(gx, 100.0);
    assert_eq!(gy, 200.0);
}

#[test]
fn test_setters() {
    let mut scaler = AtlasMapScaler::default();
    assert_eq!(scaler.atlas_image_size_x, 512.0);
    assert_eq!(scaler.atlas_image_size_y, 512.0);
    assert_eq!(scaler.atlas_max_x, 0.0);
    assert_eq!(scaler.atlas_max_y, 0.0);
    
    scaler.set_map_size(1024.0, 1024.0);
    assert_eq!(scaler.get_atlas_uniform_scale(), 0.5);
    
    scaler.set_image_size(256.0, 256.0);
    assert_eq!(scaler.get_atlas_uniform_scale(), 0.25);
}
