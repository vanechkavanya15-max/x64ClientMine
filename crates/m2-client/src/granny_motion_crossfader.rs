use glam::{Vec3, Quat};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct BoneTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl BoneTransform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn blend(&self, target: &BoneTransform, weight: f32) -> Self {
        let t = weight.clamp(0.0, 1.0);
        
        let position = self.position.lerp(target.position, t);
        let scale = self.scale.lerp(target.scale, t);
        // Slerp for quaternions. Use slerp to properly interpolate rotations spherically
        let rotation = self.rotation.slerp(target.rotation, t);

        Self {
            position,
            rotation,
            scale,
        }
    }
}

pub struct MotionCrossfader {
    pub blend_duration: f32,
    pub current_time: f32,
    pub source_pose: Vec<BoneTransform>,
    pub target_pose: Vec<BoneTransform>,
}

impl MotionCrossfader {
    pub fn new(blend_duration: f32, source_pose: Vec<BoneTransform>, target_pose: Vec<BoneTransform>) -> Self {
        assert_eq!(source_pose.len(), target_pose.len(), "Source and target poses must have the same number of bones");
        Self {
            blend_duration: blend_duration.max(0.0),
            current_time: 0.0,
            source_pose,
            target_pose,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        if self.current_time > self.blend_duration {
            self.current_time = self.blend_duration;
        }
    }

    pub fn get_blend_weight(&self) -> f32 {
        if self.blend_duration <= 0.0 {
            return 1.0;
        }
        (self.current_time / self.blend_duration).clamp(0.0, 1.0)
    }

    pub fn get_blended_pose(&self) -> Vec<BoneTransform> {
        let weight = self.get_blend_weight();
        self.source_pose
            .iter()
            .zip(self.target_pose.iter())
            .map(|(source, target)| source.blend(target, weight))
            .collect()
    }
    
    pub fn is_finished(&self) -> bool {
        if self.blend_duration <= 0.0 {
            return true;
        }
        self.current_time >= self.blend_duration
    }
}
