use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Inside,
    Partial,
    Outside,
}

#[derive(Debug, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone)]
pub struct FrustumCullingMapEngine {
    pub planes: [Vec4; 6],
}

impl Default for FrustumCullingMapEngine {
    fn default() -> Self {
        Self {
            planes: [Vec4::ZERO; 6],
        }
    }
}

impl FrustumCullingMapEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_view_frustum(&mut self, mat: Mat4) {
        let r0 = mat.row(0);
        let r1 = mat.row(1);
        let r2 = mat.row(2);
        let r3 = mat.row(3);

        // Near (D3D depth range is [0, 1]) -> mat._13, mat._23, mat._33, mat._43 -> row(2)
        self.planes[0] = r2;
        // Far
        self.planes[1] = r3 - r2;
        // Left
        self.planes[2] = r3 + r0;
        // Right
        self.planes[3] = r3 - r0;
        // Bottom
        self.planes[4] = r3 + r1;
        // Top
        self.planes[5] = r3 - r1;

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

    pub fn test_aabb(&self, aabb: &Aabb) -> ViewState {
        let mut result = ViewState::Inside;

        for i in 0..6 {
            let plane = self.planes[i];

            let mut px = aabb.min.x;
            let mut py = aabb.min.y;
            let mut pz = aabb.min.z;
            let mut nx = aabb.max.x;
            let mut ny = aabb.max.y;
            let mut nz = aabb.max.z;

            if plane.x > 0.0 {
                px = aabb.max.x;
                nx = aabb.min.x;
            }
            if plane.y > 0.0 {
                py = aabb.max.y;
                ny = aabb.min.y;
            }
            if plane.z > 0.0 {
                pz = aabb.max.z;
                nz = aabb.min.z;
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

    pub fn should_cull_aabb(&self, min: Vec3, max: Vec3) -> bool {
        let aabb = Aabb { min, max };
        self.test_aabb(&aabb) == ViewState::Outside
    }
}
