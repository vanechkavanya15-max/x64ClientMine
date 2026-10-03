use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, OnceLock};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderPass {
    ShadowMap = 0,
    SkyBox = 1,
    TerrainSplatting = 2,
    Water = 3,
    Characters = 4,
    MagicEffects = 5,
    Ui2D = 6,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderCompositionCallbacks {
    pub pass_shadow_map: Option<extern "C" fn()>,
    pub pass_sky_box: Option<extern "C" fn()>,
    pub pass_terrain_splatting: Option<extern "C" fn()>,
    pub pass_water: Option<extern "C" fn()>,
    pub pass_characters: Option<extern "C" fn()>,
    pub pass_magic_effects: Option<extern "C" fn()>,
    pub pass_ui_2d: Option<extern "C" fn()>,
}

impl Default for RenderCompositionCallbacks {
    fn default() -> Self {
        Self {
            pass_shadow_map: None,
            pass_sky_box: None,
            pass_terrain_splatting: None,
            pass_water: None,
            pass_characters: None,
            pass_magic_effects: None,
            pass_ui_2d: None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderCompositionMetrics {
    pub frames_rendered: u64,
    pub last_pass_completed: u32,
    pub is_rendering: bool,
}

impl Default for RenderCompositionMetrics {
    fn default() -> Self {
        Self {
            frames_rendered: 0,
            last_pass_completed: u32::MAX,
            is_rendering: false,
        }
    }
}

struct RenderCompositionState {
    callbacks: RenderCompositionCallbacks,
    metrics: RenderCompositionMetrics,
}

static STATE: OnceLock<Mutex<RenderCompositionState>> = OnceLock::new();

fn get_state() -> std::sync::MutexGuard<'static, RenderCompositionState> {
    STATE
        .get_or_init(|| {
            Mutex::new(RenderCompositionState {
                callbacks: RenderCompositionCallbacks::default(),
                metrics: RenderCompositionMetrics::default(),
            })
        })
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_render_composition_register_callbacks(
    callbacks: *const RenderCompositionCallbacks,
) -> bool {
    safe_ffi_call(false, || {
        if callbacks.is_null() {
            return false;
        }

        let mut state = get_state();
        state.callbacks = unsafe { std::ptr::read(callbacks) };
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_render_composition_execute_frame() -> bool {
    safe_ffi_call(false, || {
        let callbacks = {
            let mut state = get_state();
            if state.metrics.is_rendering {
                return false;
            }
            state.metrics.is_rendering = true;
            state.callbacks
        };

        if let Some(cb) = callbacks.pass_shadow_map {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::ShadowMap as u32;

        if let Some(cb) = callbacks.pass_sky_box {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::SkyBox as u32;

        if let Some(cb) = callbacks.pass_terrain_splatting {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::TerrainSplatting as u32;

        if let Some(cb) = callbacks.pass_water {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::Water as u32;

        if let Some(cb) = callbacks.pass_characters {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::Characters as u32;

        if let Some(cb) = callbacks.pass_magic_effects {
            cb();
        }
        get_state().metrics.last_pass_completed = RenderPass::MagicEffects as u32;

        if let Some(cb) = callbacks.pass_ui_2d {
            cb();
        }
        
        let mut final_state = get_state();
        final_state.metrics.last_pass_completed = RenderPass::Ui2D as u32;
        final_state.metrics.frames_rendered = final_state.metrics.frames_rendered.saturating_add(1);
        final_state.metrics.is_rendering = false;

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_render_composition_get_metrics(out_metrics: *mut RenderCompositionMetrics) -> bool {
    safe_ffi_call(false, || {
        if out_metrics.is_null() {
            return false;
        }

        let state = get_state();
        unsafe {
            std::ptr::write(out_metrics, state.metrics);
        }
        true
    })
}

#[cfg(test)]
pub fn reset_state_for_test() {
    let mut state = get_state();
    state.callbacks = RenderCompositionCallbacks::default();
    state.metrics = RenderCompositionMetrics::default();
}
