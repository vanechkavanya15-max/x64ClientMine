use glam::{Vec3, Vec4};
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    None,
    Part,
    All,
}

#[derive(Debug, Clone)]
pub struct TerrainQuadtreeNode {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub size: i32,
    pub patch_num: i32,

    pub aabb_min: Vec3,
    pub aabb_max: Vec3,

    pub nw_node: Option<Box<TerrainQuadtreeNode>>,
    pub ne_node: Option<Box<TerrainQuadtreeNode>>,
    pub sw_node: Option<Box<TerrainQuadtreeNode>>,
    pub se_node: Option<Box<TerrainQuadtreeNode>>,
}

impl TerrainQuadtreeNode {
    pub fn alloc(x0: i32, y0: i32, x1: i32, y1: i32, patch_count: i32) -> Option<Box<Self>> {
        let xsize = x1 - x0 + 1;
        let ysize = y1 - y0 + 1;
        if xsize <= 0 || ysize <= 0 {
            return None;
        }

        let size = if ysize > xsize { ysize } else { xsize };
        let patch_num = y0 * patch_count + x0;

        Some(Box::new(Self {
            x0,
            y0,
            x1,
            y1,
            size,
            patch_num,
            aabb_min: Vec3::splat(f32::INFINITY),
            aabb_max: Vec3::splat(f32::NEG_INFINITY),
            nw_node: None,
            ne_node: None,
            sw_node: None,
            se_node: None,
        }))
    }

    pub fn subdivide(&mut self, patch_count: i32) {
        let nw_size = self.size / 2;

        self.nw_node = Self::alloc(
            self.x0,
            self.y0,
            self.x0 + nw_size - 1,
            self.y0 + nw_size - 1,
            patch_count,
        );
        self.ne_node = Self::alloc(
            self.x0 + nw_size,
            self.y0,
            self.x1,
            self.y0 + nw_size - 1,
            patch_count,
        );
        self.sw_node = Self::alloc(
            self.x0,
            self.y0 + nw_size,
            self.x0 + nw_size - 1,
            self.y1,
            patch_count,
        );
        self.se_node = Self::alloc(
            self.x0 + nw_size,
            self.y0 + nw_size,
            self.x1,
            self.y1,
            patch_count,
        );

        if let Some(ref mut node) = self.nw_node {
            if node.size > 1 {
                node.subdivide(patch_count);
            }
        }
        if let Some(ref mut node) = self.ne_node {
            if node.size > 1 {
                node.subdivide(patch_count);
            }
        }
        if let Some(ref mut node) = self.sw_node {
            if node.size > 1 {
                node.subdivide(patch_count);
            }
        }
        if let Some(ref mut node) = self.se_node {
            if node.size > 1 {
                node.subdivide(patch_count);
            }
        }
    }

    pub fn check_frustum(&self, frustum_planes: &[Vec4; 6]) -> ViewState {
        let mut result = ViewState::All;

        for i in 0..6 {
            let plane = frustum_planes[i];

            let mut px = self.aabb_min.x;
            let mut py = self.aabb_min.y;
            let mut pz = self.aabb_min.z;
            let mut nx = self.aabb_max.x;
            let mut ny = self.aabb_max.y;
            let mut nz = self.aabb_max.z;

            if plane.x >= 0.0 {
                px = self.aabb_max.x;
                nx = self.aabb_min.x;
            }
            if plane.y >= 0.0 {
                py = self.aabb_max.y;
                ny = self.aabb_min.y;
            }
            if plane.z >= 0.0 {
                pz = self.aabb_max.z;
                nz = self.aabb_min.z;
            }

            let dp = plane.x * px + plane.y * py + plane.z * pz + plane.w;
            if dp < 0.0 {
                return ViewState::None;
            }

            let dn = plane.x * nx + plane.y * ny + plane.z * nz + plane.w;
            if dn < 0.0 {
                result = ViewState::Part;
            }
        }

        result
    }
}

pub struct MapOutdoorQuadtree {
    pub patch_count: i32,
    pub root_node: Option<Box<TerrainQuadtreeNode>>,
}

impl MapOutdoorQuadtree {
    pub fn new() -> Self {
        Self {
            patch_count: 0,
            root_node: None,
        }
    }

