pub struct MinimapNavigation {
    pub map_width_units: f32,
    pub map_height_units: f32,
}

impl MinimapNavigation {
    pub fn new(map_width_units: f32, map_height_units: f32) -> Self {
        Self {
            map_width_units,
            map_height_units,
        }
    }

    pub fn world_to_minimap(&self, x: f32, y: f32) -> (f32, f32) {
        if self.map_width_units == 0.0 || self.map_height_units == 0.0 {
            return (0.0, 0.0);
        }

        let mini_x = (x / self.map_width_units) * 256.0;
        let mini_y = (y / self.map_height_units) * 256.0;

        (mini_x, mini_y)
    }

    pub fn calculate_arrow_rotation(character_rotation: f32) -> f32 {
        let mut f_rotation = 540.0 - character_rotation;
        
        while f_rotation >= 360.0 {
            f_rotation -= 360.0;
        }
        
        while f_rotation < 0.0 {
            f_rotation += 360.0;
        }
        
        f_rotation
    }
}
