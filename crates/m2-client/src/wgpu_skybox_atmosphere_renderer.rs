use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtmosphereState {
    Day,
    Night,
    TransitioningToDay,
    TransitioningToNight,
}

pub struct SkyboxAtmosphereRenderer {
    pub view_proj_matrix: Mat4,
    pub current_ambient: [f32; 4],
    pub current_diffuse: [f32; 4],
    pub current_fog: [f32; 4],
    pub current_sun_dir: Vec3,

    pub day_ambient: [f32; 4],
    pub day_diffuse: [f32; 4],
    pub day_fog: [f32; 4],
    pub day_sun_dir: Vec3,

    pub night_ambient: [f32; 4],
    pub night_diffuse: [f32; 4],
    pub night_fog: [f32; 4],
    pub night_sun_dir: Vec3,

    pub transition_duration: f32,
    pub transition_progress: f32,
    pub state: AtmosphereState,
}

impl SkyboxAtmosphereRenderer {
    pub fn new() -> Self {
        Self {
            view_proj_matrix: Mat4::IDENTITY,
            current_ambient: [0.6, 0.6, 0.6, 1.0],
            current_diffuse: [1.0, 0.9, 0.8, 1.0],
            current_fog: [0.6, 0.7, 0.9, 1.0],
            current_sun_dir: Vec3::new(0.0, 1.0, 0.5).normalize(),

            day_ambient: [0.6, 0.6, 0.6, 1.0],
            day_diffuse: [1.0, 0.9, 0.8, 1.0],
            day_fog: [0.6, 0.7, 0.9, 1.0],
            day_sun_dir: Vec3::new(0.0, 1.0, 0.5).normalize(),

            night_ambient: [0.1, 0.1, 0.2, 1.0],
            night_diffuse: [0.2, 0.2, 0.4, 1.0],
            night_fog: [0.1, 0.1, 0.15, 1.0],
            night_sun_dir: Vec3::new(0.0, -1.0, 0.5).normalize(),

            transition_duration: 5.0, // Default 5 seconds
            transition_progress: 1.0,
            state: AtmosphereState::Day,
        }
    }

    pub fn update_camera(&mut self, mut view: Mat4, proj: Mat4) {
        // Block translation by zeroing out the w_axis
        view.w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0);
        self.view_proj_matrix = proj * view;
    }

    pub fn change_to_night(&mut self) {
        if self.state != AtmosphereState::Night && self.state != AtmosphereState::TransitioningToNight {
            if self.state == AtmosphereState::Day {
                self.transition_progress = 0.0;
            } else if self.state == AtmosphereState::TransitioningToDay {
                self.transition_progress = 1.0 - self.transition_progress;
            }
            self.state = AtmosphereState::TransitioningToNight;
        }
    }

    pub fn change_to_day(&mut self) {
        if self.state != AtmosphereState::Day && self.state != AtmosphereState::TransitioningToDay {
            if self.state == AtmosphereState::Night {
                self.transition_progress = 0.0;
            } else if self.state == AtmosphereState::TransitioningToNight {
                self.transition_progress = 1.0 - self.transition_progress;
            }
            self.state = AtmosphereState::TransitioningToDay;
        }
    }

    pub fn update_atmosphere_with_delta(&mut self, delta_sec: f32) {
        match self.state {
            AtmosphereState::Day => {
                self.current_ambient = self.day_ambient;
                self.current_diffuse = self.day_diffuse;
                self.current_fog = self.day_fog;
                self.current_sun_dir = self.day_sun_dir;
            }
            AtmosphereState::Night => {
                self.current_ambient = self.night_ambient;
                self.current_diffuse = self.night_diffuse;
                self.current_fog = self.night_fog;
                self.current_sun_dir = self.night_sun_dir;
            }
            AtmosphereState::TransitioningToNight => {
                self.transition_progress += delta_sec / self.transition_duration;
                if self.transition_progress >= 1.0 {
                    self.transition_progress = 1.0;
                    self.state = AtmosphereState::Night;
                }
                
                let t = self.transition_progress;
                self.current_ambient = Self::interpolate_color(self.day_ambient, self.night_ambient, t);
                self.current_diffuse = Self::interpolate_color(self.day_diffuse, self.night_diffuse, t);
                self.current_fog = Self::interpolate_color(self.day_fog, self.night_fog, t);
                self.current_sun_dir = self.day_sun_dir.lerp(self.night_sun_dir, t).normalize();
            }
            AtmosphereState::TransitioningToDay => {
                self.transition_progress += delta_sec / self.transition_duration;
                if self.transition_progress >= 1.0 {
                    self.transition_progress = 1.0;
                    self.state = AtmosphereState::Day;
                }
                
                let t = self.transition_progress;
                self.current_ambient = Self::interpolate_color(self.night_ambient, self.day_ambient, t);
                self.current_diffuse = Self::interpolate_color(self.night_diffuse, self.day_diffuse, t);
                self.current_fog = Self::interpolate_color(self.night_fog, self.day_fog, t);
                self.current_sun_dir = self.night_sun_dir.lerp(self.day_sun_dir, t).normalize();
            }
        }
    }

    pub fn interpolate_color(start: [f32; 4], end: [f32; 4], factor: f32) -> [f32; 4] {
        let t = factor.clamp(0.0, 1.0);
        [
            start[0] + (end[0] - start[0]) * t,
            start[1] + (end[1] - start[1]) * t,
            start[2] + (end[2] - start[2]) * t,
            start[3] + (end[3] - start[3]) * t,
        ]
    }
}

impl Default for SkyboxAtmosphereRenderer {
    fn default() -> Self {
        Self::new()
    }
}
