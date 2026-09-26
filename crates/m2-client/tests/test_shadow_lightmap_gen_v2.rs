#[path = "../src/shadow_lightmap_gen_v2.rs"]
mod shadow_lightmap_gen_v2;

use glam::Vec3;
use shadow_lightmap_gen_v2::*;

#[test]
fn test_terrain_quadtree_node_creation() {
    let node = TerrainQuadtreeNode::new(0, 0, 7, 7, 8).unwrap();
    assert_eq!(node.x0, 0);
    assert_eq!(node.y0, 0);
    assert_eq!(node.x1, 7);
    assert_eq!(node.y1, 7);
    assert_eq!(node.size, 8);
    assert_eq!(node.patch_num, 0);
}

#[test]
fn test_terrain_quadtree_invalid_creation() {
    let node = TerrainQuadtreeNode::new(7, 7, 6, 7, 8); // xsize = 0
    assert!(node.is_none());
}

#[test]
fn test_map_outdoor_quadtree_build() {
    let mut tree = MapOutdoorQuadtree::new();
    tree.patch_count = 8;
    tree.build_quad_tree();

    let root = tree.root_node.as_ref().unwrap();
    assert_eq!(root.size, 8);
    assert_eq!(root.x0, 0);
    assert_eq!(root.y0, 0);
    assert_eq!(root.x1, 7);
    assert_eq!(root.y1, 7);

    // Verify subdivision
    let nw = root.nw_node.as_ref().unwrap();
    assert_eq!(nw.size, 4);
    assert_eq!(nw.x0, 0);
    assert_eq!(nw.y0, 0);
    assert_eq!(nw.x1, 3);
    assert_eq!(nw.y1, 3);
    
    let se = root.se_node.as_ref().unwrap();
    assert_eq!(se.size, 4);
    assert_eq!(se.x0, 4);
    assert_eq!(se.y0, 4);
    assert_eq!(se.x1, 7);
    assert_eq!(se.y1, 7);
}

#[test]
fn test_map_outdoor_quadtree_free() {
    let mut tree = MapOutdoorQuadtree::new();
    tree.patch_count = 2;
    tree.build_quad_tree();
    assert!(tree.root_node.is_some());

    tree.free_quad_tree();
    assert!(tree.root_node.is_none());
}

#[test]
fn test_shadow_lightmap_gen_no_shadow() {
    let sun_dir = Vec3::new(1.0, 1.0, 1.0);
    let gen = ShadowLightmapGen::new(sun_dir);

    let get_height = |_, _| 0.0; // Flat terrain at z=0

    let factor = gen.compute_shadow_factor(0.0, 0.0, 100.0, &get_height);
    assert_eq!(factor, 1.0); // Fully lit
}

#[test]
fn test_shadow_lightmap_gen_in_shadow() {
    let sun_dir = Vec3::new(1.0, 0.0, 1.0); // Sun is at 45 degrees pitch
    let gen = ShadowLightmapGen::new(sun_dir);

    // Terrain is a giant wall at x >= 500, height = 2000
    let get_height = |x: f32, _y: f32| -> f32 {
        if x > 200.0 {
            2000.0
        } else {
            0.0
        }
    };

    // Cast from x=0, z=0. The ray goes towards +x, +z. 
    // At x=500 (distance 500), ray z is 500. Terrain z is 2000.
    // 2000 > 500, so it hits the terrain and is in shadow.
    let factor = gen.compute_shadow_factor(0.0, 0.0, 0.0, &get_height);
    assert_eq!(factor, 0.0); // In shadow
}

#[test]
fn test_shadow_lightmap_gen_invalid_sun_direction() {
    let sun_dir = Vec3::new(1.0, 1.0, -1.0); // Sun is below horizon
    let gen = ShadowLightmapGen::new(sun_dir);

    let get_height = |_, _| 1000.0; // Terrain is high, but sun is below horizon

    let factor = gen.compute_shadow_factor(0.0, 0.0, 0.0, &get_height);
    assert_eq!(factor, 1.0); // Returns lit when invalid
}

#[test]
fn test_ffi_m2_terrain_is_in_shadow() {
    clear_terrain_height_for_ffi();
    set_sun_direction_for_ffi(Vec3::new(1.0, 0.0, 1.0));
    
    // Set a block of terrain high up
    // Ray from (0,0,0) with dir (1,0,1) passes through x=200, z=200.
    set_terrain_height_for_ffi(1, 0, 1000.0); // At x=200, terrain is 1000
    
    // We expect it to hit the terrain and return true (is in shadow)
    let in_shadow = m2_terrain_is_in_shadow(0.0, 0.0, 0.0);
    assert!(in_shadow);
    
    clear_terrain_height_for_ffi();
    
    // No terrain, should not be in shadow
    let in_shadow_lit = m2_terrain_is_in_shadow(0.0, 0.0, 0.0);
    assert!(!in_shadow_lit);
}
