use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        let mut plane = Plane {
            normal: Vec3::new(a, b, c),
            distance: d,
        };
        plane.normalize();
        plane
    }

    pub fn normalize(&mut self) {
        let length = self.normal.length();
        if length > 0.0 {
            self.normal /= length;
            self.distance /= length;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_proj(mat: Mat4) -> Self {
        let r0 = mat.row(0);
        let r1 = mat.row(1);
        let r2 = mat.row(2);
        let r3 = mat.row(3);

        let mut frustum = Frustum {
            planes: [
                Plane::new(r3.x + r0.x, r3.y + r0.y, r3.z + r0.z, r3.w + r0.w), // Left
                Plane::new(r3.x - r0.x, r3.y - r0.y, r3.z - r0.z, r3.w - r0.w), // Right
                Plane::new(r3.x - r1.x, r3.y - r1.y, r3.z - r1.z, r3.w - r1.w), // Top
                Plane::new(r3.x + r1.x, r3.y + r1.y, r3.z + r1.z, r3.w + r1.w), // Bottom
                Plane::new(r2.x, r2.y, r2.z, r2.w),                             // Near (WGPU/D3D: Z in [0, 1])
                Plane::new(r3.x - r2.x, r3.y - r2.y, r3.z - r2.z, r3.w - r2.w), // Far
            ],
        };

        for plane in &mut frustum.planes {
            plane.normalize();
        }

        frustum
    }

    pub fn intersects_sphere(&self, sphere: &BoundingSphere) -> bool {
        for plane in &self.planes {
            if plane.normal.dot(sphere.center) + plane.distance < -sphere.radius {
                return false;
            }
        }
        true
    }

    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            let mut p = aabb.min;
            if plane.normal.x >= 0.0 {
                p.x = aabb.max.x;
            }
            if plane.normal.y >= 0.0 {
                p.y = aabb.max.y;
            }
            if plane.normal.z >= 0.0 {
                p.z = aabb.max.z;
            }

            if plane.normal.dot(p) + plane.distance < 0.0 {
                return false;
            }
        }
        true
    }
}
