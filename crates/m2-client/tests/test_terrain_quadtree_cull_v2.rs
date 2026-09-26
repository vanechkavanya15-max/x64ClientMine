#[cfg(test)]
mod tests {
    use m2_client::terrain_quadtree_cull_v2::*;
    use glam::{Vec3, Vec4};
use serial_test::serial;

    #[test]
    fn test_node_allocation() {
        let node = TerrainQuadtreeNode::alloc(0, 0, 15, 15, 16).unwrap();
        assert_eq!(node.x0, 0);
        assert_eq!(node.y0, 0);
        assert_eq!(node.x1, 15);
        assert_eq!(node.y1, 15);
        assert_eq!(node.size, 16);
        assert_eq!(node.patch_num, 0);
    }

    #[test]
    fn test_invalid_allocation() {
        let node = TerrainQuadtreeNode::alloc(15, 15, 0, 0, 16);
        assert!(node.is_none());
    }

    #[test]
    fn test_subdivision() {
        let mut node = TerrainQuadtreeNode::alloc(0, 0, 15, 15, 16).unwrap();
        node.subdivide(16);

        // NW Node
        let nw = node.nw_node.as_ref().unwrap();
        assert_eq!(nw.size, 8);
        assert_eq!(nw.x0, 0);
        assert_eq!(nw.y0, 0);
        assert_eq!(nw.x1, 7);
        assert_eq!(nw.y1, 7);

        // SE Node
        let se = node.se_node.as_ref().unwrap();
        assert_eq!(se.size, 8);
        assert_eq!(se.x0, 8);
        assert_eq!(se.y0, 8);
        assert_eq!(se.x1, 15);
        assert_eq!(se.y1, 15);
    }

    #[test]
    fn test_check_frustum_all_inside() {
        let mut node = TerrainQuadtreeNode::alloc(0, 0, 15, 15, 16).unwrap();
        node.aabb_min = Vec3::new(-10.0, -10.0, -10.0);
        node.aabb_max = Vec3::new(10.0, 10.0, 10.0);

        let mut frustum_planes = [Vec4::ZERO; 6];
        frustum_planes[0] = Vec4::new(1.0, 0.0, 0.0, 100.0); // right facing
        frustum_planes[1] = Vec4::new(-1.0, 0.0, 0.0, 100.0); // left facing
        frustum_planes[2] = Vec4::new(0.0, 1.0, 0.0, 100.0); // top facing
        frustum_planes[3] = Vec4::new(0.0, -1.0, 0.0, 100.0); // bottom facing
        frustum_planes[4] = Vec4::new(0.0, 0.0, 1.0, 100.0); // back facing
        frustum_planes[5] = Vec4::new(0.0, 0.0, -1.0, 100.0); // front facing

        assert_eq!(node.check_frustum(&frustum_planes), ViewState::All);
    }

    #[test]
    fn test_check_frustum_outside() {
        let mut node = TerrainQuadtreeNode::alloc(0, 0, 15, 15, 16).unwrap();
        node.aabb_min = Vec3::new(200.0, 200.0, 200.0);
        node.aabb_max = Vec3::new(210.0, 210.0, 210.0);

        let mut frustum_planes = [Vec4::ZERO; 6];
        frustum_planes[0] = Vec4::new(1.0, 0.0, 0.0, 100.0); 
        frustum_planes[1] = Vec4::new(-1.0, 0.0, 0.0, 100.0); 
        frustum_planes[2] = Vec4::new(0.0, 1.0, 0.0, 100.0); 
        frustum_planes[3] = Vec4::new(0.0, -1.0, 0.0, 100.0); 
        frustum_planes[4] = Vec4::new(0.0, 0.0, 1.0, 100.0); 
        frustum_planes[5] = Vec4::new(0.0, 0.0, -1.0, 100.0); 

        // Plane 1: -x + 100 = 0 -> -200 + 100 = -100 < 0
        assert_eq!(node.check_frustum(&frustum_planes), ViewState::None);
    }

    #[test]
    fn test_check_frustum_partial() {
        let mut node = TerrainQuadtreeNode::alloc(0, 0, 15, 15, 16).unwrap();
        node.aabb_min = Vec3::new(95.0, 95.0, 95.0);
        node.aabb_max = Vec3::new(105.0, 105.0, 105.0);

        let mut frustum_planes = [Vec4::ZERO; 6];
        frustum_planes[0] = Vec4::new(1.0, 0.0, 0.0, 100.0); 
        frustum_planes[1] = Vec4::new(-1.0, 0.0, 0.0, 100.0); 
        frustum_planes[2] = Vec4::new(0.0, 1.0, 0.0, 100.0); 
        frustum_planes[3] = Vec4::new(0.0, -1.0, 0.0, 100.0); 
        frustum_planes[4] = Vec4::new(0.0, 0.0, 1.0, 100.0); 
        frustum_planes[5] = Vec4::new(0.0, 0.0, -1.0, 100.0); 

        assert_eq!(node.check_frustum(&frustum_planes), ViewState::Part);
    }

    #[test]
    #[serial]
    fn test_ffi_build_and_cull_all() {
        m2_quadtree_build(16);
        
        let frustum_planes = [
            1.0, 0.0, 0.0, 1000.0,
            -1.0, 0.0, 0.0, 1000.0,
            0.0, 1.0, 0.0, 1000.0,
            0.0, -1.0, 0.0, 1000.0,
            0.0, 0.0, 1.0, 1000.0,
            0.0, 0.0, -1.0, 1000.0,
        ];

        let visible = m2_quadtree_cull(frustum_planes.as_ptr());
        assert_eq!(visible, 256); // 16 * 16 patches
    }

    #[test]
    #[serial]
    fn test_ffi_cull_none() {
        m2_quadtree_build(16);

        // A trick to make AABBs far away. 
        // We set AABB for all leaf patches to be far
        for y in 0..16 {
            for x in 0..16 {
                let patch_num = y * 16 + x;
                m2_quadtree_set_aabb(patch_num, 2000.0, 2000.0, 2000.0, 2100.0, 2100.0, 2100.0);
            }
        }

        let frustum_planes = [
            1.0, 0.0, 0.0, 100.0,
            -1.0, 0.0, 0.0, 100.0,
            0.0, 1.0, 0.0, 100.0,
            0.0, -1.0, 0.0, 100.0,
            0.0, 0.0, 1.0, 100.0,
            0.0, 0.0, -1.0, 100.0,
        ];

        let visible = m2_quadtree_cull(frustum_planes.as_ptr());
        assert_eq!(visible, 0, "visible: {}", visible); 
    }

    #[test]
    #[serial]
    fn test_ffi_free() {
        m2_quadtree_build(16);
        m2_quadtree_free();
        
        let frustum_planes = [
            1.0, 0.0, 0.0, 100.0,
            -1.0, 0.0, 0.0, 100.0,
            0.0, 1.0, 0.0, 100.0,
            0.0, -1.0, 0.0, 100.0,
            0.0, 0.0, 1.0, 100.0,
            0.0, 0.0, -1.0, 100.0,
        ];

        let visible = m2_quadtree_cull(frustum_planes.as_ptr());
        assert_eq!(visible, 0, "visible: {}", visible); 
    }
}
