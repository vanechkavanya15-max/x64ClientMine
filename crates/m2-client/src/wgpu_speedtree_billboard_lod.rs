#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedTreeRenderMode {
    Full3D,
    Transition,
    Billboard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpeedTreeLodState {
    pub mode: SpeedTreeRenderMode,
    pub lod_value: f32,
    pub tree_alpha: f32,
    pub billboard_alpha: f32,
}

pub const NEAR_LOD_FACTOR: f32 = 2.0;
pub const FAR_LOD_FACTOR: f32 = 9.0;

pub struct SpeedTreeLodEvaluator {
    pub near_distance: f32,
    pub far_distance: f32,
}

impl SpeedTreeLodEvaluator {
    pub fn new(near_distance: f32, far_distance: f32) -> Self {
        Self {
            near_distance,
            far_distance,
        }
    }

    pub fn from_tree_height(height: f32) -> Self {
        Self {
            near_distance: height * NEAR_LOD_FACTOR,
            far_distance: height * FAR_LOD_FACTOR,
        }
    }

    pub fn from_bounding_box(bbox: &[f32; 6]) -> Self {
        let height = bbox[5] - bbox[2];
        Self::from_tree_height(height)
    }

    fn calculate_distance(camera_pos: [f32; 3], tree_pos: [f32; 3]) -> f32 {
        let dx = camera_pos[0] - tree_pos[0];
        let dy = camera_pos[1] - tree_pos[1];
        let dz = camera_pos[2] - tree_pos[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn evaluate(&self, camera_pos: [f32; 3], tree_pos: [f32; 3]) -> SpeedTreeLodState {
        let distance = Self::calculate_distance(camera_pos, tree_pos);

        if distance < self.near_distance {
            SpeedTreeLodState {
                mode: SpeedTreeRenderMode::Full3D,
                lod_value: 0.0,
                tree_alpha: 1.0,
                billboard_alpha: 0.0,
            }
        } else if distance >= self.far_distance {
            SpeedTreeLodState {
                mode: SpeedTreeRenderMode::Billboard,
                lod_value: 1.0,
                tree_alpha: 0.0,
                billboard_alpha: 1.0,
            }
        } else {
            let range = self.far_distance - self.near_distance;
            if range <= 0.0 {
                return SpeedTreeLodState {
                    mode: SpeedTreeRenderMode::Billboard,
                    lod_value: 1.0,
                    tree_alpha: 0.0,
                    billboard_alpha: 1.0,
                };
            }

            let lod_value = (distance - self.near_distance) / range;
            
            SpeedTreeLodState {
                mode: SpeedTreeRenderMode::Transition,
                lod_value,
                tree_alpha: 1.0 - lod_value,
                billboard_alpha: lod_value,
            }
        }
    }
}

pub struct SpeedTreeBillboardGenerator {
    pub width: f32,
    pub height: f32,
}

impl SpeedTreeBillboardGenerator {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn generate_billboard_quad(&self, tree_pos: [f32; 3], camera_pos: [f32; 3]) -> [[f32; 3]; 4] {
        let dx = camera_pos[0] - tree_pos[0];
        let dy = camera_pos[1] - tree_pos[1];

        let dist_2d = (dx * dx + dy * dy).sqrt();

        let (right_x, right_y) = if dist_2d > 0.0 {
            (dy / dist_2d, -dx / dist_2d)
        } else {
            (1.0, 0.0)
        };

        let half_w = self.width * 0.5;
        let h = self.height;

        let bottom_left = [
            tree_pos[0] - right_x * half_w,
            tree_pos[1] - right_y * half_w,
            tree_pos[2],
        ];

        let bottom_right = [
            tree_pos[0] + right_x * half_w,
            tree_pos[1] + right_y * half_w,
            tree_pos[2],
        ];

        let top_right = [
            tree_pos[0] + right_x * half_w,
            tree_pos[1] + right_y * half_w,
            tree_pos[2] + h,
        ];

        let top_left = [
            tree_pos[0] - right_x * half_w,
            tree_pos[1] - right_y * half_w,
            tree_pos[2] + h,
        ];

        [bottom_left, bottom_right, top_right, top_left]
    }
}
