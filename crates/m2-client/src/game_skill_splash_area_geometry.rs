use glam::Vec3;

/// Trait defining the contract for splash area geometries used in game skills.
pub trait SplashGeometry {
    /// Checks if a single 3D point is within the splash geometry.
    fn is_point_inside(&self, point: Vec3) -> bool;
    /// Checks if a bounding sphere intersects or is within the splash geometry.
    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool;
}

/// Represents a cone-shaped attack area (e.g., Sword Spin / Wir Miecza).
#[derive(Debug, Clone)]
pub struct ConeSplash {
    pub origin: Vec3,
    pub direction: Vec3, // Must be a normalized vector
    pub radius: f32,
    pub cone_angle_cos: f32, // The cosine of half the cone's opening angle
}

impl SplashGeometry for ConeSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let offset = point - self.origin;
        let dist_sq = offset.length_squared();

        // Check distance boundary
        if dist_sq > self.radius * self.radius {
            return false;
        }

        // Exact origin match
        if dist_sq <= f32::EPSILON {
            return true;
        }

        let dist = dist_sq.sqrt();
        let target_dir = offset / dist;
        let dot_val = self.direction.dot(target_dir);

        // Check angle boundary
        dot_val >= self.cone_angle_cos
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let offset = center - self.origin;
        let dist_sq = offset.length_squared();
        
        let expanded_radius = self.radius + radius;
        if dist_sq > expanded_radius * expanded_radius {
            return false;
        }

        let dist = dist_sq.sqrt();
        if dist <= radius {
            return true;
        }

        let target_dir = offset / dist;
        let dot_val = self.direction.dot(target_dir);

        // Approximate relaxation of angle for the sphere size
        let relax_ang = (radius / dist).asin();
        let cone_ang = self.cone_angle_cos.acos();
        
        dot_val.acos() <= cone_ang + relax_ang
    }
}

/// Represents a cylindrical area around the caster (e.g., Ghost Strike / Duch Duchow).
#[derive(Debug, Clone)]
pub struct CylinderSplash {
    pub center: Vec3,
    pub radius: f32,
    pub height_min: f32,
    pub height_max: f32,
}

impl SplashGeometry for CylinderSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let delta_z = point.z - self.center.z;
        if delta_z < self.height_min || delta_z > self.height_max {
            return false;
        }

        let delta_x = point.x - self.center.x;
        let delta_y = point.y - self.center.y;
        (delta_x.powi(2) + delta_y.powi(2)) <= self.radius.powi(2)
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let delta_z = center.z - self.center.z;
        if (delta_z + radius) < self.height_min || (delta_z - radius) > self.height_max {
            return false;
        }

        let delta_x = center.x - self.center.x;
        let delta_y = center.y - self.center.y;
        let expanded_radius = self.radius + radius;
        
        (delta_x.powi(2) + delta_y.powi(2)) <= expanded_radius.powi(2)
    }
}

/// Represents a rectangular box/line attack area (e.g., Charge / Szarza).
#[derive(Debug, Clone)]
pub struct BoxSplash {
    pub origin: Vec3, // Center point of the base
    pub right: Vec3,  // Normalized X-axis vector
    pub forward: Vec3,// Normalized Y-axis vector
    pub up: Vec3,     // Normalized Z-axis vector
    pub extents: Vec3, // Half extents: width/2, length/2, height/2
}

impl SplashGeometry for BoxSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let offset = point - self.origin;
        
        let proj_x = offset.dot(self.right);
        if proj_x < -self.extents.x || proj_x > self.extents.x { return false; }
        
        let proj_y = offset.dot(self.forward);
        if proj_y < -self.extents.y || proj_y > self.extents.y { return false; }
        
        let proj_z = offset.dot(self.up);
        if proj_z < 0.0 || proj_z > self.extents.z * 2.0 { return false; }
        
        true
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let offset = center - self.origin;
        
        let proj_x = offset.dot(self.right);
        let proj_y = offset.dot(self.forward);
        let proj_z = offset.dot(self.up);
        
        // Clamp to find the closest point inside the OBB
        let closest_x = proj_x.clamp(-self.extents.x, self.extents.x);
        let closest_y = proj_y.clamp(-self.extents.y, self.extents.y);
        let closest_z = proj_z.clamp(0.0, self.extents.z * 2.0);
        
        let delta_x = proj_x - closest_x;
        let delta_y = proj_y - closest_y;
        let delta_z = proj_z - closest_z;
        
        (delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2)) <= radius.powi(2)
    }
}

/// Represents a simple spherical splash area (fallback/general usage).
#[derive(Debug, Clone)]
pub struct SphereSplash {
    pub center: Vec3,
    pub radius: f32,
}

impl SplashGeometry for SphereSplash {
    fn is_point_inside(&self, point: Vec3) -> bool {
        let offset = point - self.center;
        offset.length_squared() <= self.radius.powi(2)
    }

    fn is_sphere_inside(&self, center: Vec3, radius: f32) -> bool {
        let offset = center - self.center;
        let combined_radius = self.radius + radius;
        offset.length_squared() <= combined_radius.powi(2)
    }
}
