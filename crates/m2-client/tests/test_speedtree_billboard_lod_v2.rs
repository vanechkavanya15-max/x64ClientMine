use m2_client::speedtree_billboard_lod_v2::*;

#[test]
fn test_calculate_speedtree_lod_3d() {
    let lod = calculate_speedtree_lod(1000.0);
    assert_eq!(lod.level, 0);
    assert_eq!(lod.alpha_3d, 1.0);
    assert_eq!(lod.alpha_billboard, 0.0);
    
    // Boundary < 3000
    let lod_boundary = calculate_speedtree_lod(2999.9);
    assert_eq!(lod_boundary.level, 0);
    assert_eq!(lod_boundary.alpha_3d, 1.0);
    assert_eq!(lod_boundary.alpha_billboard, 0.0);
}

#[test]
fn test_calculate_speedtree_lod_negative() {
    let lod = calculate_speedtree_lod(-500.0);
    assert_eq!(lod.level, 0);
    assert_eq!(lod.alpha_3d, 1.0);
    assert_eq!(lod.alpha_billboard, 0.0);
}

#[test]
fn test_calculate_speedtree_lod_crossfade() {
    let lod_mid = calculate_speedtree_lod(4500.0);
    assert_eq!(lod_mid.level, 1);
    // Factor = (4500 - 3000) / 3000 = 1500 / 3000 = 0.5
    assert_eq!(lod_mid.alpha_3d, 0.5);
    assert_eq!(lod_mid.alpha_billboard, 0.5);
    
    let lod_near = calculate_speedtree_lod(3000.0);
    assert_eq!(lod_near.level, 1);
    assert_eq!(lod_near.alpha_3d, 1.0);
    assert_eq!(lod_near.alpha_billboard, 0.0);
    
    let lod_far = calculate_speedtree_lod(6000.0);
    assert_eq!(lod_far.level, 1);
    assert_eq!(lod_far.alpha_3d, 0.0);
    assert_eq!(lod_far.alpha_billboard, 1.0);
}

#[test]
fn test_calculate_speedtree_lod_billboard() {
    let lod = calculate_speedtree_lod(7000.0);
    assert_eq!(lod.level, 2);
    assert_eq!(lod.alpha_3d, 0.0);
    assert_eq!(lod.alpha_billboard, 1.0);
    
    let lod_far = calculate_speedtree_lod(100000.0);
    assert_eq!(lod_far.level, 2);
    assert_eq!(lod_far.alpha_3d, 0.0);
    assert_eq!(lod_far.alpha_billboard, 1.0);
}

#[test]
fn test_ffi_m2_forest_get_lod_level() {
    assert_eq!(m2_forest_get_lod_level(1500.0), 0);
    assert_eq!(m2_forest_get_lod_level(3000.0), 1);
    assert_eq!(m2_forest_get_lod_level(4500.0), 1);
    assert_eq!(m2_forest_get_lod_level(6000.0), 1);
    assert_eq!(m2_forest_get_lod_level(8000.0), 2);
    assert_eq!(m2_forest_get_lod_level(-100.0), 0);
}

#[test]
fn test_generate_cross_billboard_geometry() {
    let center = [10.0, 20.0, 30.0];
    let width = 10.0;
    let height = 40.0;
    
    let vertices = generate_cross_billboard(center, width, height);
    assert_eq!(vertices.len(), 12);
    
    // Check UV mapping logic
    // Bottom left = [0.0, 1.0], Top Right = [1.0, 0.0]
    let uv_bl = [0.0, 1.0];
    let uv_tr = [1.0, 0.0];
    
    // First vertex of Quad 1, Triangle 1 should be bottom left
    assert_eq!(vertices[0].uv, uv_bl);
    
    // Fifth vertex (Quad 1, Triangle 2, second vertex) should be top right
    assert_eq!(vertices[4].uv, uv_tr);
    
    // Check heights (Z axis)
    // Bottom vertices should have Z = 30.0
    assert_eq!(vertices[0].position[2], 30.0);
    assert_eq!(vertices[1].position[2], 30.0);
    
    // Top vertices should have Z = 30.0 + 40.0 = 70.0
    assert_eq!(vertices[2].position[2], 70.0);
}
