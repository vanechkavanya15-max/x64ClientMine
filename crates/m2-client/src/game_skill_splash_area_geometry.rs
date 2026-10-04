use glam::Vec3;

pub trait SplashGeometry {
    fn is_point_inside(&self, point: Vec3) -> bool;
    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool;
}

#[derive(Debug, Clone)]
pub struct ConeSplash {
    pub origin: Vec3,
    pub direction: Vec3, // Should be normalized
    pub radius: f32,
    pub cone_angle_cos: f32, // Cosine of half the cone angle
}

impl SplashGeometry for ConeSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let diff = point - self.origin;
        let distance_sq = diff.length_squared();

        if distance_sq > self.radius * self.radius {
            return false;
        }

        if distance_sq == 0.0 {
            return true;
        }

        let distance = distance_sq.sqrt();
        let dir_to_target = diff / distance;
        let dot_product = self.direction.dot(dir_to_target);

        dot_product >= self.cone_angle_cos
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        // Approximate by treating the sphere as a point pushed slightly towards the origin
        let diff = center - self.origin;
        let distance_sq = diff.length_squared();
        
        let effective_radius = self.radius + radius;
        if distance_sq > effective_radius * effective_radius {
            return false;
        }

        let distance = distance_sq.sqrt();

        if distance <= radius {
            return true;
        }

        // For angle check, we check the center. A perfect check would involve finding the cone-sphere intersection.
        let dir_to_target = diff / distance;
        let dot_product = self.direction.dot(dir_to_target);

        // Relax the cone angle slightly to account for the sphere's radius.
        // theta_sphere = asin(radius / distance)
        let relax_angle = (radius / distance).asin();
        let cone_angle = self.cone_angle_cos.acos();
        
        dot_product.acos() <= cone_angle + relax_angle
    }
}

#[derive(Debug, Clone)]
pub struct CylinderSplash {
    pub center: Vec3,
    pub radius: f32,
    pub height_min: f32,
    pub height_max: f32,
}

impl SplashGeometry for CylinderSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let dz = point.z - self.center.z;
        if dz < self.height_min || dz > self.height_max {
            return false;
        }

        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        (dx * dx + dy * dy) <= self.radius * self.radius
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let dz = center.z - self.center.z;
        if dz + radius < self.height_min || dz - radius > self.height_max {
            return false;
        }

        let dx = center.x - self.center.x;
        let dy = center.y - self.center.y;
        let effective_radius = self.radius + radius;
        
        (dx * dx + dy * dy) <= effective_radius * effective_radius
    }
}

#[derive(Debug, Clone)]
pub struct BoxSplash {
    pub origin: Vec3, // Base center
    pub right: Vec3,  // Local X axis (normalized)
    pub forward: Vec3,// Local Y axis (normalized)
    pub up: Vec3,     // Local Z axis (normalized)
    pub extents: Vec3, // Half sizes: X=width/2, Y=length/2, Z=height/2
}

impl SplashGeometry for BoxSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let diff = point - self.origin;
        
        let x = diff.dot(self.right);
        if x < -self.extents.x || x > self.extents.x { return false; }
        
        let y = diff.dot(self.forward);
        if y < -self.extents.y || y > self.extents.y { return false; }
        
        let z = diff.dot(self.up);
        if z < 0.0 || z > self.extents.z * 2.0 { return false; }
        
        true
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let diff = center - self.origin;
        
        let x = diff.dot(self.right);
        let y = diff.dot(self.forward);
        let z = diff.dot(self.up);
        
        // Find closest point on OBB to sphere center
        let closest_x = x.clamp(-self.extents.x, self.extents.x);
        let closest_y = y.clamp(-self.extents.y, self.extents.y);
        let closest_z = z.clamp(0.0, self.extents.z * 2.0);
        
        let dx = x - closest_x;
        let dy = y - closest_y;
        let dz = z - closest_z;
        
        (dx*dx + dy*dy + dz*dz) <= radius * radius
    }
}

#[derive(Debug, Clone)]
pub struct SphereSplash {
    pub center: Vec3,
    pub radius: f32,
}

impl SplashGeometry for SphereSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let diff = point - self.center;
        diff.length_squared() <= self.radius * self.radius
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let diff = center - self.center;
        let r = self.radius + radius;
        diff.length_squared() <= r * r
    }
}
