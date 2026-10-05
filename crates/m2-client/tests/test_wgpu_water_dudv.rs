use m2_client::wgpu_water_dudv_distortion::{
    calculate_wave_offset, WaterDudvUniforms,
};
use bytemuck::bytes_of;

#[test]
fn test_dudv_uniforms_defaults() {
    let uniforms = WaterDudvUniforms::default();
    
    assert_eq!(uniforms.time, 0.0);
    assert_eq!(uniforms.wave_speed, 1.0);
    assert_eq!(uniforms.wave_scale, 0.1);
    assert_eq!(uniforms.refraction_strength, 0.05);
    
    // Check array values directly
    assert_eq!(uniforms.wind_dir_1[0], 1.0);
    assert_eq!(uniforms.wind_dir_1[1], 0.5);
    
    assert_eq!(uniforms.wind_dir_2[0], -0.5);
    assert_eq!(uniforms.wind_dir_2[1], 1.0);
    
    assert_eq!(uniforms.water_depth_factor, 10.0);
}

#[test]
fn test_calculate_wave_offset() {
    let wind_dir = [1.0, 0.5];
    let speed = 2.0;
    
    let time_0 = 0.0;
    let offset_0 = calculate_wave_offset(time_0, speed, wind_dir);
    assert_eq!(offset_0, [0.0, 0.0]);
    
    let time_1 = 1.0;
    let offset_1 = calculate_wave_offset(time_1, speed, wind_dir);
    assert_eq!(offset_1, [2.0, 1.0]);
    
    let time_2 = 2.5;
    let offset_2 = calculate_wave_offset(time_2, speed, wind_dir);
    assert_eq!(offset_2, [5.0, 2.5]);
}

#[test]
fn test_uniforms_alignment_and_casting() {
    // Assert 16-byte alignment
    assert_eq!(std::mem::align_of::<WaterDudvUniforms>(), 16);
    
    // Total 19 floats + 3 padding floats = 22 floats. 
    // Wait, let's count:
    // time (1) + wave_speed (1) + wave_scale (1) + refraction_strength (1) = 4
    // wind_dir_1 (2) + wind_dir_2 (2) = 4
    // sun_direction (4)
    // water_color_shallow (4)
    // water_color_deep (4)
    // water_depth_factor (1) + _pad (3) = 4
    // Total = 24 floats.
    // 24 * 4 bytes = 96 bytes.
    // Ensure size is a multiple of 16 for bytemuck casts.
    assert_eq!(std::mem::size_of::<WaterDudvUniforms>(), 96);
    
    let default_uniforms = WaterDudvUniforms::default();
    let bytes = bytes_of(&default_uniforms);
    
    assert_eq!(bytes.len(), 96);
    
    // Validate first float (time = 0.0)
    let time_bytes = &bytes[0..4];
    assert_eq!(time_bytes, 0.0f32.to_ne_bytes());
}

#[test]
fn test_uniform_modification() {
    let mut uniforms = WaterDudvUniforms::default();
    uniforms.time = 5.0;
    uniforms.wind_dir_1 = [0.0, 1.0];
    
    assert_eq!(uniforms.time, 5.0);
    assert_eq!(uniforms.wind_dir_1, [0.0, 1.0]);
}
