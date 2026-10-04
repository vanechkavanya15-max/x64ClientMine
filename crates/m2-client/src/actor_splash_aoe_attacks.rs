use glam::Vec3;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SSplashArea {
    pub is_enable_hit_process: bool,
    pub skill: u32,
    pub motion_key: u32,
    pub disappearing_time: f32, // Equivalent to fDisappearingTime
    
    // Properties for area calculations
    pub center_position: Vec3,
    pub radius: f32,
    pub cone_angle_deg: f32,
    pub rotation: f32, // Yaw rotation in degrees
    
    pub hit_limit_count: u32,
    pub hitted_instance_map: HashSet<u32>, // Store VIDs of hitted instances
}

impl SSplashArea {
    pub fn new() -> Self {
        Self {
            is_enable_hit_process: false,
            skill: 0,
            motion_key: 0,
            disappearing_time: 0.0,
            center_position: Vec3::ZERO,
            radius: 0.0,
            cone_angle_deg: 0.0,
            rotation: 0.0,
            hit_limit_count: 0,
            hitted_instance_map: HashSet::new(),
        }
    }

    pub fn setup(
        &mut self,
        center_position: Vec3,
        radius: f32,
        cone_angle_deg: f32,
        rotation: f32,
        duration: f32,
        hit_limit_count: u32,
        current_time: f32,
    ) {
        self.center_position = center_position;
        self.radius = radius;
        self.cone_angle_deg = cone_angle_deg;
        self.rotation = rotation;
        self.disappearing_time = current_time + duration;
        self.hit_limit_count = hit_limit_count;
        self.hitted_instance_map.clear();
        self.is_enable_hit_process = true;
    }

    pub fn update_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }
    
    pub fn update_position(&mut self, new_position: Vec3) {
        self.center_position = new_position;
    }

    pub fn is_in_splash_time(&self, current_time: f32) -> bool {
        self.disappearing_time > current_time
    }

    pub fn check_hit(&mut self, target_vid: u32, target_pos: Vec3, current_time: f32) -> bool {
        if !self.is_enable_hit_process {
            return false;
        }

        if !self.is_in_splash_time(current_time) {
            return false;
        }

        if self.hitted_instance_map.len() as u32 >= self.hit_limit_count && self.hit_limit_count > 0 {
            return false;
        }

        if self.hitted_instance_map.contains(&target_vid) {
            return false; // Already hitted
        }

        let diff = target_pos - self.center_position;
        let distance_sq = diff.length_squared();

        if distance_sq > self.radius * self.radius {
            return false;
        }

        // Calculate if target is within the cone
        if distance_sq > 0.0 && self.cone_angle_deg > 0.0 && self.cone_angle_deg < 360.0 {
            let distance = distance_sq.sqrt();
            let dir_to_target = diff / distance;
            
            // Assume 0 rotation means pointing towards negative Y (or positive Y based on Metin2 coordinates, usually forward is +Y or +X depending on the engine. Let's use standard +Y for Metin2 orientation typically).
            // Actually Metin2 0 degrees rotation typically points to -Y. Let's convert rotation to direction vector.
            // Wait, in standard 3D math, angle 0 is +X.
            // Let's use a simple rotation around Z axis assuming XY is ground plane.
            // direction = (sin(rot), -cos(rot), 0) is a common Metin2 dir vector for rotation.
            
            let rot_rad = self.rotation.to_radians();
            let forward_dir = Vec3::new(-rot_rad.sin(), rot_rad.cos(), 0.0).normalize_or_zero();
            
            let dot_product = forward_dir.dot(dir_to_target);
            let half_cone_angle_rad = (self.cone_angle_deg / 2.0).to_radians();
            let cone_angle_cos = half_cone_angle_rad.cos();
            
            if dot_product < cone_angle_cos {
                return false;
            }
        }

        self.hitted_instance_map.insert(target_vid);
        true
    }
}
