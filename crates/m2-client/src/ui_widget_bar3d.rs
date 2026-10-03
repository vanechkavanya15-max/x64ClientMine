use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3dxColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl D3dxColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bar3D {
    pub left_color: D3dxColor,
    pub right_color: D3dxColor,
    pub center_color: D3dxColor,
    pub rect: Rect,
}

impl Bar3D {
    pub fn new() -> Self {
        Self {
            left_color: D3dxColor::new(0.2, 0.2, 0.2, 1.0),
            right_color: D3dxColor::new(0.7, 0.7, 0.7, 1.0),
            center_color: D3dxColor::new(0.0, 0.0, 0.0, 1.0),
            rect: Rect { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 },
        }
    }

    pub fn set_color(&mut self, left: D3dxColor, right: D3dxColor, center: D3dxColor) {
        self.left_color = left;
        self.right_color = right;
        self.center_color = center;
    }

    pub fn set_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.rect = Rect { left, top, right, bottom };
    }

    /// Symulacja OnRender z C++
    pub fn on_render(&self) -> Vec<RenderCommand> {
        let mut commands = Vec::new();

        // rkpyGraphic.SetDiffuseColor(m_dwCenterColor);
        // rkpyGraphic.RenderBar2d(m_rect.left, m_rect.top, m_rect.right, m_rect.bottom);
        commands.push(RenderCommand::SetColor(self.center_color));
        commands.push(RenderCommand::RenderBar2d {
            left: self.rect.left,
            top: self.rect.top,
            right: self.rect.right,
            bottom: self.rect.bottom,
        });

        // rkpyGraphic.SetDiffuseColor(m_dwLeftColor);
        // rkpyGraphic.RenderLine2d(m_rect.left, m_rect.top, m_rect.right, m_rect.top);
        // rkpyGraphic.RenderLine2d(m_rect.left, m_rect.top, m_rect.left, m_rect.bottom);
        commands.push(RenderCommand::SetColor(self.left_color));
        commands.push(RenderCommand::RenderLine2d {
            x1: self.rect.left, y1: self.rect.top, x2: self.rect.right, y2: self.rect.top,
        });
        commands.push(RenderCommand::RenderLine2d {
            x1: self.rect.left, y1: self.rect.top, x2: self.rect.left, y2: self.rect.bottom,
        });

        // rkpyGraphic.SetDiffuseColor(m_dwRightColor);
        // rkpyGraphic.RenderLine2d(m_rect.left, m_rect.bottom, m_rect.right, m_rect.bottom);
        // rkpyGraphic.RenderLine2d(m_rect.right, m_rect.top, m_rect.right, m_rect.bottom);
        commands.push(RenderCommand::SetColor(self.right_color));
        commands.push(RenderCommand::RenderLine2d {
            x1: self.rect.left, y1: self.rect.bottom, x2: self.rect.right, y2: self.rect.bottom,
        });
        commands.push(RenderCommand::RenderLine2d {
            x1: self.rect.right, y1: self.rect.top, x2: self.rect.right, y2: self.rect.bottom,
        });

        commands
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderCommand {
    SetColor(D3dxColor),
    RenderBar2d { left: f32, top: f32, right: f32, bottom: f32 },
    RenderLine2d { x1: f32, y1: f32, x2: f32, y2: f32 },
}

/// Transformacja 3D World do 2D Screen
pub fn project_world_to_screen(
    world_x: f32, world_y: f32, world_z: f32,
    view_proj_matrix: &[f32; 16],
    screen_width: f32, screen_height: f32,
) -> Option<(f32, f32)> {
    let vp = view_proj_matrix;
    
    // clip_space = ViewProjection * (world_pos, 1.0)
    let clip_x = world_x * vp[0] + world_y * vp[4] + world_z * vp[8] + 1.0 * vp[12];
    let clip_y = world_x * vp[1] + world_y * vp[5] + world_z * vp[9] + 1.0 * vp[13];
    let clip_z = world_x * vp[2] + world_y * vp[6] + world_z * vp[10] + 1.0 * vp[14];
    let clip_w = world_x * vp[3] + world_y * vp[7] + world_z * vp[11] + 1.0 * vp[15];

    if clip_w <= 0.0 {
        return None; // Behind the camera
    }

    let ndc_x = clip_x / clip_w;
    let ndc_y = clip_y / clip_w;
    let _ndc_z = clip_z / clip_w;

    let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
    let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;

    Some((screen_x, screen_y))
}


// FFI Export - Mock Registration
#[no_mangle]
pub extern "C" fn wndMgr_RegisterBar3D_Mock(py_obj: usize) -> u32 {
    // Orkiestrator podepnie pod: get_window_manager().register_with_handler(WindowType::Bar3D, py_obj)
    // Zwracamy przykladowy uchwyt (dummy) na potrzeby specyfikacji samodzielnego modulu
    let dummy_handle = 999;
    dummy_handle
}

#[no_mangle]
pub extern "C" fn wndMgr_SetColorBar3D_Mock(
    _handle: u32,
    lr: f32, lg: f32, lb: f32, la: f32,
    rr: f32, rg: f32, rb: f32, ra: f32,
    cr: f32, cg: f32, cb: f32, ca: f32,
) -> bool {
    let mut bar = Bar3D::new();
    bar.set_color(
        D3dxColor::new(lr, lg, lb, la),
        D3dxColor::new(rr, rg, rb, ra),
        D3dxColor::new(cr, cg, cb, ca),
    );
    true
}
