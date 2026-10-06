//! UI Character Status Gauge V2
//! Implements gauge progress bars, HP flash animation, and the 4-orb EXP gauge.
//! Parity with Metin2 C++ gauge systems.

use std::sync::{Mutex, OnceLock};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectClip {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GaugeType {
    Hp = 0,
    Sp = 1,
    Stamina = 2,
}

#[derive(Debug, Clone)]
pub struct GaugeState {
    pub current_value: f64,
    pub max_value: f64,
    pub display_value: f64,
    pub width: f32,
    pub height: f32,
    
    // Flash animation state
    pub is_flashing: bool,
    pub flash_alpha: f32,
    pub flash_timer: f64,
}

impl GaugeState {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            current_value: 0.0,
            max_value: 100.0,
            display_value: 0.0,
            width,
            height,
            is_flashing: false,
            flash_alpha: 0.0,
            flash_timer: 0.0,
        }
    }

    pub fn set_value(&mut self, value: f64, max_value: f64) {
        let old_value = self.current_value;
        self.max_value = if max_value <= 0.0 { 1.0 } else { max_value };
        self.current_value = value.clamp(0.0, self.max_value);
        
        // HP Flash logic on decrease
        if self.current_value < old_value {
            self.is_flashing = true;
            self.flash_alpha = 1.0;
            self.flash_timer = 0.0;
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Smooth gauge display interpolation
        let diff = self.current_value - self.display_value;
        if diff.abs() > 0.001 {
            self.display_value += diff * (10.0 * dt).clamp(0.0, 1.0);
        } else {
            self.display_value = self.current_value;
        }

        // Flash animation interpolation
        if self.is_flashing {
            self.flash_timer += dt;
            self.flash_alpha -= (dt * 2.0) as f32; // Fade out over 0.5s
            if self.flash_alpha <= 0.0 {
                self.flash_alpha = 0.0;
                self.is_flashing = false;
            }
        }
    }

    pub fn get_percentage(&self) -> f64 {
        if self.max_value <= 0.0 {
            0.0
        } else {
            (self.display_value / self.max_value).clamp(0.0, 1.0)
        }
    }

    pub fn get_clip_rect(&self) -> RectClip {
        let percentage = self.get_percentage() as f32;
        RectClip {
            left: 0.0,
            top: 0.0,
            right: self.width * percentage,
            bottom: self.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpTaskbarGauge {
    pub exp: u64,
    pub next_exp: u64,
}

impl ExpTaskbarGauge {
    pub fn new() -> Self {
        Self {
            exp: 0,
            next_exp: 100,
        }
    }

    pub fn set_exp(&mut self, exp: u64, next_exp: u64) {
        self.exp = exp;
        self.next_exp = if next_exp == 0 { 1 } else { next_exp };
    }

    /// Returns the fill percentages (0.0 to 1.0) for each of the 4 EXP orbs.
    pub fn get_orbs_percentages(&self) -> [f32; 4] {
        let mut percentages = [0.0; 4];
        let ratio = if self.next_exp == 0 {
            0.0
        } else {
            self.exp as f64 / self.next_exp as f64
        }.clamp(0.0, 1.0);

        let total_orbs = ratio * 4.0;
        for i in 0..4 {
            let bulb_val = total_orbs - i as f64;
            percentages[i] = bulb_val.clamp(0.0, 1.0) as f32;
        }

        percentages
    }
}

pub struct GaugeSystem {
    pub hp_gauge: GaugeState,
    pub sp_gauge: GaugeState,
    pub st_gauge: GaugeState,
    pub exp_gauge: ExpTaskbarGauge,
}

impl GaugeSystem {
    pub fn new() -> Self {
        Self {
            hp_gauge: GaugeState::new(100.0, 15.0),
            sp_gauge: GaugeState::new(100.0, 15.0),
            st_gauge: GaugeState::new(100.0, 15.0),
            exp_gauge: ExpTaskbarGauge::new(),
        }
    }
}

static GAUGE_SYSTEM: OnceLock<Mutex<GaugeSystem>> = OnceLock::new();

pub fn get_gauge_system() -> &'static Mutex<GaugeSystem> {
    GAUGE_SYSTEM.get_or_init(|| Mutex::new(GaugeSystem::new()))
}

// ==========================================
// C-FFI Bindings
// ==========================================

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_set_hp(current: f64, max: f64) {
    if let Ok(mut sys) = get_gauge_system().lock() {
        sys.hp_gauge.set_value(current, max);
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_update(dt: f64) {
    if let Ok(mut sys) = get_gauge_system().lock() {
        sys.hp_gauge.update(dt);
        sys.sp_gauge.update(dt);
        sys.st_gauge.update(dt);
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_get_hp_clip_rect(out_rect: *mut RectClip) {
    if out_rect.is_null() { return; }
    if let Ok(sys) = get_gauge_system().lock() {
        unsafe {
            *out_rect = sys.hp_gauge.get_clip_rect();
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_is_hp_flashing() -> bool {
    if let Ok(sys) = get_gauge_system().lock() {
        sys.hp_gauge.is_flashing
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_get_hp_flash_alpha() -> f32 {
    if let Ok(sys) = get_gauge_system().lock() {
        sys.hp_gauge.flash_alpha
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_set_exp(exp: u64, next_exp: u64) {
    if let Ok(mut sys) = get_gauge_system().lock() {
        sys.exp_gauge.set_exp(exp, next_exp);
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_v2_get_exp_orbs(out_orbs: *mut f32) {
    if out_orbs.is_null() { return; }
    if let Ok(sys) = get_gauge_system().lock() {
        let orbs = sys.exp_gauge.get_orbs_percentages();
        unsafe {
            for i in 0..4 {
                *out_orbs.add(i) = orbs[i];
            }
        }
    }
}
