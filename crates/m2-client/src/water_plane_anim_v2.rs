use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

// Simple LCG matching standard C/C++ random generators
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state / 65536) % 32768
    }

    fn random_range(&mut self, min: i32, max: i32) -> i32 {
        if min == max {
            return min;
        }
        let diff = (max - min).abs() + 1;
        let r = self.next() as i32;
        min + (r % diff)
    }
}

pub struct WaterPlaneAnimV2 {
    pub current_height_offset: f32,
    pub height_begin: f32,
    pub height_end: f32,
    pub last_height_change_time: u32,
    pub blend_time: u32,
    
    pub u: f32,
    pub v: f32,
    pub speed_u: f32,
    pub speed_v: f32,
    
    rng: SimpleRng,
    
    sector_water_heights: HashMap<(i32, i32), f32>,
}

impl WaterPlaneAnimV2 {
    pub fn new(current_time: u32) -> Self {
        Self {
            current_height_offset: 0.0,
            height_begin: 0.0,
            height_end: 0.0,
            last_height_change_time: current_time,
            blend_time: 300,
            u: 0.0,
            v: 0.0,
            speed_u: 0.05,
            speed_v: 0.02,
            rng: SimpleRng::new(current_time.max(1)),
            sector_water_heights: HashMap::new(),
        }
    }

    pub fn set_uv_speed(&mut self, su: f32, sv: f32) {
        self.speed_u = su;
        self.speed_v = sv;
    }

    pub fn set_sector_water_height(&mut self, sector_x: i32, sector_y: i32, height: f32) {
        self.sector_water_heights.insert((sector_x, sector_y), height);
    }
    
    pub fn get_sector_water_height(&self, sector_x: i32, sector_y: i32) -> Option<f32> {
        self.sector_water_heights.get(&(sector_x, sector_y)).copied()
    }

    pub fn update(&mut self, current_time: u32, dt: f32) {
        // UV flow update
        self.u = (self.u + dt * self.speed_u).fract();
        self.v = (self.v + dt * self.speed_v).fract();
        
        if self.u < 0.0 { self.u += 1.0; }
        if self.v < 0.0 { self.v += 1.0; }

        // Time difference for height animation
        let time_diff = if current_time >= self.last_height_change_time {
            current_time - self.last_height_change_time
        } else {
            0
        };

        if time_diff > self.blend_time {
            self.blend_time = self.rng.random_range(1000, 3000) as u32;

            if self.height_end == 0.0 {
                self.height_end = -self.rng.random_range(0, 15) as f32;
            } else {
                self.height_end = 0.0;
            }

            self.height_begin = self.current_height_offset;
            self.last_height_change_time = current_time;
        }

        let new_time_diff = if current_time >= self.last_height_change_time {
            current_time - self.last_height_change_time
        } else {
            0
        };

        let progress = if self.blend_time > 0 {
            (new_time_diff as f32) / (self.blend_time as f32)
        } else {
            0.0
        };

        self.current_height_offset = self.height_begin + (self.height_end - self.height_begin) * progress.min(1.0);
    }

    pub fn is_underwater(&self, x: f32, y: f32, z: f32) -> bool {
        let sector_x = (x / 25600.0).floor() as i32;
        let sector_y = (y / 25600.0).floor() as i32;

        if let Some(base_height) = self.get_sector_water_height(sector_x, sector_y) {
            let water_level = base_height + self.current_height_offset;
            return z < water_level;
        }

        false
    }
}

pub static GLOBAL_WATER_ANIM: OnceLock<Mutex<WaterPlaneAnimV2>> = OnceLock::new();

pub fn get_global_water_anim() -> &'static Mutex<WaterPlaneAnimV2> {
    GLOBAL_WATER_ANIM.get_or_init(|| Mutex::new(WaterPlaneAnimV2::new(0)))
}

#[no_mangle]
pub extern "C" fn m2_water_is_under(x: f32, y: f32, z: f32) -> bool {
    let anim = get_global_water_anim().lock().unwrap();
    anim.is_underwater(x, y, z)
}

#[no_mangle]
pub extern "C" fn m2_water_anim_update(current_time: u32, dt: f32) {
    let mut anim = get_global_water_anim().lock().unwrap();
    anim.update(current_time, dt);
}

#[no_mangle]
pub extern "C" fn m2_water_set_sector_height(sector_x: i32, sector_y: i32, height: f32) {
    let mut anim = get_global_water_anim().lock().unwrap();
    anim.set_sector_water_height(sector_x, sector_y, height);
}

#[no_mangle]
pub extern "C" fn m2_water_anim_init(current_time: u32) {
    let mut anim = get_global_water_anim().lock().unwrap();
    *anim = WaterPlaneAnimV2::new(current_time);
}
