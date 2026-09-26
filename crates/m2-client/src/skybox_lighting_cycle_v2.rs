use glam::Vec3;
use std::time::Instant;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3dColorValue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for D3dColorValue {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3dLight9 {
    pub type_: u32,
    pub diffuse: D3dColorValue,
    pub specular: D3dColorValue,
    pub ambient: D3dColorValue,
    pub position: Vec3,
    pub direction: Vec3,
    pub range: f32,
    pub falloff: f32,
    pub attenuation0: f32,
    pub attenuation1: f32,
    pub attenuation2: f32,
    pub theta: f32,
    pub phi: f32,
}

impl Default for D3dLight9 {
    fn default() -> Self {
        Self {
            type_: 0,
            diffuse: D3dColorValue::default(),
            specular: D3dColorValue::default(),
            ambient: D3dColorValue::default(),
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            range: 0.0,
            falloff: 0.0,
            attenuation0: 0.0,
            attenuation1: 0.0,
            attenuation2: 0.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

pub const ENV_DIRLIGHT_BACKGROUND: usize = 0;
pub const ENV_DIRLIGHT_CHARACTER: usize = 1;
pub const ENV_DIRLIGHT_NUM: usize = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for TColor {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TGradientColor {
    pub first_color: TColor,
    pub second_color: TColor,
}

pub type BOOL = i32;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3dMaterial9 {
    pub diffuse: D3dColorValue,
    pub ambient: D3dColorValue,
    pub specular: D3dColorValue,
    pub emissive: D3dColorValue,
    pub power: f32,
}

impl Default for D3dMaterial9 {
    fn default() -> Self {
        Self {
            diffuse: D3dColorValue::default(),
            ambient: D3dColorValue::default(),
            specular: D3dColorValue::default(),
            emissive: D3dColorValue::default(),
            power: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SEnvironmentData {
    pub dir_lights_enable: [BOOL; ENV_DIRLIGHT_NUM],
    pub dir_lights: [D3dLight9; ENV_DIRLIGHT_NUM],

    pub material: D3dMaterial9,

    pub fog_enable: BOOL,
    pub density_fog: BOOL,
    pub fog_near_distance: f32,
    pub fog_far_distance: f32,
    pub fog_color: u32, // ARGB D3DCOLOR
    pub fog_level: u8,

    pub cloud_gradient_color: TGradientColor,
    pub skybox_gradient_color_vector: *const TGradientColor,
    pub skybox_gradient_color_vector_len: usize,
}

impl Default for SEnvironmentData {
    fn default() -> Self {
        Self {
            dir_lights_enable: [FALSE; ENV_DIRLIGHT_NUM],
            dir_lights: [D3dLight9::default(); ENV_DIRLIGHT_NUM],
            material: D3dMaterial9::default(),
            fog_enable: FALSE,
            density_fog: FALSE,
            fog_near_distance: 0.0,
            fog_far_distance: 0.0,
            fog_color: 0,
            fog_level: 0,
            cloud_gradient_color: TGradientColor::default(),
            skybox_gradient_color_vector: std::ptr::null(),
            skybox_gradient_color_vector_len: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorTransitionHelper {
    src_color: TColor,
    dst_color: TColor,
    cur_color: u32,
    duration: u32,
    start_time: Option<Instant>,
    pub transition_started: bool,
}

impl ColorTransitionHelper {
    pub fn new() -> Self {
        let mut helper = Self {
            src_color: TColor::default(),
            dst_color: TColor::default(),
            cur_color: 0,
            duration: 0,
            start_time: None,
            transition_started: false,
        };
        helper.clear(0.0, 0.0, 0.0, 0.0);
        helper
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.src_color = TColor { r, g, b, a };
        self.dst_color = self.src_color;
        self.cur_color = 0;
        self.duration = 0;
        self.start_time = None;
    }

    pub fn set_src_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.src_color = TColor { r, g, b, a };
    }

    pub fn set_transition(&mut self, r: f32, g: f32, b: f32, a: f32, duration: u32) {
        self.dst_color = TColor { r, g, b, a };
        self.duration = duration;
    }

    pub fn start_transition(&mut self) {
        self.transition_started = true;
        self.start_time = Some(Instant::now());
    }

    pub fn update(&mut self) -> bool {
        let elapsed = if let Some(start) = self.start_time {
            start.elapsed().as_millis() as u32
        } else {
            0
        };

        let mut percent = if self.duration == 0 {
            1.0
        } else {
            elapsed as f32 / self.duration as f32
        };

        if percent < 0.0 {
            percent = 0.0;
        }
        if percent > 1.0 {
            percent = 1.0;
        }

        let cur_r = self.src_color.r + (self.dst_color.r - self.src_color.r) * percent;
        let cur_g = self.src_color.g + (self.dst_color.g - self.src_color.g) * percent;
        let cur_b = self.src_color.b + (self.dst_color.b - self.src_color.b) * percent;
        let cur_a = self.src_color.a + (self.dst_color.a - self.src_color.a) * percent;

        self.cur_color = (((cur_a * 255.0) as u32 & 0xff) << 24)
            | (((cur_r * 255.0) as u32 & 0xff) << 16)
            | (((cur_g * 255.0) as u32 & 0xff) << 8)
            | ((cur_b * 255.0) as u32 & 0xff);

        if percent >= 1.0
            && cur_a == self.dst_color.a
            && cur_r == self.dst_color.r
            && cur_g == self.dst_color.g
            && cur_b == self.dst_color.b
        {
            self.transition_started = false;
            return false;
        }

        true
    }

    pub fn get_cur_color(&self) -> u32 {
        self.cur_color
    }

    // A test helper to mock time
    pub fn update_with_elapsed(&mut self, elapsed_ms: u32) -> bool {
        let mut percent = if self.duration == 0 {
            1.0
        } else {
            elapsed_ms as f32 / self.duration as f32
        };

        if percent < 0.0 {
            percent = 0.0;
        }
        if percent > 1.0 {
            percent = 1.0;
        }

        let cur_r = self.src_color.r + (self.dst_color.r - self.src_color.r) * percent;
        let cur_g = self.src_color.g + (self.dst_color.g - self.src_color.g) * percent;
        let cur_b = self.src_color.b + (self.dst_color.b - self.src_color.b) * percent;
        let cur_a = self.src_color.a + (self.dst_color.a - self.src_color.a) * percent;

        self.cur_color = (((cur_a * 255.0) as u32 & 0xff) << 24)
            | (((cur_r * 255.0) as u32 & 0xff) << 16)
            | (((cur_g * 255.0) as u32 & 0xff) << 8)
            | ((cur_b * 255.0) as u32 & 0xff);

        if percent >= 1.0
            && cur_a == self.dst_color.a
            && cur_r == self.dst_color.r
            && cur_g == self.dst_color.g
            && cur_b == self.dst_color.b
        {
            self.transition_started = false;
            return false;
        }

        true
    }
}

pub struct SkyBox {
    pub cloud_helpers: Vec<ColorTransitionHelper>,
    pub sky_helpers: Vec<ColorTransitionHelper>,
}

impl SkyBox {
    pub fn new() -> Self {
        Self {
            cloud_helpers: Vec::new(),
            sky_helpers: Vec::new(),
        }
    }

    pub fn set_cloud_color(
        &mut self,
        color: &TGradientColor,
        next_color: &TGradientColor,
        transition_time: u32,
    ) {
        let mut helper = ColorTransitionHelper::new();
        helper.set_src_color(
            color.first_color.r,
            color.first_color.g,
            color.first_color.b,
            color.first_color.a,
        );
        helper.set_transition(
            next_color.first_color.r,
            next_color.first_color.g,
            next_color.first_color.b,
            next_color.first_color.a,
            transition_time,
        );
        self.cloud_helpers.push(helper);
    }

    pub fn set_sky_color(
        &mut self,
        color_vector: *const TGradientColor,
        color_vector_len: usize,
        next_color_vector: *const TGradientColor,
        next_color_vector_len: usize,
        transition_time: u32,
    ) {
        if color_vector.is_null() || next_color_vector.is_null() {
            return;
        }

        let colors = unsafe { std::slice::from_raw_parts(color_vector, color_vector_len) };
        let next_colors = unsafe { std::slice::from_raw_parts(next_color_vector, next_color_vector_len) };

        for (color, next_color) in colors.iter().zip(next_colors.iter()) {
            let mut helper = ColorTransitionHelper::new();
            helper.set_src_color(
                color.first_color.r,
                color.first_color.g,
                color.first_color.b,
                color.first_color.a,
            );
            helper.set_transition(
                next_color.first_color.r,
                next_color.first_color.g,
                next_color.first_color.b,
                next_color.first_color.a,
                transition_time,
            );
            self.sky_helpers.push(helper);
        }
    }

    pub fn start_transition(&mut self) {
        for helper in &mut self.cloud_helpers {
            helper.start_transition();
        }
        for helper in &mut self.sky_helpers {
            helper.start_transition();
        }
    }
}

#[repr(C)]
pub struct FogParamsFFI {
    pub near_distance: f32,
    pub far_distance: f32,
    pub color: u32,
}

pub fn calculate_fog_params(
    env_data: &SEnvironmentData,
    python_fog_level: usize,
) -> FogParamsFFI {
    if env_data.fog_enable == 0 {
        return FogParamsFFI {
            near_distance: 0.0,
            far_distance: 0.0,
            color: env_data.fog_color,
        };
    }

    // Python fog level: 2=Dense, 1=Middle, 0=Light
    // From MapManager.cpp: fFogScaleLevel = { 0.75f, 1.0f, 1.25f }
    let fog_scale_level: [f32; 3] = [0.75, 1.0, 1.25];
    let scale = if python_fog_level < 3 {
        fog_scale_level[python_fog_level]
    } else {
        1.0
    };

    let fog_near = env_data.fog_near_distance * scale;
    let fog_far = env_data.fog_far_distance * scale;

    FogParamsFFI {
        near_distance: fog_near,
        far_distance: fog_far,
        color: env_data.fog_color,
    }
}

#[no_mangle]
pub extern "C" fn m2_skybox_get_fog_params(
    env_data: *const SEnvironmentData,
    python_fog_level: usize,
) -> FogParamsFFI {
    if env_data.is_null() {
        return FogParamsFFI {
            near_distance: 0.0,
            far_distance: 0.0,
            color: 0,
        };
    }

    let data = unsafe { &*env_data };
    calculate_fog_params(data, python_fog_level)
}
