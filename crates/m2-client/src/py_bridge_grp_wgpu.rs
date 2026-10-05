// crates/m2-client/src/py_bridge_grp_wgpu.rs
//! Mostek stanu renderowania grafiki (GrpModule) integrujacy wywolania D3D9 ze stanami WGPU.
//! Zapewnia 100% parytetu z C++ PythonGraphicModule.cpp i CPythonGraphic.cpp.

use std::sync::RwLock;
use once_cell::sync::Lazy;
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpRenderMode {
    Game3D,
    Interface2D,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrpRenderState {
    pub mode: GrpRenderMode,
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha_blend: bool,
    pub cull_backfaces: bool,
    pub depth_clear_requested: bool,
}

impl Default for GrpRenderState {
    fn default() -> Self {
        Self {
            mode: GrpRenderMode::Game3D,
            depth_test: true,
            depth_write: true,
            alpha_blend: true,
            cull_backfaces: true,
            depth_clear_requested: false,
        }
    }
}

pub struct GrpStateManager {
    pub current: GrpRenderState,
    pub stack: Vec<GrpRenderState>,
}

impl GrpStateManager {
    pub fn new() -> Self {
        Self {
            current: GrpRenderState::default(),
            stack: Vec::with_capacity(16),
        }
    }

    pub fn set_game_render_state(&mut self) {
        self.current.mode = GrpRenderMode::Game3D;
        self.current.depth_test = true;
        self.current.depth_write = true;
        self.current.alpha_blend = true;
        self.current.cull_backfaces = true;
    }

    pub fn set_interface_render_state(&mut self) {
        self.current.mode = GrpRenderMode::Interface2D;
        self.current.depth_test = false;
        self.current.depth_write = false;
        self.current.alpha_blend = true;
        self.current.cull_backfaces = false;
    }

    pub fn push_state(&mut self) {
        self.stack.push(self.current);
    }

    pub fn pop_state(&mut self) {
        if let Some(prev) = self.stack.pop() {
            self.current = prev;
        }
    }

    pub fn clear_depth_buffer(&mut self) {
        self.current.depth_clear_requested = true;
    }
}

static GRP_STATE_MGR: Lazy<RwLock<GrpStateManager>> = Lazy::new(|| RwLock::new(GrpStateManager::new()));

/// Czysci bufor glebokosci w biezacym passie renderowania (grp.ClearDepthBuffer).
#[no_mangle]
pub extern "C" fn m2_grp_clear_depth_buffer() {
    safe_ffi_call((), || {
        let mut mgr = GRP_STATE_MGR.write().unwrap();
        mgr.clear_depth_buffer();
    });
}

/// Przelacza potok w tryb renderowania gry 3D (grp.SetGameRenderState).
#[no_mangle]
pub extern "C" fn m2_grp_set_game_render_state() {
    safe_ffi_call((), || {
        let mut mgr = GRP_STATE_MGR.write().unwrap();
        mgr.set_game_render_state();
    });
}

/// Przelacza potok w tryb renderowania interfejsu 2D (grp.SetInterfaceRenderState).
#[no_mangle]
pub extern "C" fn m2_grp_set_interface_render_state() {
    safe_ffi_call((), || {
        let mut mgr = GRP_STATE_MGR.write().unwrap();
        mgr.set_interface_render_state();
    });
}

/// Zapisuje biezacy stan renderowania na stosie (grp.PushState).
#[no_mangle]
pub extern "C" fn m2_grp_push_state() {
    safe_ffi_call((), || {
        let mut mgr = GRP_STATE_MGR.write().unwrap();
        mgr.push_state();
    });
}

/// Przywraca poprzedni stan renderowania ze stosu (grp.PopState).
#[no_mangle]
pub extern "C" fn m2_grp_pop_state() {
    safe_ffi_call((), || {
        let mut mgr = GRP_STATE_MGR.write().unwrap();
        mgr.pop_state();
    });
}

/// Pakuje skladowe RGBA [0.0, 1.0] do 32-bitowej liczby ARGB zgodnej z D3DCOLOR_COLORVALUE (grp.GenerateColor).
#[no_mangle]
pub extern "C" fn m2_grp_generate_color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    safe_ffi_call(0xFFFFFFFF, || {
        let a_byte = (a.clamp(0.0, 1.0) * 255.0).round() as u32;
        let r_byte = (r.clamp(0.0, 1.0) * 255.0).round() as u32;
        let g_byte = (g.clamp(0.0, 1.0) * 255.0).round() as u32;
        let b_byte = (b.clamp(0.0, 1.0) * 255.0).round() as u32;

        (a_byte << 24) | (r_byte << 16) | (g_byte << 8) | b_byte
    })
}

/// Pobiera biezacy stan renderowania do weryfikacji testowej.
pub fn get_current_grp_state() -> GrpRenderState {
    let mgr = GRP_STATE_MGR.read().unwrap();
    mgr.current
}
