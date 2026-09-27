use m2_client::granny_mesh_decompressor::*;
use std::os::raw::c_void;
use std::ptr;

fn create_mock_mesh(
    vertices: &mut [GrannyPwnt3432Vertex],
    indices16: &mut [u16],
) -> GrannyMesh {
    let mut vertex_data = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: vertices.len() as i32,
        vertices: vertices.as_mut_ptr() as *mut u8,
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mut topology = GrannyTriTopology {
        group_count: 0,
        groups: ptr::null_mut(),
        index_count: 0,
        indices: ptr::null_mut(),
        index16_count: indices16.len() as i32,
        indices16: indices16.as_mut_ptr(),
        vertex_to_vertex_count: 0,
        vertex_to_vertex_map: ptr::null_mut(),
        vertex_to_triangle_count: 0,
        vertex_to_triangle_map: ptr::null_mut(),
        side_to_neighbor_count: 0,
        side_to_neighbor_map: ptr::null_mut(),
        polygon_index_start_count: 0,
        polygon_index_starts: ptr::null_mut(),
    };

    // Need to allocate on heap or keep in scope, but for a synchronous test we can trick it by pinning in a box 
    // Actually we can't easily return a struct with pointers to locals unless we are careful in the test.
    // Let's rely on Box to keep them alive.
    
    GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vertex_data as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: &mut topology as *mut _,
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    }
}

#[test]
fn test_null_mesh_ptr() {
    let result = decompress_granny_mesh(ptr::null());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "raw_mesh_ptr is null");
}

#[test]
fn test_bone_weight_normalization_exact_1() {
    // 255 / 0 / 0 / 0 => 1.0
    let mut vertices = [GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [255, 0, 0, 0],
        bone_indices: [0, 0, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }];
    let mut indices16 = [0, 0, 0];

    let mut vd = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: vertices.len() as i32,
        vertices: vertices.as_mut_ptr() as *mut u8,
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mut topo = GrannyTriTopology {
        group_count: 0,
        groups: ptr::null_mut(),
        index_count: 0,
        indices: ptr::null_mut(),
        index16_count: indices16.len() as i32,
        indices16: indices16.as_mut_ptr(),
        vertex_to_vertex_count: 0,
        vertex_to_vertex_map: ptr::null_mut(),
        vertex_to_triangle_count: 0,
        vertex_to_triangle_map: ptr::null_mut(),
        side_to_neighbor_count: 0,
        side_to_neighbor_map: ptr::null_mut(),
        polygon_index_start_count: 0,
        polygon_index_starts: ptr::null_mut(),
    };

    let mesh = GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vd as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: &mut topo as *mut _,
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    };

    let decompressed = decompress_granny_mesh(&mesh as *const _ as *const c_void).unwrap();
    let w = decompressed.bone_weights[0];
    assert_eq!(w, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(w.iter().sum::<f32>(), 1.0);
}

#[test]
fn test_bone_weight_normalization_half_half() {
    // 128 / 127 / 0 / 0
    let mut vertices = [GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [128, 127, 0, 0],
        bone_indices: [0, 1, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }];
    let mut indices16 = [0, 0, 0];

    let mut vd = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: vertices.len() as i32,
        vertices: vertices.as_mut_ptr() as *mut u8,
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mut topo = GrannyTriTopology {
        group_count: 0,
        groups: ptr::null_mut(),
        index_count: 0,
        indices: ptr::null_mut(),
        index16_count: indices16.len() as i32,
        indices16: indices16.as_mut_ptr(),
        vertex_to_vertex_count: 0,
        vertex_to_vertex_map: ptr::null_mut(),
        vertex_to_triangle_count: 0,
        vertex_to_triangle_map: ptr::null_mut(),
        side_to_neighbor_count: 0,
        side_to_neighbor_map: ptr::null_mut(),
        polygon_index_start_count: 0,
        polygon_index_starts: ptr::null_mut(),
    };

    let mesh = GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vd as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: &mut topo as *mut _,
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    };

    let decompressed = decompress_granny_mesh(&mesh as *const _ as *const c_void).unwrap();
    let w = decompressed.bone_weights[0];
    
    // sum is 255. 128 / 255, 127 / 255
    let w0 = 128.0 / 255.0;
    let w1 = 127.0 / 255.0;
    
    assert!((w[0] - w0).abs() < 1e-6);
    assert!((w[1] - w1).abs() < 1e-6);
    assert_eq!(w[2], 0.0);
    assert_eq!(w[3], 0.0);
    
    let sum: f32 = w.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6);
}

