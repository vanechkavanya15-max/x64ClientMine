use crate::wgpu_engine_context::Quad2D;
use crate::ui_window_core::{HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_RIGHT};

#[derive(Clone, Default)]
pub struct NumberAtlasLayout {
    pub positions: Vec<(i32, i32)>,
    pub uvs: Vec<[f32; 4]>,
}

pub struct NumberLineAtlasWidget {
    pub number_str: String,
    pub horizontal_align: u8,
    pub width_summary: u32,
    pub char_width: u32,
    pub char_height: u32,
    pub spacing: i32,
    pub atlas_uvs: [f32; 4],
    pub layout: NumberAtlasLayout,
    pub base_x: i32,
    pub base_y: i32,
}

impl Default for NumberLineAtlasWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberLineAtlasWidget {
    pub fn new() -> Self {
        Self {
            number_str: String::new(),
            horizontal_align: HORIZONTAL_ALIGN_LEFT,
            width_summary: 0,
            char_width: 0,
            char_height: 0,
            spacing: 0,
            atlas_uvs: [0.0, 0.0, 1.0, 1.0],
            layout: NumberAtlasLayout::default(),
            base_x: 0,
            base_y: 0,
        }
    }

    pub fn set_atlas_info(&mut self, char_width: u32, char_height: u32, atlas_uvs: [f32; 4]) {
        self.char_width = char_width;
        self.char_height = char_height;
        self.atlas_uvs = atlas_uvs;
        self.update_layout();
    }

    pub fn set_horizontal_align(&mut self, align: u8) {
        self.horizontal_align = align;
        self.update_layout();
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = spacing;
        self.update_layout();
    }

    pub fn set_number(&mut self, number: &str) {
        if self.number_str == number {
            return;
        }
        self.number_str = number.to_string();
        self.update_layout();
    }

    pub fn on_change_position(&mut self, base_x: i32, base_y: i32) {
        self.base_x = base_x;
        self.base_y = base_y;
        self.update_layout();
    }

    fn update_layout(&mut self) {
        self.layout.positions.clear();
        self.layout.uvs.clear();
        self.width_summary = 0;

        if self.char_width == 0 || self.number_str.is_empty() {
            return;
        }

        let mut valid_chars = 0;
        for c in self.number_str.chars() {
            if c.is_ascii_digit() {
                valid_chars += 1;
            }
        }

        if valid_chars == 0 {
            return;
        }

        // Calculate total width based on characters and spacing
        let total_spacing = if valid_chars > 0 { (valid_chars - 1) as i32 * self.spacing } else { 0 };
        self.width_summary = (valid_chars * self.char_width) as u32;
        if total_spacing > 0 {
            self.width_summary += total_spacing as u32;
        } else if total_spacing < 0 && self.width_summary as i32 + total_spacing > 0 {
            self.width_summary = (self.width_summary as i32 + total_spacing) as u32;
        }

        let u_width = self.atlas_uvs[2] - self.atlas_uvs[0];
        let v0 = self.atlas_uvs[1];
        let v1 = self.atlas_uvs[3];
        // digit width ratio assuming 10 digits
        let char_u_w = u_width / 10.0; 

        for c in self.number_str.chars() {
            if c.is_ascii_digit() {
                let digit_idx = c.to_digit(10).unwrap() as f32;
                let u0 = self.atlas_uvs[0] + digit_idx * char_u_w;
                let u1 = u0 + char_u_w;
                self.layout.uvs.push([u0, v0, u1, v1]);
            }
        }

        let mut ix = self.base_x;
        let iy = self.base_y;

        match self.horizontal_align {
            HORIZONTAL_ALIGN_LEFT => {}
            HORIZONTAL_ALIGN_CENTER => {
                ix -= (self.width_summary / 2) as i32;
            }
            HORIZONTAL_ALIGN_RIGHT => {
                ix -= self.width_summary as i32;
            }
            _ => {}
        }

        for _ in 0..self.layout.uvs.len() {
            self.layout.positions.push((ix, iy));
            ix += self.char_width as i32 + self.spacing;
        }
    }

    pub fn get_quads(&self) -> Vec<Quad2D> {
        let mut quads = Vec::new();
        for (i, &(x, y)) in self.layout.positions.iter().enumerate() {
            if let Some(uv) = self.layout.uvs.get(i) {
                quads.push(Quad2D::with_uv(
                    x as f32,
                    y as f32,
                    self.char_width as f32,
                    self.char_height as f32,
                    uv[0],
                    uv[1],
                    uv[2],
                    uv[3],
                ));
            }
        }
        quads
    }
}
