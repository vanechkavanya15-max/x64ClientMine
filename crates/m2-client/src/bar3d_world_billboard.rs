#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3dxColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl D3dxColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

pub struct CharacterHpBar3D {
    pub current_hp: i32,
    pub max_hp: i32,
    pub bar_max_width: f32,
    pub bar_height: f32,
}

impl CharacterHpBar3D {
    pub fn new(current_hp: i32, max_hp: i32, bar_max_width: f32, bar_height: f32) -> Self {
        Self {
            current_hp,
            max_hp,
            bar_max_width,
            bar_height,
        }
    }

    pub fn calculate_width(&self) -> f32 {
        if self.max_hp <= 0 {
            return 0.0;
        }

        let hp_ratio = (self.current_hp as f32) / (self.max_hp as f32);
        let ratio_clamped = hp_ratio.max(0.0).min(1.0);
        
        ratio_clamped * self.bar_max_width
    }

    pub fn calculate_color(&self) -> D3dxColor {
        if self.max_hp <= 0 || self.current_hp <= 0 {
            return D3dxColor::new(1.0, 0.0, 0.0, 1.0); // Red
        }

        let hp_ratio = (self.current_hp as f32) / (self.max_hp as f32);

        if hp_ratio > 0.5 {
            // Green
            D3dxColor::new(0.0, 1.0, 0.0, 1.0)
        } else if hp_ratio > 0.25 {
            // Yellow
            D3dxColor::new(1.0, 1.0, 0.0, 1.0)
        } else {
            // Red
            D3dxColor::new(1.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn project_world_to_screen(
        world_x: f32, world_y: f32, world_z: f32,
        view_proj_matrix: &[f32; 16],
        screen_width: f32, screen_height: f32,
    ) -> Option<(f32, f32)> {
        let vp = view_proj_matrix;
        
        // clip_space = ViewProjection * (world_pos, 1.0)
        let clip_x = world_x * vp[0] + world_y * vp[4] + world_z * vp[8] + 1.0 * vp[12];
        let clip_y = world_x * vp[1] + world_y * vp[5] + world_z * vp[9] + 1.0 * vp[13];
        let clip_z = world_x * vp[2] + world_y * vp[6] + world_z * vp[10] + 1.0 * vp[14];
        let clip_w = world_x * vp[3] + world_y * vp[7] + world_z * vp[11] + 1.0 * vp[15];

        if clip_w <= 0.0 {
            return None; // Behind the camera
        }

        let ndc_x = clip_x / clip_w;
        let ndc_y = clip_y / clip_w;
        let _ndc_z = clip_z / clip_w;

        let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;

        Some((screen_x, screen_y))
    }
}
