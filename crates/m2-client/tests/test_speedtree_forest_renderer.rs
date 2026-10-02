use m2_client::speedtree_forest_renderer::{
    BranchVertex, LeafVertex, Material, TreeGeometry, WindMatrices, m2_speedtree_get_vertex_buffer, m2_speedtree_parse_geometry, clear_geometry_cache, NUM_WIND_MATRICES
};

#[test]
fn test_wind_matrices_initialization() {
    let mut wind = WindMatrices::default();
    
    assert_eq!(wind.wind_strength, 0.0);
    assert_eq!(wind.old_strength, 0.0);
    assert_eq!(wind.time_of_last_call, 0.0);
    assert_eq!(wind.matrices.len(), NUM_WIND_MATRICES);
    
    wind.wind_strength = 0.5;
    wind.setup_wind_matrices(1.0);
    
    assert_ne!(wind.matrices[0][0], 0.0);
    assert_eq!(wind.old_strength, 0.5);
    assert_eq!(wind.time_of_last_call, 1.0);
}

#[test]
fn test_parse_geometry_ffi() {
    clear_geometry_cache();
    
    // Creating a mock valid SPT block data (magic "SPT " + version + raw data)
    let mut spt_data = vec![];
    spt_data.extend_from_slice(b"SPT "); // magic
    spt_data.extend_from_slice(&1u32.to_le_bytes()); // version
    spt_data.extend_from_slice(&[0x01, 0x02, 0x03]); // dummy raw data
    
    let result = m2_speedtree_parse_geometry(1, spt_data.as_ptr(), spt_data.len() as u32);
    assert!(result, "Failed to parse geometry with valid SPT data");
    
    let mut vertex_ptr: *const BranchVertex = std::ptr::null();
    let mut vertex_count: u32 = 0;
    
    let result2 = m2_speedtree_get_vertex_buffer(1, &mut vertex_ptr, &mut vertex_count);
    assert!(result2, "Failed to get vertex buffer");
    assert!(!vertex_ptr.is_null());
    assert_eq!(vertex_count, 0, "No branches pushed in empty mock payload");
}

#[test]
fn test_parse_geometry_invalid_ffi() {
    let result = m2_speedtree_parse_geometry(2, std::ptr::null(), 0);
    assert!(!result, "Should fail with null pointer");
    
    // Creating an invalid SPT block
    let invalid_data = vec![0x00, 0x00];
    let result2 = m2_speedtree_parse_geometry(2, invalid_data.as_ptr(), invalid_data.len() as u32);
    assert!(!result2, "Should fail with invalid magic/size");
}

use m2_client::speedtree_forest_renderer::{WIND_SHADER_WGSL, LEAF_SHADER_WGSL, calculate_tree_lod_alpha, m2_speedtree_get_lod_alpha};

#[test]
fn test_wgsl_shader_compilation() {
    // Check that shader text is accessible and starts with correct syntax
    assert!(WIND_SHADER_WGSL.contains("struct WindMatrices"));
    assert!(WIND_SHADER_WGSL.contains("fn vs_main(model: VertexInput) -> VertexOutput"));
    assert!(WIND_SHADER_WGSL.contains("let wind_offset = (wind_matrix * pos).xyz - pos.xyz;"));
    
    assert!(LEAF_SHADER_WGSL.contains("struct CameraUniforms"));
    assert!(LEAF_SHADER_WGSL.contains("offset_pos = base_pos.xyz + (right * model.corner.x) + (up * model.corner.y)"));
}

#[test]
fn test_parse_geometry_real_binary() {
    clear_geometry_cache();
    
    let mut data = vec![];
    data.extend_from_slice(b"SPT "); // magic
    data.extend_from_slice(&1u32.to_le_bytes()); // version
    
    // num branches = 1
    data.extend_from_slice(&1u32.to_le_bytes());
    // 8 f32s for 1 branch vertex (pos 3, normal 3, uv 2)
    for _ in 0..8 {
        data.extend_from_slice(&1.5f32.to_le_bytes());
    }
    
    // num leaves = 1
    data.extend_from_slice(&1u32.to_le_bytes());
    // 8 f32s for 1 leaf vertex
    for _ in 0..8 {
        data.extend_from_slice(&2.5f32.to_le_bytes());
    }
    
    let result = m2_speedtree_parse_geometry(10, data.as_ptr(), data.len() as u32);
    assert!(result, "Failed to parse binary buffer");
    
    let mut vertex_ptr: *const BranchVertex = std::ptr::null();
    let mut vertex_count: u32 = 0;
    
    let result2 = m2_speedtree_get_vertex_buffer(10, &mut vertex_ptr, &mut vertex_count);
    assert!(result2, "Failed to get vertex buffer");
    assert_eq!(vertex_count, 1, "Expected 1 branch vertex");
    
    unsafe {
        let v = &*vertex_ptr;
        assert_eq!(v.position[0], 1.5);
    }
}

#[test]
fn test_lod_alpha_calculation() {
    let lod_close = calculate_tree_lod_alpha(1000.0);
    assert_eq!(lod_close.level, 0);
    assert_eq!(lod_close.alpha_3d, 1.0);
    assert_eq!(lod_close.alpha_billboard, 0.0);
    
    let lod_far = calculate_tree_lod_alpha(8000.0);
    assert_eq!(lod_far.level, 2);
    assert_eq!(lod_far.alpha_3d, 0.0);
    assert_eq!(lod_far.alpha_billboard, 1.0);
    
    let lod_crossfade = calculate_tree_lod_alpha(4500.0);
    assert_eq!(lod_crossfade.level, 1);
    assert!(lod_crossfade.alpha_3d > 0.0 && lod_crossfade.alpha_3d < 1.0);
    assert!(lod_crossfade.alpha_billboard > 0.0 && lod_crossfade.alpha_billboard < 1.0);
}

#[test]
fn test_lod_alpha_ffi() {
    let mut out_3d = 0.0;
    let mut out_bb = 0.0;
    
    let res = m2_speedtree_get_lod_alpha(1000.0, &mut out_3d, &mut out_bb);
    assert!(res);
    assert_eq!(out_3d, 1.0);
    assert_eq!(out_bb, 0.0);
    
    let res_null = m2_speedtree_get_lod_alpha(1000.0, std::ptr::null_mut(), std::ptr::null_mut());
    assert!(!res_null);
}
