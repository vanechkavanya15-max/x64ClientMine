#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3DViewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub min_z: f32,
    pub max_z: f32,
}

impl Default for D3DViewport {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            min_z: 0.0,
            max_z: 1.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderTargetDesc {
    pub width: u32,
    pub height: u32,
    pub format: u32,
}

pub const D3DCLEAR_TARGET: u32 = 0x00000001;
pub const D3DCLEAR_ZBUFFER: u32 = 0x00000002;
pub const D3DCLEAR_STENCIL: u32 = 0x00000004;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderTargetError {
    InvalidViewportDimensions,
    InvalidClearFlags,
}

pub struct RenderTargetManager {
    viewport: D3DViewport,
    active_render_target: Option<RenderTargetDesc>,
}

impl RenderTargetManager {
    pub fn new() -> Self {
        Self {
            viewport: D3DViewport::default(),
            active_render_target: None,
        }
    }

    pub fn set_viewport(&mut self, vp: &D3DViewport) -> Result<(), RenderTargetError> {
        if vp.width == 0 || vp.height == 0 {
            return Err(RenderTargetError::InvalidViewportDimensions);
        }
        self.viewport = *vp;
        Ok(())
    }

    pub fn get_viewport(&self) -> D3DViewport {
        self.viewport
    }

    pub fn clear_render_target(
        &self,
        clear_flags: u32,
        _color_argb: u32,
        _depth: f32,
        _stencil: u32,
    ) -> Result<(), RenderTargetError> {
        let valid_flags = D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER | D3DCLEAR_STENCIL;
        if (clear_flags & !valid_flags) != 0 {
            return Err(RenderTargetError::InvalidClearFlags);
        }

        // Logic to interface with graphics API would go here.
        // For pure Rust logic validation, we just verify the flags.
        
        Ok(())
    }

    pub fn calculate_letterbox_viewport(
        &self,
        screen_w: u32,
        screen_h: u32,
        target_ratio: f32,
    ) -> Result<D3DViewport, RenderTargetError> {
        if screen_w == 0 || screen_h == 0 || target_ratio <= 0.0 {
            return Err(RenderTargetError::InvalidViewportDimensions);
        }

        let screen_ratio = screen_w as f32 / screen_h as f32;
        let mut vp = D3DViewport {
            min_z: 0.0,
            max_z: 1.0,
            ..Default::default()
        };

        if screen_ratio > target_ratio {
            // Screen is wider than target (Pillarbox)
            let new_w = (screen_h as f32 * target_ratio).round() as u32;
            vp.width = new_w;
            vp.height = screen_h;
            vp.x = (screen_w - new_w) / 2;
            vp.y = 0;
        } else {
            // Screen is taller than target or matches (Letterbox)
            let new_h = (screen_w as f32 / target_ratio).round() as u32;
            vp.width = screen_w;
            vp.height = new_h;
            vp.x = 0;
            vp.y = (screen_h - new_h) / 2;
        }

        Ok(vp)
    }
}
