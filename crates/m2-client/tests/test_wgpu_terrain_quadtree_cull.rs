use m2_client::wgpu_terrain_quadtree_cull::{
    Frustum, PatchProxyProvider, TerrainQuadtree, Vector3, ViewState, TerrainQuadtreeNode,
};

struct MockPatchProvider {
    pub used: bool,
    pub min: f32,
    pub max: f32,
}

impl PatchProxyProvider for MockPatchProvider {
    fn is_used(&self, _patch_num: i32) -> bool {
        self.used
    }
    fn get_min_x(&self, _patch_num: i32) -> f32 {
        self.min
    }
    fn get_max_x(&self, _patch_num: i32) -> f32 {
        self.max
    }
    fn get_min_y(&self, _patch_num: i32) -> f32 {
        self.min
    }
    fn get_max_y(&self, _patch_num: i32) -> f32 {
        self.max
    }
    fn get_min_z(&self, _patch_num: i32) -> f32 {
        self.min
    }
    fn get_max_z(&self, _patch_num: i32) -> f32 {
        self.max
    }
}

#[test]
fn test_terrain_quadtree_build() {
    let mut tree = TerrainQuadtree::new();
    tree.build(8); // 8x8 patches

    assert!(tree.root_node.is_some());
    let root = tree.root_node.as_ref().unwrap();

    // Check size
    assert_eq!(root.size, 8);
    assert_eq!(root.x0, 0);
    assert_eq!(root.y0, 0);
    assert_eq!(root.x1, 7);
    assert_eq!(root.y1, 7);

    // Root should have 4 children
    assert!(root.nw_node.is_some());
    assert!(root.ne_node.is_some());
    assert!(root.sw_node.is_some());
    assert!(root.se_node.is_some());
    
    // Check NW Node
    let nw = root.nw_node.as_ref().unwrap();
    assert_eq!(nw.size, 4);
    assert_eq!(nw.x0, 0);
    assert_eq!(nw.y0, 0);
    assert_eq!(nw.x1, 3);
    assert_eq!(nw.y1, 3);
}

#[test]
fn test_terrain_quadtree_heights_and_bounds() {
    let mut tree = TerrainQuadtree::new();
    tree.build(4);

    let provider = MockPatchProvider {
        used: true,
        min: -100.0,
        max: 100.0,
    };

    tree.update_quad_tree_heights(&provider);

    let root = tree.root_node.as_ref().unwrap();
    
    // Bounds should reflect the global mock values
    assert_eq!(root.min_x, -100.0);
    assert_eq!(root.max_x, 100.0);
    assert_eq!(root.min_y, -100.0);
    assert_eq!(root.max_y, 100.0);
    assert_eq!(root.min_z, -100.0);
    assert_eq!(root.max_z, 100.0);

    // Center should be (0,0,0)
    assert_eq!(root.center, Vector3::new(0.0, 0.0, 0.0));
    
    // Radius should be correct distance
    let diff = 200.0;
    let expected_radius = ((diff * diff + diff * diff + diff * diff) as f32).sqrt() / 2.0;
    assert!((root.radius - expected_radius).abs() < 0.001);
}

#[test]
fn test_frustum_plane_extraction() {
    let mut frustum = Frustum::new();
    
    // Identity matrix should map somewhat trivially
    let id_mat: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    frustum.build_view_frustum(&id_mat);
    
    // Plane 0 (Near): a=0, b=0, c=1, d=0
    assert_eq!(frustum.planes[0].a, 0.0);
    assert_eq!(frustum.planes[0].b, 0.0);
    assert_eq!(frustum.planes[0].c, 1.0);
    assert_eq!(frustum.planes[0].d, 0.0);
}

#[test]
fn test_view_volume_test_sphere() {
    let mut frustum = Frustum::new();
    // Simulate orthographic box frustum facing Z: x in [-1,1], y in [-1,1], z in [0,1]
    let id_mat: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    frustum.build_view_frustum(&id_mat);

    // Point in center: Inside
    let center = Vector3::new(0.0, 0.0, 0.5);
    assert_eq!(frustum.view_volume_test_sphere(&center, 0.1), ViewState::Inside);

    // Point exactly at edge with small radius: Partial
    let edge_center = Vector3::new(1.0, 0.0, 0.5);
    assert_eq!(frustum.view_volume_test_sphere(&edge_center, 0.1), ViewState::Partial);

    // Point far outside: Outside
    let out_center = Vector3::new(5.0, 0.0, 0.5);
    assert_eq!(frustum.view_volume_test_sphere(&out_center, 0.1), ViewState::Outside);
}

#[test]
fn test_quadtree_get_visible_patches() {
    let mut tree = TerrainQuadtree::new();
    tree.build(2); // 2x2 = 4 patches total

    struct TestProvider;
    impl PatchProxyProvider for TestProvider {
        fn is_used(&self, _patch_num: i32) -> bool { true }
        fn get_min_x(&self, p: i32) -> f32 { (p % 2) as f32 * 100.0 }
        fn get_max_x(&self, p: i32) -> f32 { (p % 2) as f32 * 100.0 + 100.0 }
        fn get_min_y(&self, p: i32) -> f32 { (p / 2) as f32 * 100.0 }
        fn get_max_y(&self, p: i32) -> f32 { (p / 2) as f32 * 100.0 + 100.0 }
        fn get_min_z(&self, _p: i32) -> f32 { 0.0 }
        fn get_max_z(&self, _p: i32) -> f32 { 10.0 }
    }

    let provider = TestProvider;
    tree.update_quad_tree_heights(&provider);

    // Fake a frustum that sees everything (all planes face outwards with huge positive distance)
    let mut frustum = Frustum::new();
    for plane in &mut frustum.planes {
        plane.a = 0.0;
        plane.b = 0.0;
        plane.c = 0.0;
        plane.d = 99999.0;
    }

    // Camera at (200, 200, 0)
    let cam_pos = Vector3::new(200.0, 200.0, 0.0);
    let visible = tree.get_visible_patches(&frustum, &cam_pos);
    
    // Should see all 4 patches
    assert_eq!(visible.len(), 4);

    // Sorted nearest to farthest. 
    // Patch 0 is (0-100, 0-100), Center (50, 50, 5) => dx=150, dy=150 => sq = 45000 + 25 = 45025
    // Patch 1 is (100-200, 0-100), Center (150, 50, 5) => dx=50, dy=150 => sq = 25000 + 25 = 25025
    // Patch 2 is (0-100, 100-200), Center (50, 150, 5) => dx=150, dy=50 => sq = 25000 + 25 = 25025
    // Patch 3 is (100-200, 100-200), Center (150, 150, 5) => dx=50, dy=50 => sq = 5000 + 25 = 5025
    
    // Farthest to nearest distance should be: 3, 1, 2, 0 or 3, 2, 1, 0 (1 and 2 are equal distance)
    assert_eq!(visible[0], 3); // nearest
    assert_eq!(visible[3], 0); // farthest
}
