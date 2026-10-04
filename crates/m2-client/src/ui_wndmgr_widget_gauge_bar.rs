//! Gauge progress bar widget (HP, SP, ST, EXP, Target HP) with smooth animation and C-FFI bindings.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GaugeType {
    Hp = 0,
    Sp = 1,
    Stamina = 2,
    Exp = 3,
    TargetHp = 4,
    Custom = 5,
}

impl From<u8> for GaugeType {
    fn from(val: u8) -> Self {
        match val {
            0 => GaugeType::Hp,
            1 => GaugeType::Sp,
            2 => GaugeType::Stamina,
            3 => GaugeType::Exp,
            4 => GaugeType::TargetHp,
            _ => GaugeType::Custom,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GaugeBarWidget {
    pub handle: u32,
    pub gauge_type: GaugeType,
    pub current_value: f32,
    pub display_value: f32,
    pub max_value: f32,
    pub anim_speed: f32,
    pub color_rgba: [f32; 4],
    pub bg_color_rgba: [f32; 4],
    pub visible: bool,
    pub width: f32,
    pub height: f32,
}

impl GaugeBarWidget {
    pub fn new(handle: u32, gauge_type: GaugeType) -> Self {
        let (color, bg) = match gauge_type {
            GaugeType::Hp => ([0.85, 0.15, 0.15, 1.0], [0.25, 0.05, 0.05, 0.8]),
            GaugeType::Sp => ([0.15, 0.45, 0.90, 1.0], [0.05, 0.15, 0.30, 0.8]),
            GaugeType::Stamina => ([0.95, 0.80, 0.15, 1.0], [0.30, 0.25, 0.05, 0.8]),
            GaugeType::Exp => ([0.15, 0.85, 0.25, 1.0], [0.05, 0.25, 0.05, 0.8]),
            GaugeType::TargetHp => ([0.90, 0.20, 0.20, 1.0], [0.20, 0.05, 0.05, 0.8]),
            GaugeType::Custom => ([0.70, 0.70, 0.70, 1.0], [0.20, 0.20, 0.20, 0.8]),
        };

        Self {
            handle,
            gauge_type,
            current_value: 100.0,
            display_value: 100.0,
            max_value: 100.0,
            anim_speed: 8.0,
            color_rgba: color,
            bg_color_rgba: bg,
            visible: true,
            width: 100.0,
            height: 12.0,
        }
    }

    pub fn set_value(&mut self, current: f32, max: f32) {
        self.max_value = if max <= 0.0 { 1.0 } else { max };
        self.current_value = current.clamp(0.0, self.max_value);
    }

    pub fn set_value_instant(&mut self, current: f32, max: f32) {
        self.set_value(current, max);
        self.display_value = self.current_value;
    }

    pub fn update(&mut self, dt: f32) {
        let diff = self.current_value - self.display_value;
        if diff.abs() > 0.001 {
            let step = diff * (self.anim_speed * dt).clamp(0.0, 1.0);
            self.display_value += step;
        } else {
            self.display_value = self.current_value;
        }
    }

    pub fn percentage(&self) -> f32 {
        if self.max_value <= 0.0 {
            0.0
        } else {
            (self.display_value / self.max_value).clamp(0.0, 1.0)
        }
    }

    pub fn get_fill_rect(&self, x: f32, y: f32) -> [f32; 4] {
        let fill_w = self.width * self.percentage();
        [x, y, fill_w, self.height]
    }
}

pub struct GaugeManager {
    widgets: HashMap<u32, GaugeBarWidget>,
    next_handle: u32,
}

impl Default for GaugeManager {
    fn default() -> Self {
        Self {
            widgets: HashMap::new(),
            next_handle: 1000,
        }
    }
}

static GAUGE_MGR: OnceLock<Mutex<GaugeManager>> = OnceLock::new();

pub fn get_gauge_mgr() -> &'static Mutex<GaugeManager> {
    GAUGE_MGR.get_or_init(|| Mutex::new(GaugeManager::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_ui_gauge_create(gauge_type: u8) -> u32 {
    safe_ffi_call(0, || {
        let mut mgr = get_gauge_mgr().lock().unwrap();
        let handle = mgr.next_handle;
        mgr.next_handle += 1;
        let widget = GaugeBarWidget::new(handle, GaugeType::from(gauge_type));
        mgr.widgets.insert(handle, widget);
        handle
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_set_value(handle: u32, current: f32, max: f32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_gauge_mgr().lock().unwrap();
        if let Some(widget) = mgr.widgets.get_mut(&handle) {
            widget.set_value(current, max);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_get_percentage(handle: u32) -> f32 {
    safe_ffi_call(0.0, || {
        let mgr = get_gauge_mgr().lock().unwrap();
        if let Some(widget) = mgr.widgets.get(&handle) {
            widget.percentage()
        } else {
            0.0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_update(handle: u32, dt: f32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_gauge_mgr().lock().unwrap();
        if let Some(widget) = mgr.widgets.get_mut(&handle) {
            widget.update(dt);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_gauge_destroy(handle: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_gauge_mgr().lock().unwrap();
        mgr.widgets.remove(&handle).is_some()
    })
}
