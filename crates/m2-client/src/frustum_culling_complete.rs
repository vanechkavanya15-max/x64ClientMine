use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Inside,
    Partial,
    Outside,
}

#[derive(Debug, Clone)]
pub struct FrustumCulling {
    pub planes: [Vec4; 6],
    pub is_using_sphere: bool,
    pub center: Vec3,
    pub radius: f32,
}

impl Default for FrustumCulling {
    fn default() -> Self {
        Self {
            planes: [Vec4::ZERO; 6],
            is_using_sphere: false,
            center: Vec3::ZERO,
            radius: 0.0,
        }
    }
}

impl FrustumCulling {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_view_frustum(&mut self, mat: Mat4) {
        self.is_using_sphere = false;
        
        let r0 = mat.row(0);
        let r1 = mat.row(1);
        let r2 = mat.row(2);
        let r3 = mat.row(3);

        // Matching D3DXPLANE logic from frustum.cpp L50
        // m_plane[0] = D3DXPLANE(mat._13, mat._23, mat._33, mat._43); (DirectX depth near is 0)
        self.planes[0] = r2; // Near
        
        // m_plane[1] = D3DXPLANE(mat._14 - mat._13, mat._24 - mat._23, mat._34 - mat._33, mat._44 - mat._43);
        self.planes[1] = r3 - r2; // Far
        
        // m_plane[2] = D3DXPLANE(mat._14 + mat._11, mat._24 + mat._21, mat._34 + mat._31, mat._44 + mat._41);
        self.planes[2] = r3 + r0; // Left
        
        // m_plane[3] = D3DXPLANE(mat._14 - mat._11, mat._24 - mat._21, mat._34 - mat._31, mat._44 - mat._41);
        self.planes[3] = r3 - r0; // Right
        
        // m_plane[4] = D3DXPLANE(mat._14 + mat._12, mat._24 + mat._22, mat._34 + mat._32, mat._44 + mat._42);
        self.planes[4] = r3 + r1; // Bottom
        
        // m_plane[5] = D3DXPLANE(mat._14 - mat._12, mat._24 - mat._22, mat._34 - mat._32, mat._44 - mat._42);
        self.planes[5] = r3 - r1; // Top

        for i in 0..6 {
            let length = (self.planes[i].x * self.planes[i].x
                + self.planes[i].y * self.planes[i].y
                + self.planes[i].z * self.planes[i].z)
                .sqrt();
                
            if length > 0.0 {
                self.planes[i] /= length;
            }
        }
    }

    pub fn build_view_frustum_2(&mut self, mat: Mat4, near: f32, far: f32, fov: f32, aspect: f32, camera: Vec3, look: Vec3) {
        let view_len = far - near;
        let h = view_len * (fov * 0.5).tan();
        let w = h * aspect;
        
        let p = Vec3::new(0.0, 0.0, near + view_len * 0.5);
        let q = Vec3::new(w, h, view_len);
        let pq = p - q;
        
        self.radius = pq.length();
        self.center = camera + look * (near + view_len * 0.5);
        
        self.build_view_frustum(mat);
        self.is_using_sphere = true;
    }

    pub fn test_sphere(&self, center: Vec3, radius: f32) -> ViewState {
        let mut result = ViewState::Inside;
        for i in 0..6 {
            let plane = self.planes[i];
            let d = plane.x * center.x + plane.y * center.y + plane.z * center.z + plane.w;
            
            if d < -radius {
                return ViewState::Outside;
            }
            if d < radius {
                result = ViewState::Partial;
            }
        }
        result
    }

    pub fn test_aabb(&self, min: Vec3, max: Vec3) -> ViewState {
        let mut result = ViewState::Inside;

        for i in 0..6 {
            let plane = self.planes[i];

            let mut px = min.x;
            let mut py = min.y;
            let mut pz = min.z;
            let mut nx = max.x;
            let mut ny = max.y;
            let mut nz = max.z;

            if plane.x > 0.0 {
                px = max.x;
                nx = min.x;
            }
            if plane.y > 0.0 {
                py = max.y;
                ny = min.y;
            }
            if plane.z > 0.0 {
                pz = max.z;
                nz = min.z;
            }

            let dp = plane.x * px + plane.y * py + plane.z * pz + plane.w;
            if dp < 0.0 {
                return ViewState::Outside;
            }

            let dn = plane.x * nx + plane.y * ny + plane.z * nz + plane.w;
            if dn < 0.0 {
                result = ViewState::Partial;
            }
        }

        result
    }

    pub fn should_skip_draw_call_sphere(&self, center: Vec3, radius: f32) -> bool {
        self.test_sphere(center, radius) == ViewState::Outside
    }

    pub fn should_skip_draw_call_aabb(&self, min: Vec3, max: Vec3) -> bool {
        self.test_aabb(min, max) == ViewState::Outside
    }
}

// 1:1 match with CTerrainQuadtreeNode in TerrainQuadtreeNode.h
#[repr(C)]
pub struct TerrainQuadtreeNode {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub nw_node: *mut TerrainQuadtreeNode,
    pub ne_node: *mut TerrainQuadtreeNode,
    pub sw_node: *mut TerrainQuadtreeNode,
    pub se_node: *mut TerrainQuadtreeNode,
    pub size: i32,
    pub patch_num: i32,
    pub center: Vec3, // Using glam Vec3 for D3DXVECTOR3
    pub radius: f32,
    pub by_lod_level: u8,
}

impl Default for TerrainQuadtreeNode {
    fn default() -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
            nw_node: std::ptr::null_mut(),
            ne_node: std::ptr::null_mut(),
            sw_node: std::ptr::null_mut(),
            se_node: std::ptr::null_mut(),
            size: 0,
            patch_num: 0,
            center: Vec3::new(-1.0, -1.0, -1.0),
            radius: 0.0,
            by_lod_level: 0,
        }
    }
}
