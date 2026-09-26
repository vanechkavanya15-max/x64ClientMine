pub const SECTOR_SIZE_CM: f32 = 128.0 * 200.0; // Przykładowo, dopasujemy później, jeżeli nie będzie podane jawnie. Z zadania: "min_x = sector_base_x", "max_x = sector_base_x + SECTOR_SIZE_CM"

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB3D {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

impl AABB3D {
    pub fn new(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
        Self { min, max }
    }

    pub fn from_sector(sector_base_x: f32, sector_base_y: f32, min_height: f32, max_height: f32, sector_size_cm: f32) -> Self {
        Self {
            min: (sector_base_x, sector_base_y, min_height),
            max: (sector_base_x + sector_size_cm, sector_base_y + sector_size_cm, max_height),
        }
    }

    pub fn intersects_aabb(&self, other: &AABB3D) -> bool {
        self.min.0 <= other.max.0 && self.max.0 >= other.min.0 &&
        self.min.1 <= other.max.1 && self.max.1 >= other.min.1 &&
        self.min.2 <= other.max.2 && self.max.2 >= other.min.2
    }

    pub fn contains_point(&self, point: (f32, f32, f32)) -> bool {
        point.0 >= self.min.0 && point.0 <= self.max.0 &&
        point.1 >= self.min.1 && point.1 <= self.max.1 &&
        point.2 >= self.min.2 && point.2 <= self.max.2
    }

    pub fn is_in_frustum_planes(&self, planes: &[(f32, f32, f32, f32); 6]) -> bool {
        for plane in planes.iter() {
            // A plane is (a, b, c, d) representing a*x + b*y + c*z + d = 0.
            // The normal is (a, b, c), and the plane equation gives distance (signed).
            // The positive half-space is a*x + b*y + c*z + d > 0.
            // We want to cull if the AABB is completely on the negative side of the plane.
            
            // To do this efficiently, find the vertex of the AABB furthest along the plane normal.
            // If that vertex has distance < 0, then the whole AABB is outside.
            // Wait, we need to find the vertex of the AABB that is in the direction of the normal.
            
            // If normal points outside, then distance > 0 means outside. Wait, standard view frustum planes usually
            // have normals pointing INSIDE the frustum. So ax+by+cz+d > 0 is inside.
            // If ax+by+cz+d < 0 for all points in AABB, then AABB is completely outside.
            // So we find the maximum value of ax+by+cz+d over the AABB.
            // max_val = a * (a > 0 ? max.x : min.x) + b * (b > 0 ? max.y : min.y) + c * (c > 0 ? max.z : min.z) + d.
            // If max_val < 0, then all points have < 0 distance, so it's completely outside -> return false.

            let (a, b, c, d) = *plane;
            
            let mut p_x = self.min.0;
            let mut p_y = self.min.1;
            let mut p_z = self.min.2;
            
            if a >= 0.0 { p_x = self.max.0; }
            if b >= 0.0 { p_y = self.max.1; }
            if c >= 0.0 { p_z = self.max.2; }
            
            if a * p_x + b * p_y + c * p_z + d < 0.0 {
                return false; // completely outside
            }
        }
        
        true
    }
}
