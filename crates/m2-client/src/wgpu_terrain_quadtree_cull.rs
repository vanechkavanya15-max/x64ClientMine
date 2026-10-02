use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Inside,
    Partial,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }
    
    pub fn normalize(&mut self) {
        let length = (self.a * self.a + self.b * self.b + self.c * self.c).sqrt();
        if length > 0.0001 {
            let inv_length = 1.0 / length;
            self.a *= inv_length;
            self.b *= inv_length;
            self.c *= inv_length;
            self.d *= inv_length;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

#[derive(Debug, Clone)]
pub struct TerrainQuadtreeNode {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub size: i32,
    pub patch_num: i32,

    pub center: Vector3,
    pub radius: f32,
    
    // Explicit AABB bounds as requested by prompt
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,

    pub nw_node: Option<Box<TerrainQuadtreeNode>>,
    pub ne_node: Option<Box<TerrainQuadtreeNode>>,
    pub sw_node: Option<Box<TerrainQuadtreeNode>>,
    pub se_node: Option<Box<TerrainQuadtreeNode>>,
}

pub struct TerrainQuadtree {
    pub root_node: Option<Box<TerrainQuadtreeNode>>,
    pub patch_count: i32,
}

pub trait PatchProxyProvider {
    fn is_used(&self, patch_num: i32) -> bool;
    fn get_min_x(&self, patch_num: i32) -> f32;
    fn get_max_x(&self, patch_num: i32) -> f32;
    fn get_min_y(&self, patch_num: i32) -> f32;
    fn get_max_y(&self, patch_num: i32) -> f32;
    fn get_min_z(&self, patch_num: i32) -> f32;
    fn get_max_z(&self, patch_num: i32) -> f32;
}

impl Frustum {
    pub fn new() -> Self {
        Self {
            planes: [Plane::new(0.0, 0.0, 0.0, 0.0); 6],
        }
    }

    /// Builds the view frustum from a column-major view-projection matrix.
    /// Uses Gribb-Hartmann plane extraction method mirroring the C++ implementation.
    pub fn build_view_frustum(&mut self, mat: &[f32; 16]) {
        // mat is assumed to be column-major order just like D3DMATRIX / wgpu matrices
        // mat[0] = _11, mat[4] = _21, mat[8] = _31, mat[12] = _41
        // mat[1] = _12, mat[5] = _22, mat[9] = _32, mat[13] = _42
        // mat[2] = _13, mat[6] = _23, mat[10] = _33, mat[14] = _43
        // mat[3] = _14, mat[7] = _24, mat[11] = _34, mat[15] = _44

        // Plane 0: Near
        self.planes[0] = Plane::new(mat[2], mat[6], mat[10], mat[14]);
        
        // Plane 1: Far
        self.planes[1] = Plane::new(mat[3] - mat[2], mat[7] - mat[6], mat[11] - mat[10], mat[15] - mat[14]);
        
        // Plane 2: Left
        self.planes[2] = Plane::new(mat[3] + mat[0], mat[7] + mat[4], mat[11] + mat[8], mat[15] + mat[12]);
        
        // Plane 3: Right
        self.planes[3] = Plane::new(mat[3] - mat[0], mat[7] - mat[4], mat[11] - mat[8], mat[15] - mat[12]);
        
        // Plane 4: Bottom
        self.planes[4] = Plane::new(mat[3] + mat[1], mat[7] + mat[5], mat[11] + mat[9], mat[15] + mat[13]);
        
        // Plane 5: Top
        self.planes[5] = Plane::new(mat[3] - mat[1], mat[7] - mat[5], mat[11] - mat[9], mat[15] - mat[13]);

        for i in 0..6 {
            self.planes[i].normalize();
        }
    }

    pub fn view_volume_test_sphere(&self, center: &Vector3, radius: f32) -> ViewState {
        let mut result = ViewState::Inside;
        
        for plane in &self.planes {
            let distance = plane.a * center.x + plane.b * center.y + plane.c * center.z + plane.d;
            if distance < -radius {
                return ViewState::Outside;
            }
            if distance < radius {
                result = ViewState::Partial;
            }
        }
        
        result
    }

    pub fn view_volume_test_aabb(&self, min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> ViewState {
        let mut result = ViewState::Inside;

        for plane in &self.planes {
            let mut px = min_x;
            let mut py = min_y;
            let mut pz = min_z;
            let mut nx = max_x;
            let mut ny = max_y;
            let mut nz = max_z;

            if plane.a >= 0.0 {
                px = max_x;
                nx = min_x;
            }
            if plane.b >= 0.0 {
                py = max_y;
                ny = min_y;
            }
            if plane.c >= 0.0 {
                pz = max_z;
                nz = min_z;
            }

            // Test Positive Vertex
            let dp = plane.a * px + plane.b * py + plane.c * pz + plane.d;
            if dp < 0.0 {
                return ViewState::Outside;
            }

            // Test Negative Vertex
            let dn = plane.a * nx + plane.b * ny + plane.c * nz + plane.d;
            if dn < 0.0 {
                result = ViewState::Partial;
            }
        }

        result
    }
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
            center: Vector3::new(-1.0, -1.0, -1.0),
            radius: 0.0,
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,
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

    pub fn update_quad_tree_heights<P: PatchProxyProvider>(&mut self, provider: &P, patch_count: i32) {
        let mut minx = 0.0f32;
        let mut maxx = 0.0f32;
        let mut miny = 0.0f32;
        let mut maxy = 0.0f32;
        let mut minz = 0.0f32;
        let mut maxz = 0.0f32;

        if provider.is_used(self.patch_num) {
            minx = provider.get_min_x(self.patch_num);
            maxx = provider.get_max_x(self.patch_num);
            miny = provider.get_min_y(self.patch_num);
            maxy = provider.get_max_y(self.patch_num);
            minz = provider.get_min_z(self.patch_num);
            maxz = provider.get_max_z(self.patch_num);
        }

        for y in self.y0..=self.y1 {
            for x in self.x0..=self.x1 {
                let patch = y * patch_count + x;

                if !provider.is_used(patch) {
                    continue;
                }

                let px_min = provider.get_min_x(patch);
                let px_max = provider.get_max_x(patch);
                let py_min = provider.get_min_y(patch);
                let py_max = provider.get_max_y(patch);
                let pz_min = provider.get_min_z(patch);
                let pz_max = provider.get_max_z(patch);

                if px_min < minx { minx = px_min; }
                if px_max > maxx { maxx = px_max; }
                if py_min < miny { miny = py_min; }
                if py_max > maxy { maxy = py_max; }
                if pz_min < minz { minz = pz_min; }
                if pz_max > maxz { maxz = pz_max; }
            }
        }

        self.min_x = minx;
        self.max_x = maxx;
        self.min_y = miny;
        self.max_y = maxy;
        self.min_z = minz;
        self.max_z = maxz;

        self.center.x = (maxx + minx) * 0.5;
        self.center.y = (maxy + miny) * 0.5;
        self.center.z = (maxz + minz) * 0.5;

        self.radius = (((maxx - minx) * (maxx - minx) +
                        (maxy - miny) * (maxy - miny) +
                        (maxz - minz) * (maxz - minz)).sqrt()) / 2.0;

        if let Some(ref mut child) = self.nw_node { child.update_quad_tree_heights(provider, patch_count); }
        if let Some(ref mut child) = self.ne_node { child.update_quad_tree_heights(provider, patch_count); }
        if let Some(ref mut child) = self.sw_node { child.update_quad_tree_heights(provider, patch_count); }
        if let Some(ref mut child) = self.se_node { child.update_quad_tree_heights(provider, patch_count); }
    }
}

impl TerrainQuadtree {
    pub fn new() -> Self {
        Self {
            root_node: None,
            patch_count: 0,
        }
    }

    pub fn build(&mut self, patch_count: i32) {
        self.root_node = None;
        
        if patch_count <= 0 {
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

    pub fn free(&mut self) {
        self.root_node = None;
        self.patch_count = 0;
    }

    pub fn update_quad_tree_heights<P: PatchProxyProvider>(&mut self, provider: &P) {
        if let Some(ref mut root) = self.root_node {
            root.update_quad_tree_heights(provider, self.patch_count);
        }
    }

    fn recurse_render_quadtree(
        node: &TerrainQuadtreeNode,
        frustum: &Frustum,
        camera_pos: &Vector3,
        mut b_cull_check_need: bool,
        visible_patches: &mut Vec<(f32, i32)>, // (distance squared, patch_num)
    ) {
        if b_cull_check_need {
            let state = frustum.view_volume_test_aabb(
                node.min_x, node.min_y, node.min_z,
                node.max_x, node.max_y, node.max_z
            );
            
            match state {
                ViewState::Outside => { return; },
                ViewState::Inside => { b_cull_check_need = false; },
                ViewState::Partial => {},
            }
        }

        if node.size == 1 {
            let dx = camera_pos.x - node.center.x;
            let dy = camera_pos.y - node.center.y;
            let dz = camera_pos.z - node.center.z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            visible_patches.push((dist_sq, node.patch_num));
        } else {
            if let Some(ref child) = node.nw_node { Self::recurse_render_quadtree(child, frustum, camera_pos, b_cull_check_need, visible_patches); }
            if let Some(ref child) = node.ne_node { Self::recurse_render_quadtree(child, frustum, camera_pos, b_cull_check_need, visible_patches); }
            if let Some(ref child) = node.sw_node { Self::recurse_render_quadtree(child, frustum, camera_pos, b_cull_check_need, visible_patches); }
            if let Some(ref child) = node.se_node { Self::recurse_render_quadtree(child, frustum, camera_pos, b_cull_check_need, visible_patches); }
        }
    }

    pub fn get_visible_patches(&self, frustum: &Frustum, camera_pos: &Vector3) -> Vec<i32> {
        let mut visible_patches_with_dist = Vec::new();
        
        if let Some(ref root) = self.root_node {
            Self::recurse_render_quadtree(root, frustum, camera_pos, true, &mut visible_patches_with_dist);
        }

        // Sort by distance (nearest first)
        visible_patches_with_dist.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal)
        });

        visible_patches_with_dist.into_iter().map(|(_, patch_num)| patch_num).collect()
    }
}
