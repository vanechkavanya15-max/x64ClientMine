use m2_client::d3d9_terrain_mesh_renderer::{
    generate_patch_vertices, is_patch_in_frustum, TerrainLodLevel, TerrainPatchRenderData,
    TerrainVertex,
};

#[test]
fn test_terrain_lod_level_step_sizes() {
    assert_eq!(TerrainLodLevel::Lod0.step_size(), 1);
    assert_eq!(TerrainLodLevel::Lod1.step_size(), 2);
    assert_eq!(TerrainLodLevel::Lod2.step_size(), 4);
}

#[test]
fn test_generate_patch_vertices_byte_counts() {
    let mut height_map = [0.0; 17 * 17];
    let mut normal_map = [[0.0, 0.0, 1.0]; 17 * 17];
    let patch = TerrainPatchRenderData {
        patch_x: 0,
        patch_y: 0,
        height_map,
        normal_map,
        splat_texture_ids: [0, 0, 0, 0],
    };

    let vertex_size = std::mem::size_of::<TerrainVertex>();

    // Lod0: 16x16 grid, step 1 = 16 * 16 = 256 cells. Each cell has 2 triangles = 6 vertices. Total 1536 vertices.
    let bytes_lod0 = generate_patch_vertices(&patch, TerrainLodLevel::Lod0);
    assert_eq!(bytes_lod0.len(), 1536 * vertex_size);

    // Lod1: 16x16 grid, step 2 = 8 * 8 = 64 cells. Each cell has 2 triangles = 6 vertices. Total 384 vertices.
    let bytes_lod1 = generate_patch_vertices(&patch, TerrainLodLevel::Lod1);
    assert_eq!(bytes_lod1.len(), 384 * vertex_size);

    // Lod2: 16x16 grid, step 4 = 4 * 4 = 16 cells. Each cell has 2 triangles = 6 vertices. Total 96 vertices.
    let bytes_lod2 = generate_patch_vertices(&patch, TerrainLodLevel::Lod2);
    assert_eq!(bytes_lod2.len(), 96 * vertex_size);
}

#[test]
fn test_generate_patch_vertices_positions_and_normals() {
    let mut height_map = [0.0; 17 * 17];
    let mut normal_map = [[0.0, 0.0, 1.0]; 17 * 17];

    // Set some specific heights for validation
    height_map[0] = 10.0; // x=0, y=0
    height_map[1] = 20.0; // x=1, y=0
    height_map[17] = 30.0; // x=0, y=1
    height_map[18] = 40.0; // x=1, y=1

    // Set normal for v00
    normal_map[0] = [0.1, 0.2, 0.9];

    let patch = TerrainPatchRenderData {
        patch_x: 1, // base_x = 1 * 16 * 200 = 3200
        patch_y: 2, // base_y = 2 * 16 * 200 = 6400
        height_map,
        normal_map,
        splat_texture_ids: [0, 0, 0, 0],
    };

    let bytes = generate_patch_vertices(&patch, TerrainLodLevel::Lod0);
    
    // Read the first triangle (Triangle right: v00, v10, v11)
    let vertices: &[TerrainVertex] = unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const TerrainVertex,
            bytes.len() / std::mem::size_of::<TerrainVertex>(),
        )
    };

    // v00
    assert_eq!(vertices[0].position, [3200.0, 6400.0, 10.0]);
    assert_eq!(vertices[0].normal, [0.1, 0.2, 0.9]);

    // v10
    assert_eq!(vertices[1].position, [3400.0, 6400.0, 20.0]);
    
    // v11
    assert_eq!(vertices[2].position, [3400.0, 6600.0, 40.0]);

    // Read the second triangle (Triangle left: v00, v11, v01)
    
    // v00
    assert_eq!(vertices[3].position, [3200.0, 6400.0, 10.0]);
    
    // v11
    assert_eq!(vertices[4].position, [3400.0, 6600.0, 40.0]);

    // v01
    assert_eq!(vertices[5].position, [3200.0, 6600.0, 30.0]);
}

#[test]
fn test_is_patch_in_frustum_fully_inside() {
    let patch_min = [0.0, 0.0, 0.0];
    let patch_max = [100.0, 100.0, 100.0];

    // Simple frustum: six planes forming a box from -100 to 200
    let frustum_planes = [
        [1.0, 0.0, 0.0, 100.0],  // Left plane, x >= -100
        [-1.0, 0.0, 0.0, 200.0], // Right plane, x <= 200
        [0.0, 1.0, 0.0, 100.0],  // Bottom plane, y >= -100
        [0.0, -1.0, 0.0, 200.0], // Top plane, y <= 200
        [0.0, 0.0, 1.0, 100.0],  // Near plane, z >= -100
        [0.0, 0.0, -1.0, 200.0], // Far plane, z <= 200
    ];

    assert!(is_patch_in_frustum(patch_min, patch_max, &frustum_planes));
}

#[test]
fn test_is_patch_in_frustum_completely_outside() {
    let patch_min = [300.0, 0.0, 0.0];
    let patch_max = [400.0, 100.0, 100.0];

    // Simple frustum: six planes forming a box from -100 to 200
    let frustum_planes = [
        [1.0, 0.0, 0.0, 100.0],  // Left plane, x >= -100
        [-1.0, 0.0, 0.0, 200.0], // Right plane, x <= 200 -> Normal points -X, Dist = 200
        [0.0, 1.0, 0.0, 100.0],  // Bottom plane, y >= -100
        [0.0, -1.0, 0.0, 200.0], // Top plane, y <= 200
        [0.0, 0.0, 1.0, 100.0],  // Near plane, z >= -100
        [0.0, 0.0, -1.0, 200.0], // Far plane, z <= 200
    ];

    // x is between 300 and 400. Right plane: -x + 200. Distance for p_x (positive vertex is 300 since normal is -1, so 300 is lower, wait: plane is [-1, 0, 0, 200]. positive vertex for normal [-1, 0, 0] uses min_x = 300. -300 + 200 = -100. Less than 0, so OUTSIDE.)
    assert!(!is_patch_in_frustum(patch_min, patch_max, &frustum_planes));
}

#[test]
fn test_is_patch_in_frustum_partially_inside() {
    let patch_min = [150.0, 0.0, 0.0];
    let patch_max = [250.0, 100.0, 100.0];

    // Simple frustum: six planes forming a box from -100 to 200
    let frustum_planes = [
        [1.0, 0.0, 0.0, 100.0],  // Left plane, x >= -100
        [-1.0, 0.0, 0.0, 200.0], // Right plane, x <= 200
        [0.0, 1.0, 0.0, 100.0],  // Bottom plane, y >= -100
        [0.0, -1.0, 0.0, 200.0], // Top plane, y <= 200
        [0.0, 0.0, 1.0, 100.0],  // Near plane, z >= -100
        [0.0, 0.0, -1.0, 200.0], // Far plane, z <= 200
    ];

    // Plane 2: [-1, 0, 0, 200]. Normal is -X, so positive vertex for -X is min_X (150).
    // dist = -150 + 200 = +50. Greater than 0, so it's not strictly outside this plane.
    // So it should return true.
    assert!(is_patch_in_frustum(patch_min, patch_max, &frustum_planes));
}
