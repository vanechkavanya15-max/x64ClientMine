use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActorTransform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot_deg: f32,
}

impl Default for ActorTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot_deg: 0.0,
        }
    }
}

impl ActorTransform {
    pub fn new(x: f32, y: f32, z: f32, rot_deg: f32) -> Self {
        let mut transform = Self { x, y, z, rot_deg };
        transform.normalize_rotation();
        transform
    }

    pub fn look_at(&mut self, target_x: f32, target_y: f32) {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        
        self.rot_deg = dx.atan2(dy) * (180.0 / PI);
        self.normalize_rotation();
    }

    pub fn normalize_rotation(&mut self) {
        self.rot_deg %= 360.0;
        if self.rot_deg < 0.0 {
            self.rot_deg += 360.0;
        }
    }

    pub fn distance_2d(&self, target_x: f32, target_y: f32) -> f32 {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn distance_3d(&self, target_x: f32, target_y: f32, target_z: f32) -> f32 {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let dz = target_z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
