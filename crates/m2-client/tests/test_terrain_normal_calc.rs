use m2_client::terrain_normal_calc::calculate_terrain_normal;

#[test]
fn test_terrain_normal_flat() {
    let cell_size = 200.0;
    // Flat terrain means all heights are equal.
    let normal = calculate_terrain_normal(100.0, 100.0, 100.0, 100.0, cell_size);
    // dx = 0, dy = 0, nz = 1.0 => (0, 0, 1) => (0, 0, 127) after clamping
    assert_eq!(normal, [0, 0, 127]);
}

#[test]
fn test_terrain_normal_slope_x_positive() {
    let cell_size = 100.0;
    // h_left = 0, h_right = 200, dx = (200 - 0) / (2 * 100) = 1.0
    // h_down = 100, h_up = 100, dy = 0
    // nx = -1.0, ny = 0, nz = 1.0
    // len = sqrt(1 + 0 + 1) = sqrt(2)
    // normal_x = -1 / sqrt(2) ≈ -0.707
    // packed_x = (-0.707 * 127) ≈ -89.78 => -89 or -90
    let normal = calculate_terrain_normal(0.0, 200.0, 100.0, 100.0, cell_size);
    let expected_x = (-(1.0_f32 / 2.0_f32.sqrt()) * 127.0).clamp(-128.0, 127.0) as i8;
    let expected_z = ((1.0_f32 / 2.0_f32.sqrt()) * 127.0).clamp(-128.0, 127.0) as i8;
    assert_eq!(normal[0], expected_x);
    assert_eq!(normal[1], 0);
    assert_eq!(normal[2], expected_z);
}

#[test]
fn test_terrain_normal_slope_y_positive() {
    let cell_size = 100.0;
    // h_left = 100, h_right = 100, dx = 0
    // h_down = 0, h_up = 200, dy = 1.0
    // nx = 0, ny = -1.0, nz = 1.0
    let normal = calculate_terrain_normal(100.0, 100.0, 0.0, 200.0, cell_size);
    let expected_y = (-(1.0_f32 / 2.0_f32.sqrt()) * 127.0).clamp(-128.0, 127.0) as i8;
    let expected_z = ((1.0_f32 / 2.0_f32.sqrt()) * 127.0).clamp(-128.0, 127.0) as i8;
    assert_eq!(normal[0], 0);
    assert_eq!(normal[1], expected_y);
    assert_eq!(normal[2], expected_z);
}

#[test]
fn test_terrain_normal_steep_slope_clamp() {
    let cell_size = 10.0;
    // dx = 1000 / 20 = 50
    // nx = -50, nz = 1
    // normal_x ≈ -1.0, normal_z ≈ 0.02
    // packed_x should be -127 or -128
    let normal = calculate_terrain_normal(0.0, 1000.0, 100.0, 100.0, cell_size);
    
    let nx = -50.0_f32;
    let ny = 0.0_f32;
    let nz = 1.0_f32;
    let len = (nx*nx + ny*ny + nz*nz).sqrt();
    
    let expected_x = ((nx / len) * 127.0).clamp(-128.0, 127.0) as i8;
    let expected_y = 0;
    let expected_z = ((nz / len) * 127.0).clamp(-128.0, 127.0) as i8;

    assert_eq!(normal[0], expected_x);
    assert_eq!(normal[1], expected_y);
    assert_eq!(normal[2], expected_z);
}

#[test]
fn test_normalized_vector_length() {
    let cell_size = 200.0;
    let normal = calculate_terrain_normal(0.0, 400.0, 100.0, 800.0, cell_size);
    
    let fnx = normal[0] as f32 / 127.0;
    let fny = normal[1] as f32 / 127.0;
    let fnz = normal[2] as f32 / 127.0;
    
    let len = (fnx*fnx + fny*fny + fnz*fnz).sqrt();
    
    // Allow a small margin of error due to quantization to i8
    assert!((len - 1.0).abs() < 0.05, "Length of vector after quantization should be close to 1.0 (got {})", len);
}

#[test]
fn test_terrain_normal_extreme_values() {
    // Handling very large numbers that could cause floating point issues
    let cell_size = 200.0;
    let normal = calculate_terrain_normal(-1e10, 1e10, -1e10, 1e10, cell_size);
    
    // nx and ny will be extremely dominant, nz will be negligible
    // Length is dominated by nx and ny
    // Since nx = -1e8, ny = -1e8
    // Normal x and y should be around -1/sqrt(2) = -0.707
    // Quantized they will be around -89
    
    let dx = (1e10 - (-1e10)) / 400.0;
    let dy = (1e10 - (-1e10)) / 400.0;
    let nx = -dx as f32;
    let ny = -dy as f32;
    let nz = 1.0_f32;
    let len = (nx*nx + ny*ny + nz*nz).sqrt();
    
    let expected_x = ((nx / len) * 127.0).clamp(-128.0, 127.0) as i8;
    let expected_y = ((ny / len) * 127.0).clamp(-128.0, 127.0) as i8;
    let expected_z = ((nz / len) * 127.0).clamp(-128.0, 127.0) as i8;
    
    assert_eq!(normal[0], expected_x);
    assert_eq!(normal[1], expected_y);
    assert_eq!(normal[2], expected_z);
}
