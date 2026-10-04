use glam::{Mat4, Quat, Vec3};

/// Represents a single keyframe in a Daider / OBB animation curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DaiderObbKey {
    pub time: f32,
    pub packed_rotation: [f32; 3], // x, y, z
}

impl DaiderObbKey {
    pub fn new(time: f32, x: f32, y: f32, z: f32) -> Self {
        Self {
            time,
            packed_rotation: [x, y, z],
        }
    }

    /// Decodes the packed rotation into a full normalized quaternion.
    /// Reconstructs the W component as `sqrt(1.0 - x^2 - y^2 - z^2)`.
    pub fn decode_rotation(&self) -> Quat {
        let [x, y, z] = self.packed_rotation;
        let sum_sq = x * x + y * y + z * z;
        let w = if sum_sq > 1.0 {
            0.0
        } else {
            (1.0 - sum_sq).sqrt()
        };
        let q = Quat::from_xyzw(x, y, z, w);
        q.normalize()
    }
}

/// Represents a rotation curve of decoded or packed keys over time.
#[derive(Debug, Clone, Default)]
pub struct DaiderObbCurve {
    pub keys: Vec<DaiderObbKey>,
}

impl DaiderObbCurve {
    pub fn new(keys: Vec<DaiderObbKey>) -> Self {
        let mut curve = Self { keys };
        // Ensure keys are sorted by time
        curve.keys.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
        curve
    }

    /// Evaluates the curve at a specific local time, returning the interpolated quaternion.
    pub fn evaluate_rotation(&self, local_time: f32) -> Quat {
        if self.keys.is_empty() {
            return Quat::IDENTITY;
        }

        if self.keys.len() == 1 || local_time <= self.keys.first().unwrap().time {
            return self.keys.first().unwrap().decode_rotation();
        }

        if local_time >= self.keys.last().unwrap().time {
            return self.keys.last().unwrap().decode_rotation();
        }

        // Find the bounding keys
        let mut idx = 0;
        for i in 0..self.keys.len() - 1 {
            if local_time >= self.keys[i].time && local_time < self.keys[i + 1].time {
                idx = i;
                break;
            }
        }

        let key1 = &self.keys[idx];
        let key2 = &self.keys[idx + 1];

        let q1 = key1.decode_rotation();
        let mut q2 = key2.decode_rotation();

        // Shortest path dot product check is built into glam's slerp, but we need
        // to implement the explicit check to ensure shortest path on SO(3) 
        // in cases where the decoder gives us components that are flipped.
        if q1.dot(q2) < 0.0 {
            q2 = Quat::from_xyzw(-q2.x, -q2.y, -q2.z, -q2.w);
        }

        let t = (local_time - key1.time) / (key2.time - key1.time);
        
        // Use lerp + normalize to perfectly match standard Granny Shortest path Slerp exactly as it works under the hood
        // or just rely on glam slerp if the inputs are aligned.
        q1.slerp(q2, t)
    }
}

/// Helper function to generate a local transformation matrix for a bone at a specific time.
pub fn generate_local_bone_matrix(
    curve: &DaiderObbCurve,
    local_time: f32,
    position: Vec3,
    scale: Vec3,
) -> Mat4 {
    let rotation = curve.evaluate_rotation(local_time);
    Mat4::from_scale_rotation_translation(scale, rotation, position)
}

