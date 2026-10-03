use m2_client::terrain_height_patch::RawHeightMap;

use m2_client::wgpu_terrain_mesh_wire;

#[test]
fn test_calculate_normal_metin2_flat() {
    let heights = vec![0.0; 131 * 131];
    let map = RawHeightMap::new(131, 131, heights);
    let normal = wgpu_terrain_mesh_wire::calculate_normal_metin2(5, 5, 0, 0, &map, 1.0);
    assert_eq!(normal, [0.0, 0.0, 127.0]);
}

#[test]
fn test_calculate_normal_metin2_slope_x() {
    let mut heights = vec![0.0; 131 * 131];
    for y in 0..131 {
        for x in 0..131 {
             heights[y * 131 + x] = x as f32;
        }
    }
    let map = RawHeightMap::new(131, 131, heights);
    let normal = wgpu_terrain_mesh_wire::calculate_normal_metin2(5, 5, 0, 0, &map, 1.0);
    
    assert!((normal[0] - 0.634992).abs() < 0.01);
    assert_eq!(normal[1], 0.0);
    assert!((normal[2] - 126.9984).abs() < 0.01);
}

#[test]
fn test_calculate_normal_metin2_slope_y() {
    let mut heights = vec![0.0; 131 * 131];
    for y in 0..131 {
        for x in 0..131 {
             heights[y * 131 + x] = y as f32;
        }
    }
    let map = RawHeightMap::new(131, 131, heights);
    let normal = wgpu_terrain_mesh_wire::calculate_normal_metin2(5, 5, 0, 0, &map, 1.0);
    
    assert_eq!(normal[0], 0.0);
    assert!((normal[1] - 0.634992).abs() < 0.01);
    assert!((normal[2] - 126.9984).abs() < 0.01);
}

#[test]
fn test_calculate_normal_metin2_height_scale() {
    let mut heights = vec![0.0; 131 * 131];
    for y in 0..131 {
        for x in 0..131 {
             heights[y * 131 + x] = x as f32;
        }
    }
    let map = RawHeightMap::new(131, 131, heights);
    
    // Scale 0.0 means normal vector won't bend despite the slope
    let normal = wgpu_terrain_mesh_wire::calculate_normal_metin2(5, 5, 0, 0, &map, 0.0);
    assert_eq!(normal, [0.0, 0.0, 127.0]);
}


#[test]
fn test_build_patch_indices_wireframe_lod0() {
    let indices = wgpu_terrain_mesh_wire::build_patch_indices_wireframe(0);
    assert_eq!(indices.len(), 256 * 10);
}

#[test]
fn test_build_patch_indices_wireframe_lod1() {
    let indices = wgpu_terrain_mesh_wire::build_patch_indices_wireframe(1);
    assert_eq!(indices.len(), 64 * 10);
}

#[test]
fn test_build_patch_indices_wireframe_lod2() {
    let indices = wgpu_terrain_mesh_wire::build_patch_indices_wireframe(2);
    assert_eq!(indices.len(), 16 * 10);
}

#[test]
fn test_build_patch_indices_wireframe_correctness() {
    let indices = wgpu_terrain_mesh_wire::build_patch_indices_wireframe(4);
    // TERRAIN_PATCHSIZE = 16. Lod=4 -> Step=4.
    // Iter 0: x=0, y=0. current = 0. next_x = 4. next_y = 68. next_xy = 72.
    // Top: 0, 4. Left: 0, 68. Diag: 0, 72. Right: 4, 72. Bottom: 68, 72.
    
    // It should push exactly these 10 indices first.
    assert_eq!(indices[0..10], [0, 4, 0, 68, 0, 72, 4, 72, 68, 72]);
}
