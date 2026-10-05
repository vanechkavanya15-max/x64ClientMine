use glam::{Mat4, Vec3, Vec4};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub const fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub const fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextTailData {
    pub id: u32,
    pub position: Vec3,
    pub height_offset: f32,
    pub name: String,
    pub guild_name: Option<String>,
    pub kingdom_color: Option<Color>, // Kingdom color for guild name
    pub level: Option<u32>,
    pub hp_percent: Option<u8>,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderElement {
    Text {
        x: f32,
        y: f32,
        z: f32,
        text: String,
        color: Color,
        centered: bool,
    },
    Rect {
        x: f32,
        y: f32,
        z: f32,
        width: f32,
        height: f32,
        color: Color,
    },
}

pub struct TextTailWorldRenderer {
    pub text_tails: HashMap<u32, TextTailData>,
}

impl TextTailWorldRenderer {
    pub fn new() -> Self {
        Self {
            text_tails: HashMap::new(),
        }
    }

    pub fn add_text_tail(&mut self, data: TextTailData) {
        self.text_tails.insert(data.id, data);
    }

    pub fn remove_text_tail(&mut self, id: u32) {
        self.text_tails.remove(&id);
    }

    pub fn clear(&mut self) {
        self.text_tails.clear();
    }

    /// Projects a 3D world position (with height offset) to 2D screen coordinates.
    /// Returns Some((screen_x, screen_y, screen_z)) if in front of the camera, None otherwise.
    pub fn project_position(
        world_pos: Vec3,
        height_offset: f32,
        view_proj: &Mat4,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Option<(f32, f32, f32)> {
        let pos_3d = Vec4::new(world_pos.x, world_pos.y, world_pos.z + height_offset, 1.0);
        let clip_coords = *view_proj * pos_3d;

        if clip_coords.w <= 0.0 {
            return None;
        }

        let ndc_x = clip_coords.x / clip_coords.w;
        let ndc_y = clip_coords.y / clip_coords.w;
        let ndc_z = clip_coords.z / clip_coords.w;

        let screen_x = (ndc_x + 1.0) * 0.5 * viewport_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * viewport_height;

        Some((screen_x.floor(), screen_y.floor(), ndc_z))
    }

    pub fn generate_render_elements(
        &self,
        view_proj: &Mat4,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<RenderElement> {
        let mut elements = Vec::new();

        // Temporary structure to hold projected data so we can sort them by Z (back-to-front)
        struct ProjectedTail<'a> {
            data: &'a TextTailData,
            x: f32,
            y: f32,
            z: f32,
        }

        let mut projected_tails = Vec::new();

        for data in self.text_tails.values() {
            if let Some((x, y, z)) = Self::project_position(
                data.position,
                data.height_offset,
                view_proj,
                viewport_width,
                viewport_height,
            ) {
                projected_tails.push(ProjectedTail { data, x, y, z });
            }
        }

        // Sort by Z descending (back-to-front rendering)
        projected_tails.sort_by(|a, b| b.z.partial_cmp(&a.z).unwrap_or(std::cmp::Ordering::Equal));

        for p in projected_tails {
            let mut current_y = p.y;

            // 1. Guild Name (if exists)
            if let Some(guild) = &p.data.guild_name {
                let color = p.data.kingdom_color.clone().unwrap_or(Color::white());
                elements.push(RenderElement::Text {
                    x: p.x,
                    y: current_y,
                    z: p.z,
                    text: guild.clone(),
                    color,
                    centered: true,
                });
                current_y += 15.0; // move down for next element
            }

            // 2. Character Name (with Level if exists)
            let mut name_text = String::new();
            if let Some(lvl) = p.data.level {
                name_text.push_str(&format!("[Lv. {}] ", lvl));
            }
            name_text.push_str(&p.data.name);

            elements.push(RenderElement::Text {
                x: p.x,
                y: current_y,
                z: p.z,
                text: name_text,
                color: p.data.color.clone(),
                centered: true,
            });
            current_y += 15.0;

            // 3. HP Bar (if exists)
            if let Some(hp) = p.data.hp_percent {
                let bar_width = 50.0;
                let bar_height = 5.0;
                let start_x = p.x - bar_width * 0.5;

                // Background
                elements.push(RenderElement::Rect {
                    x: start_x,
                    y: current_y,
                    z: p.z,
                    width: bar_width,
                    height: bar_height,
                    color: Color::black(),
                });

                // Fill
                let fill_width = bar_width * (hp as f32 / 100.0);
                elements.push(RenderElement::Rect {
                    x: start_x,
                    y: current_y,
                    z: p.z,
                    width: fill_width,
                    height: bar_height,
                    color: Color::red(),
                });
            }
        }

        elements
    }
}