    pub fn build(&mut self, patch_count: i32) {
        self.root_node = None;
        if patch_count == 0 {
            return;
        }

        self.patch_count = patch_count;
        let mut root = TerrainQuadtreeNode::alloc(0, 0, patch_count - 1, patch_count - 1, patch_count);
        if let Some(ref mut r) = root {
            if r.size > 1 {
                r.subdivide(patch_count);
            }
        }
        self.root_node = root;
    }

    pub fn set_node_aabb(&mut self, patch_num: i32, aabb_min: Vec3, aabb_max: Vec3) -> bool {
        fn recurse_set_aabb(node: &mut TerrainQuadtreeNode, patch_num: i32, aabb_min: Vec3, aabb_max: Vec3) -> bool {
            if node.size == 1 {
                if node.patch_num == patch_num {
                    node.aabb_min = aabb_min;
                    node.aabb_max = aabb_max;
                    return true;
                }
                return false;
            }

            let mut found = false;
            if let Some(ref mut child) = node.nw_node { found |= recurse_set_aabb(child, patch_num, aabb_min, aabb_max); }
            if let Some(ref mut child) = node.ne_node { found |= recurse_set_aabb(child, patch_num, aabb_min, aabb_max); }
            if let Some(ref mut child) = node.sw_node { found |= recurse_set_aabb(child, patch_num, aabb_min, aabb_max); }
            if let Some(ref mut child) = node.se_node { found |= recurse_set_aabb(child, patch_num, aabb_min, aabb_max); }

            if found {
                // Update parent AABB
                node.aabb_min = node.aabb_min.min(aabb_min);
                node.aabb_max = node.aabb_max.max(aabb_max);
            }
            found
        }

        if let Some(ref mut root) = self.root_node {
            return recurse_set_aabb(root, patch_num, aabb_min, aabb_max);
        }
        false
    }
}

pub fn recurse_render_quadtree(
    node: &TerrainQuadtreeNode,
    frustum_planes: &[Vec4; 6],
    mut b_cull_check_need: bool,
    visible_patches: &mut usize,
) {
    if b_cull_check_need {
        match node.check_frustum(frustum_planes) {
            ViewState::All => {
                b_cull_check_need = false;
            }
            ViewState::Part => {
                // Keep checking children
            }
            ViewState::None => {
                return;
            }
        }
    }

    if node.size == 1 {
        *visible_patches += 1;
    } else {
        if let Some(ref child) = node.nw_node {
            recurse_render_quadtree(child, frustum_planes, b_cull_check_need, visible_patches);
        }
        if let Some(ref child) = node.ne_node {
            recurse_render_quadtree(child, frustum_planes, b_cull_check_need, visible_patches);
        }
        if let Some(ref child) = node.sw_node {
            recurse_render_quadtree(child, frustum_planes, b_cull_check_need, visible_patches);
        }
        if let Some(ref child) = node.se_node {
            recurse_render_quadtree(child, frustum_planes, b_cull_check_need, visible_patches);
        }
    }
}

// Global state for FFI
lazy_static::lazy_static! {
    static ref GLOBAL_QUADTREE: RwLock<MapOutdoorQuadtree> = RwLock::new(MapOutdoorQuadtree::new());
}

#[no_mangle]
pub extern "C" fn m2_quadtree_build(patch_count: i32) {
    if let Ok(mut tree) = GLOBAL_QUADTREE.write() {
        tree.build(patch_count);
    }
}

#[no_mangle]
pub extern "C" fn m2_quadtree_free() {
    if let Ok(mut tree) = GLOBAL_QUADTREE.write() {
        tree.root_node = None;
        tree.patch_count = 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_quadtree_set_aabb(patch_num: i32, min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> bool {
    if let Ok(mut tree) = GLOBAL_QUADTREE.write() {
        return tree.set_node_aabb(patch_num, Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z));
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_quadtree_cull(camera_frustum: *const f32) -> usize {
    if camera_frustum.is_null() {
        return 0;
    }

    let planes_slice = unsafe { std::slice::from_raw_parts(camera_frustum, 24) };
    let mut frustum_planes = [Vec4::ZERO; 6];
    for i in 0..6 {
        frustum_planes[i] = Vec4::new(
            planes_slice[i * 4],
            planes_slice[i * 4 + 1],
            planes_slice[i * 4 + 2],
            planes_slice[i * 4 + 3],
        );
    }

    let mut visible_patches = 0;
    if let Ok(tree) = GLOBAL_QUADTREE.read() {
        if let Some(ref root) = tree.root_node {
            recurse_render_quadtree(root, &frustum_planes, true, &mut visible_patches);
        }
    }

    visible_patches
}
