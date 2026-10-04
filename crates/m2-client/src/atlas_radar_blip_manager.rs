#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlipType {
    Player,
    Party,
    Monster,
    Npc,
    MetinStone,
    Warp,
}

impl BlipType {
    pub fn get_color_argb(&self) -> u32 {
        match self {
            BlipType::Player => 0xFF00FF00,     // Green
            BlipType::Party => 0xFF0000FF,      // Blue
            BlipType::Monster => 0xFFFF0000,    // Red
            BlipType::Npc => 0xFFFFFF00,        // Yellow
            BlipType::MetinStone => 0xFFFF00FF, // Purple
            BlipType::Warp => 0xFF00FFFF,       // Cyan
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadarBlip {
    pub id: u32,
    pub blip_type: BlipType,
    pub x_cm: f32,
    pub y_cm: f32,
}

#[derive(Debug, Clone)]
pub struct SignalPoint {
    pub x_cm: f32,
    pub y_cm: f32,
    pub expire_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct BlipRenderInfo {
    pub id: u32,
    pub color_argb: u32,
    pub screen_x: f32,
    pub screen_y: f32,
}

#[derive(Debug, Clone)]
pub struct SignalRenderInfo {
    pub screen_x: f32,
    pub screen_y: f32,
}

pub struct AtlasRadarBlipManager {
    blips: Vec<RadarBlip>,
    signal_points: Vec<SignalPoint>,
    signal_fade_duration_ms: u64,
}

impl AtlasRadarBlipManager {
    pub fn new(signal_fade_duration_ms: u64) -> Self {
        Self {
            blips: Vec::new(),
            signal_points: Vec::new(),
            signal_fade_duration_ms,
        }
    }

    pub fn add_blip(&mut self, blip: RadarBlip) {
        if let Some(existing) = self.blips.iter_mut().find(|b| b.id == blip.id) {
            *existing = blip;
        } else {
            self.blips.push(blip);
        }
    }

    pub fn remove_blip(&mut self, id: u32) {
        self.blips.retain(|b| b.id != id);
    }

    pub fn clear_blips(&mut self) {
        self.blips.clear();
    }

    pub fn add_signal_point(&mut self, x_cm: f32, y_cm: f32, current_time_ms: u64) {
        self.signal_points.push(SignalPoint {
            x_cm,
            y_cm,
            expire_time_ms: current_time_ms + self.signal_fade_duration_ms,
        });
    }

    pub fn update_signal_points(&mut self, current_time_ms: u64) {
        self.signal_points.retain(|s| s.expire_time_ms > current_time_ms);
    }

    pub fn clear_signal_points(&mut self) {
        self.signal_points.clear();
    }

    pub fn get_radar_blips(
        &self,
        center_x_cm: f32,
        center_y_cm: f32,
        radar_radius_cm: f32,
        radar_screen_radius: f32,
        camera_rotation_deg: f32,
        radar_center_screen_x: f32,
        radar_center_screen_y: f32,
    ) -> Vec<BlipRenderInfo> {
        let mut render_info = Vec::new();

        let rad = camera_rotation_deg.to_radians();
        let cos_r = rad.cos();
        let sin_r = rad.sin();

        for blip in &self.blips {
            let dx = blip.x_cm - center_x_cm;
            let dy = blip.y_cm - center_y_cm;

            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= radar_radius_cm * radar_radius_cm {
                let scale = radar_screen_radius / radar_radius_cm;
                let scaled_dx = dx * scale;
                let scaled_dy = dy * scale;

                // 2D Rotation
                let rot_x = scaled_dx * cos_r - scaled_dy * sin_r;
                let rot_y = scaled_dx * sin_r + scaled_dy * cos_r;

                render_info.push(BlipRenderInfo {
                    id: blip.id,
                    color_argb: blip.blip_type.get_color_argb(),
                    screen_x: radar_center_screen_x + rot_x,
                    screen_y: radar_center_screen_y + rot_y,
                });
            }
        }

        render_info
    }

    pub fn get_radar_signal_points(
        &self,
        center_x_cm: f32,
        center_y_cm: f32,
        radar_radius_cm: f32,
        radar_screen_radius: f32,
        camera_rotation_deg: f32,
        radar_center_screen_x: f32,
        radar_center_screen_y: f32,
    ) -> Vec<SignalRenderInfo> {
        let mut render_info = Vec::new();
        let rad = camera_rotation_deg.to_radians();
        let cos_r = rad.cos();
        let sin_r = rad.sin();

        for signal in &self.signal_points {
            let dx = signal.x_cm - center_x_cm;
            let dy = signal.y_cm - center_y_cm;

            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= radar_radius_cm * radar_radius_cm {
                let scale = radar_screen_radius / radar_radius_cm;
                let scaled_dx = dx * scale;
                let scaled_dy = dy * scale;

                let rot_x = scaled_dx * cos_r - scaled_dy * sin_r;
                let rot_y = scaled_dx * sin_r + scaled_dy * cos_r;

                render_info.push(SignalRenderInfo {
                    screen_x: radar_center_screen_x + rot_x,
                    screen_y: radar_center_screen_y + rot_y,
                });
            }
        }
        render_info
    }

    pub fn get_atlas_blips(
        &self,
        atlas_start_x_cm: f32,
        atlas_start_y_cm: f32,
        atlas_width_cm: f32,
        atlas_height_cm: f32,
        atlas_screen_x: f32,
        atlas_screen_y: f32,
        atlas_screen_width: f32,
        atlas_screen_height: f32,
    ) -> Vec<BlipRenderInfo> {
        let mut render_info = Vec::new();

        let scale_x = atlas_screen_width / atlas_width_cm;
        let scale_y = atlas_screen_height / atlas_height_cm;

        for blip in &self.blips {
            if blip.x_cm >= atlas_start_x_cm
                && blip.x_cm <= atlas_start_x_cm + atlas_width_cm
                && blip.y_cm >= atlas_start_y_cm
                && blip.y_cm <= atlas_start_y_cm + atlas_height_cm
            {
                let dx = blip.x_cm - atlas_start_x_cm;
                let dy = blip.y_cm - atlas_start_y_cm;

                let screen_x = atlas_screen_x + (dx * scale_x);
                let screen_y = atlas_screen_y + (dy * scale_y);

                render_info.push(BlipRenderInfo {
                    id: blip.id,
                    color_argb: blip.blip_type.get_color_argb(),
                    screen_x,
                    screen_y,
                });
            }
        }

        render_info
    }

    pub fn get_atlas_signal_points(
        &self,
        atlas_start_x_cm: f32,
        atlas_start_y_cm: f32,
        atlas_width_cm: f32,
        atlas_height_cm: f32,
        atlas_screen_x: f32,
        atlas_screen_y: f32,
        atlas_screen_width: f32,
        atlas_screen_height: f32,
    ) -> Vec<SignalRenderInfo> {
        let mut render_info = Vec::new();

        let scale_x = atlas_screen_width / atlas_width_cm;
        let scale_y = atlas_screen_height / atlas_height_cm;

        for signal in &self.signal_points {
            if signal.x_cm >= atlas_start_x_cm
                && signal.x_cm <= atlas_start_x_cm + atlas_width_cm
                && signal.y_cm >= atlas_start_y_cm
                && signal.y_cm <= atlas_start_y_cm + atlas_height_cm
            {
                let dx = signal.x_cm - atlas_start_x_cm;
                let dy = signal.y_cm - atlas_start_y_cm;

                let screen_x = atlas_screen_x + (dx * scale_x);
                let screen_y = atlas_screen_y + (dy * scale_y);

                render_info.push(SignalRenderInfo {
                    screen_x,
                    screen_y,
                });
            }
        }

        render_info
    }
}
