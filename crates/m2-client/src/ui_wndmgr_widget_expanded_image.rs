// ui_wndmgr_widget_expanded_image.rs
// Implementation of ExpandedImageBox widget (equivalent to CGraphicExpandedImageInstance)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingMode {
    Normal = 0,
    Screen = 1,
    ColorDodge = 2,
    Modulate = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TPDTVertex {
    pub position: [f32; 3],
    pub diffuse: u32,
    pub tex_coord: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct ExpandedImageBox {
    pub position: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub origin: [f32; 2],
    pub scale: [f32; 2],
    pub rotation: f32,
    pub rendering_rect: [f32; 4], // left, top, right, bottom
    pub rendering_mode: RenderingMode,
    pub diffuse_color: u32,
    pub image_rect: [f32; 4], // left, top, right, bottom
    pub texture_width: f32,
    pub texture_height: f32,
}

impl Default for ExpandedImageBox {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpandedImageBox {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            origin: [0.0, 0.0],
            scale: [1.0, 1.0],
            rotation: 0.0,
            rendering_rect: [0.0, 0.0, 0.0, 0.0],
            rendering_mode: RenderingMode::Normal,
            diffuse_color: 0xFFFFFFFF, // White default
            image_rect: [0.0, 0.0, 0.0, 0.0],
            texture_width: 1.0,
            texture_height: 1.0,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth;
    }

    pub fn set_origin_default(&mut self) {
        self.origin = [self.width / 2.0, self.height / 2.0];
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.origin = [x, y];
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = [x, y];
    }

    pub fn set_rendering_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        if self.width == 0.0 || self.height == 0.0 {
            return;
        }

        self.rendering_rect[0] = self.width * left;
        self.rendering_rect[1] = self.height * top;
        self.rendering_rect[2] = self.width * right;
        self.rendering_rect[3] = self.height * bottom;
    }

    pub fn set_rendering_mode(&mut self, mode: RenderingMode) {
        self.rendering_mode = mode;
    }

    pub fn set_diffuse_color(&mut self, color: u32) {
        self.diffuse_color = color;
    }

    pub fn set_image_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.image_rect = [left, top, right, bottom];
    }

    pub fn set_texture_size(&mut self, width: f32, height: f32) {
        if width > 0.0 {
            self.texture_width = width;
        }
        if height > 0.0 {
            self.texture_height = height;
        }
    }

    pub fn generate_vertices(&self) -> [TPDTVertex; 4] {
        let tex_reverse_width = 1.0 / self.texture_width;
        let tex_reverse_height = 1.0 / self.texture_height;
        
        let c_r_rect_left = self.image_rect[0];
        let c_r_rect_top = self.image_rect[1];
        let c_r_rect_right = self.image_rect[2];
        let c_r_rect_bottom = self.image_rect[3];

        let m_rendering_rect_left = self.rendering_rect[0];
        let m_rendering_rect_top = self.rendering_rect[1];
        let m_rendering_rect_right = self.rendering_rect[2];
        let m_rendering_rect_bottom = self.rendering_rect[3];

        let su = (c_r_rect_left + m_rendering_rect_left) * tex_reverse_width;
        let sv = (c_r_rect_top + m_rendering_rect_top) * tex_reverse_height;
        let eu = (c_r_rect_right + m_rendering_rect_right) * tex_reverse_width;
        let ev = (c_r_rect_bottom + m_rendering_rect_bottom) * tex_reverse_height;

        let mut vertices = [
            TPDTVertex {
                position: [self.position[0] - 0.5, self.position[1] - 0.5, self.depth],
                diffuse: self.diffuse_color,
                tex_coord: [su, sv],
            },
            TPDTVertex {
                position: [self.position[0] - 0.5, self.position[1] - 0.5, self.depth],
                diffuse: self.diffuse_color,
                tex_coord: [eu, sv],
            },
            TPDTVertex {
                position: [self.position[0] - 0.5, self.position[1] - 0.5, self.depth],
                diffuse: self.diffuse_color,
                tex_coord: [su, ev],
            },
            TPDTVertex {
                position: [self.position[0] - 0.5, self.position[1] - 0.5, self.depth],
                diffuse: self.diffuse_color,
                tex_coord: [eu, ev],
            },
        ];

        let img_width = self.width * self.scale[0];
        let img_height = self.height * self.scale[1];

        if self.rotation == 0.0 {
            vertices[0].position[0] -= m_rendering_rect_left;
            vertices[0].position[1] -= m_rendering_rect_top;
            
            vertices[1].position[0] += img_width + m_rendering_rect_right;
            vertices[1].position[1] -= m_rendering_rect_top;
            
            vertices[2].position[0] -= m_rendering_rect_left;
            vertices[2].position[1] += img_height + m_rendering_rect_bottom;
            
            vertices[3].position[0] += img_width + m_rendering_rect_right;
            vertices[3].position[1] += img_height + m_rendering_rect_bottom;
        } else {
            let left = m_rendering_rect_left;
            let right = img_width + m_rendering_rect_right;
            let top = m_rendering_rect_top;
            let bottom = img_height + m_rendering_rect_bottom;

            let pts = [
                (left, top),
                (right, top),
                (left, bottom),
                (right, bottom),
            ];

            let f_radian = self.rotation.to_radians();
            let cos_r = f_radian.cos();
            let sin_r = f_radian.sin();

            for (i, &(px, py)) in pts.iter().enumerate() {
                let ox = px - self.origin[0];
                let oy = py - self.origin[1];
                let rx = ox * cos_r - oy * sin_r;
                let ry = ox * sin_r + oy * cos_r;
                vertices[i].position[0] += self.origin[0] + rx;
                vertices[i].position[1] += self.origin[1] + ry;
            }
        }

        vertices
    }
}
