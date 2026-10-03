use crate::win32_window::{WindowHandle, set_window_fullscreen};
use crate::wgpu_engine_context::WgpuEngineContext;

pub struct DisplayModeSwitcher {
    pub hwnd: WindowHandle,
    pub is_fullscreen: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen_width: u32,
    pub fullscreen_height: u32,
}

impl DisplayModeSwitcher {
    pub fn new(
        hwnd: WindowHandle,
        window_width: u32,
        window_height: u32,
        fullscreen_width: u32,
        fullscreen_height: u32,
    ) -> Self {
        Self {
            hwnd,
            is_fullscreen: false,
            window_width,
            window_height,
            fullscreen_width,
            fullscreen_height,
        }
    }

    pub fn set_fullscreen(&mut self, wgpu_ctx: &mut WgpuEngineContext, fullscreen: bool) {
        if self.is_fullscreen == fullscreen {
            return;
        }

        self.is_fullscreen = fullscreen;
        set_window_fullscreen(self.hwnd, fullscreen);

        if fullscreen {
            wgpu_ctx.resize(self.fullscreen_width, self.fullscreen_height);
        } else {
            wgpu_ctx.resize(self.window_width, self.window_height);
        }
    }

    pub fn toggle_fullscreen(&mut self, wgpu_ctx: &mut WgpuEngineContext) {
        self.set_fullscreen(wgpu_ctx, !self.is_fullscreen);
    }

    pub fn update_resolutions(&mut self, window_w: u32, window_h: u32, full_w: u32, full_h: u32) {
        self.window_width = window_w;
        self.window_height = window_h;
        self.fullscreen_width = full_w;
        self.fullscreen_height = full_h;
    }
}