#[test]
fn test_bone_weight_normalization_zero() {
    // 0 / 0 / 0 / 0 => [1.0, 0, 0, 0] fallback
    let mut vertices = [GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [0, 0, 0, 0],
        bone_indices: [0, 0, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }];
    let mut indices16 = [0, 0, 0];

    let mut vd = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: vertices.len() as i32,
        vertices: vertices.as_mut_ptr() as *mut u8,
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mut topo = GrannyTriTopology {
        group_count: 0,
        groups: ptr::null_mut(),
        index_count: 0,
        indices: ptr::null_mut(),
        index16_count: indices16.len() as i32,
        indices16: indices16.as_mut_ptr(),
        vertex_to_vertex_count: 0,
        vertex_to_vertex_map: ptr::null_mut(),
        vertex_to_triangle_count: 0,
        vertex_to_triangle_map: ptr::null_mut(),
        side_to_neighbor_count: 0,
        side_to_neighbor_map: ptr::null_mut(),
        polygon_index_start_count: 0,
        polygon_index_starts: ptr::null_mut(),
    };

    let mesh = GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vd as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: &mut topo as *mut _,
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    };

    let decompressed = decompress_granny_mesh(&mesh as *const _ as *const c_void).unwrap();
    let w = decompressed.bone_weights[0];
    
    assert_eq!(w, [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_bounding_box_empty() {
    let empty_mesh = DecompressedMesh {
        vertex_count: 0,
        triangle_count: 0,
        positions: vec![],
        normals: vec![],
        uvs: vec![],
        bone_indices: vec![],
        bone_weights: vec![],
        indices: vec![],
    };

    let bb = calculate_bounding_box(&empty_mesh);
    assert_eq!(bb.0, [0.0, 0.0, 0.0]);
    assert_eq!(bb.1, [0.0, 0.0, 0.0]);
}

#[test]
fn test_bounding_box_positive_negative() {
    let mesh = DecompressedMesh {
        vertex_count: 3,
        triangle_count: 1,
        positions: vec![
            [-10.0, 5.0, 2.0],
            [10.0, -5.0, 2.0],
            [0.0, 0.0, 15.0],
        ],
        normals: vec![],
        uvs: vec![],
        bone_indices: vec![],
        bone_weights: vec![],
        indices: vec![],
    };

    let (min, max) = calculate_bounding_box(&mesh);
    assert_eq!(min, [-10.0, -5.0, 2.0]);
    assert_eq!(max, [10.0, 5.0, 15.0]);
}

#[test]
fn test_missing_data_errors() {
    let mut vd = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: 0,
        vertices: ptr::null_mut(), // null vertices
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mesh = GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vd as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: ptr::null_mut(), // null topology
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    };

    let result = decompress_granny_mesh(&mesh as *const _ as *const c_void);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "vertices array is null");
}

#[test]
fn test_index32_fallback() {
    let mut vertices = [GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [255, 0, 0, 0],
        bone_indices: [0, 0, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }];
    let mut indices32 = [0i32, 1, 2];

    let mut vd = GrannyVertexData {
        vertex_type: ptr::null_mut(),
        vertex_count: vertices.len() as i32,
        vertices: vertices.as_mut_ptr() as *mut u8,
        vertex_component_name_count: 0,
        vertex_component_names: ptr::null_mut(),
        vertex_annotation_set_count: 0,
        vertex_annotation_sets: ptr::null_mut(),
    };

    let mut topo = GrannyTriTopology {
        group_count: 0,
        groups: ptr::null_mut(),
        index_count: indices32.len() as i32,
        indices: indices32.as_mut_ptr(),
        index16_count: 0,
        indices16: ptr::null_mut(),
        vertex_to_vertex_count: 0,
        vertex_to_vertex_map: ptr::null_mut(),
        vertex_to_triangle_count: 0,
        vertex_to_triangle_map: ptr::null_mut(),
        side_to_neighbor_count: 0,
        side_to_neighbor_map: ptr::null_mut(),
        polygon_index_start_count: 0,
        polygon_index_starts: ptr::null_mut(),
    };

    let mesh = GrannyMesh {
        name: ptr::null(),
        primary_vertex_data: &mut vd as *mut _,
        morph_target_count: 0,
        morph_targets: ptr::null_mut(),
        primary_topology: &mut topo as *mut _,
        material_binding_count: 0,
        material_bindings: ptr::null_mut(),
        bone_binding_count: 0,
        bone_bindings: ptr::null_mut(),
        extended_data: [0; 24],
    };

    let decompressed = decompress_granny_mesh(&mesh as *const _ as *const c_void).unwrap();
    assert_eq!(decompressed.indices, vec![0u16, 1, 2]);
    assert_eq!(decompressed.triangle_count, 1);
}
