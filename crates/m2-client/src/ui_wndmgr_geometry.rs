// ui_wndmgr_geometry.rs - Geometry and Scissor Clipping for wndMgr

#[derive(Debug, Clone, PartialEq)]
pub struct WindowGeometry {
    pub local_x: f32,
    pub local_y: f32,
    pub width: f32,
    pub height: f32,
    pub clip_rect: Option<[f32; 4]>, // [left, top, right, bottom]
}

impl WindowGeometry {
    pub fn new() -> Self {
        Self {
            local_x: 0.0,
            local_y: 0.0,
            width: 0.0,
            height: 0.0,
            clip_rect: None,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.local_x = x;
        self.local_y = y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.local_x, self.local_y)
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_global_position(&self, parent_global_pos: Option<(f32, f32)>) -> (f32, f32) {
        if let Some((px, py)) = parent_global_pos {
            (px + self.local_x, py + self.local_y)
        } else {
            (self.local_x, self.local_y)
        }
    }

    pub fn set_clip_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.clip_rect = Some([left, top, right, bottom]);
    }

    pub fn clear_clip_rect(&mut self) {
        self.clip_rect = None;
    }

    pub fn get_effective_clip_rect(&self, parent_clip_rect: Option<[f32; 4]>) -> Option<[f32; 4]> {
        match (self.clip_rect, parent_clip_rect) {
            (Some(my_clip), Some(parent_clip)) => {
                let left = my_clip[0].max(parent_clip[0]);
                let top = my_clip[1].max(parent_clip[1]);
                let right = my_clip[2].min(parent_clip[2]);
                let bottom = my_clip[3].min(parent_clip[3]);
                
                if left >= right || top >= bottom {
                    // Empty intersection
                    Some([left, top, left, top]) 
                } else {
                    Some([left, top, right, bottom])
                }
            }
            (Some(my_clip), None) => Some(my_clip),
            (None, Some(parent_clip)) => Some(parent_clip),
            (None, None) => None,
        }
    }

    pub fn is_in_rect(&self, px: f32, py: f32, global_pos: (f32, f32), effective_clip_rect: Option<[f32; 4]>) -> bool {
        let (gx, gy) = global_pos;
        let right = gx + self.width;
        let bottom = gy + self.height;

        // Check if within standard bounds
        if px < gx || px > right || py < gy || py > bottom {
            return false;
        }

        // Check if within effective clip rect
        if let Some(clip) = effective_clip_rect {
            if px < clip[0] || px > clip[2] || py < clip[1] || py > clip[3] {
                return false;
            }
        }

        true
    }
}
