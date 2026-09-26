#[derive(Debug, Clone)]
pub struct AtlasMapScaler {
    pub atlas_image_size_x: f32,
    pub atlas_image_size_y: f32,
    pub atlas_max_x: f32,
    pub atlas_max_y: f32,
    pub atlas_offset_x: f32,
    pub atlas_offset_y: f32,
}

impl Default for AtlasMapScaler {
    fn default() -> Self {
        Self {
            atlas_image_size_x: 512.0,
            atlas_image_size_y: 512.0,
            atlas_max_x: 0.0,
            atlas_max_y: 0.0,
            atlas_offset_x: 0.0,
            atlas_offset_y: 0.0,
        }
    }
}

impl AtlasMapScaler {
    pub fn new(image_width: f32, image_height: f32, map_width: f32, map_height: f32) -> Self {
        let mut scaler = Self {
            atlas_image_size_x: image_width,
            atlas_image_size_y: image_height,
            atlas_max_x: map_width,
            atlas_max_y: map_height,
            atlas_offset_x: 0.0,
            atlas_offset_y: 0.0,
        };
        scaler.recalculate_offsets();
        scaler
    }

    pub fn set_map_size(&mut self, width: f32, height: f32) {
        self.atlas_max_x = width;
        self.atlas_max_y = height;
        self.recalculate_offsets();
    }

    pub fn set_image_size(&mut self, width: f32, height: f32) {
        self.atlas_image_size_x = width;
        self.atlas_image_size_y = height;
        self.recalculate_offsets();
    }

    fn recalculate_offsets(&mut self) {
        if self.atlas_max_x <= 0.0 || self.atlas_max_y <= 0.0 {
            self.atlas_offset_x = 0.0;
            self.atlas_offset_y = 0.0;
            return;
        }

        let scale = self.get_atlas_uniform_scale();
        let scaled_width = self.atlas_max_x * scale;
        let scaled_height = self.atlas_max_y * scale;

        self.atlas_offset_x = (self.atlas_image_size_x - scaled_width) * 0.5;
        self.atlas_offset_y = (self.atlas_image_size_y - scaled_height) * 0.5;
    }

    pub fn get_atlas_uniform_scale(&self) -> f32 {
        if self.atlas_max_x <= 0.0 || self.atlas_max_y <= 0.0 {
            return 1.0;
        }

        let scale_x = self.atlas_image_size_x / self.atlas_max_x;
        let scale_y = self.atlas_image_size_y / self.atlas_max_y;
        
        // Return min of scale_x and scale_y (mimics std::min in C++)
        if scale_x < scale_y {
            scale_x
        } else {
            scale_y
        }
    }

    pub fn global_position_to_atlas_position(&self, lx: f32, ly: f32) -> (f32, f32) {
        let uniform_scale = self.get_atlas_uniform_scale();
        let px = lx * uniform_scale + self.atlas_offset_x;
        let py = ly * uniform_scale + self.atlas_offset_y;
        (px, py)
    }

    pub fn atlas_position_to_global_position(&self, atlas_x: f32, atlas_y: f32) -> (f32, f32) {
        let scale = self.get_atlas_uniform_scale();
        if scale == 0.0 {
            return (0.0, 0.0); // Prevent division by zero
        }
        
        let reverse_scale = 1.0 / scale;
        let world_x = (atlas_x - self.atlas_offset_x) * reverse_scale;
        let world_y = (atlas_y - self.atlas_offset_y) * reverse_scale;
        (world_x, world_y)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasPosition {
    pub x: f32,
    pub y: f32,
}

#[no_mangle]
pub extern "C" fn m2_atlas_calc_pos(world_x: f32, world_y: f32, map_w: f32, map_h: f32) -> AtlasPosition {
    let scaler = AtlasMapScaler::new(512.0, 512.0, map_w, map_h);
    let (ax, ay) = scaler.global_position_to_atlas_position(world_x, world_y);
    AtlasPosition { x: ax, y: ay }
}
