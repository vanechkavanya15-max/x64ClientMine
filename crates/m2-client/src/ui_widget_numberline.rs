use crate::ui_texture_mgr::{UiImageResource, UiTextureManager};
use crate::wgpu_engine_context::{Quad2D, GpuTexture};
use crate::ui_window_core::{HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_RIGHT};
use std::sync::Arc;

pub struct NumberLineWidget {
    pub path: String,
    pub number: String,
    pub horizontal_align: u8,
    pub width_summary: u32,
    pub image_instances: Vec<UiImageResource>,
    pub positions: Vec<(i32, i32)>,
}

impl Default for NumberLineWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberLineWidget {
    pub fn new() -> Self {
        Self {
            path: "d:/ymir work/ui/game/taskbar/".to_string(),
            number: String::new(),
            horizontal_align: HORIZONTAL_ALIGN_LEFT,
            width_summary: 0,
            image_instances: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    pub fn set_horizontal_align(&mut self, align: u8) {
        self.horizontal_align = align;
    }

    pub fn clear_number(&mut self) {
        self.image_instances.clear();
        self.positions.clear();
        self.width_summary = 0;
        self.number.clear();
    }

    pub fn set_number(&mut self, number: &str, texture_mgr: &mut UiTextureManager) {
        if self.number == number {
            return;
        }

        self.clear_number();
        self.number = number.to_string();

        for c in number.chars() {
            let file_name_suffix = match c {
                ':' => Some("colon.sub"),
                '?' => Some("questionmark.sub"),
                '/' => Some("slash.sub"),
                '%' => Some("percent.sub"),
                '+' => Some("plus.sub"),
                'm' => Some("m.sub"),
                'g' => Some("g.sub"),
                'p' => Some("p.sub"),
                '0'..='9' => {
                    None
                }
                _ => continue,
            };

            let str_image_file_name = if let Some(suffix) = file_name_suffix {
                format!("{}{}", self.path, suffix)
            } else if c.is_ascii_digit() {
                format!("{}{}.sub", self.path, c)
            } else {
                continue;
            };

            if let Some(res) = texture_mgr.get_image_resource(&str_image_file_name) {
                self.width_summary += res.width;
                self.image_instances.push(res);
            }
        }
    }

    pub fn on_change_position(&mut self, base_x: i32, base_y: i32, _has_parent: bool) {
        let mut ix = base_x;
        let iy = base_y;

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

        self.positions.clear();
        for instance in &self.image_instances {
            self.positions.push((ix, iy));
            ix += instance.width as i32;
        }
    }

    pub fn on_render(&self, batch: &mut Vec<(Arc<GpuTexture>, Vec<Quad2D>)>) {
        for (i, instance) in self.image_instances.iter().enumerate() {
            if let Some(&(x, y)) = self.positions.get(i) {
                let quad = Quad2D::with_uv(
                    x as f32,
                    y as f32,
                    instance.width as f32,
                    instance.height as f32,
                    instance.uv[0],
                    instance.uv[1],
                    instance.uv[2],
                    instance.uv[3],
                );
                
                let mut found = false;
                for (tex, quads) in batch.iter_mut() {
                    if Arc::ptr_eq(tex, &instance.texture) {
                        quads.push(quad.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    batch.push((instance.texture.clone(), vec![quad]));
                }
            }
        }
    }
}
