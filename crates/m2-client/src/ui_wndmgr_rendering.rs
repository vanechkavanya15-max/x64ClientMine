#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum RenderingMode {
    Normal = 0,
    Screen = 1,
    ColorDodge = 2,
    Modulate = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlotCoolTime {
    pub cool_time: f32,
    pub start_time: f32,
}

#[derive(Debug, Clone)]
pub struct UiWindowRendering {
    pub color: Color,
    pub rendering_mode: RenderingMode,
    pub is_show: bool,
    pub slots: std::collections::HashMap<u32, SlotCoolTime>,
}

impl UiWindowRendering {
    pub fn new() -> Self {
        Self {
            color: Color::default(),
            rendering_mode: RenderingMode::Normal,
            is_show: false, // Windows are typically created hidden or we can default to true, wait C++ usually false then Show(), but let's default to false
            slots: std::collections::HashMap::new(),
        }
    }

    pub fn set_diffuse_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = Color { r, g, b, a };
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.color.a = alpha;
    }

    pub fn set_rendering_mode(&mut self, mode: RenderingMode) {
        self.rendering_mode = mode;
    }

    pub fn show(&mut self) {
        self.is_show = true;
    }

    pub fn hide(&mut self) {
        self.is_show = false;
    }

    pub fn is_show(&self) -> bool {
        self.is_show
    }

    pub fn set_slot_cool_time(&mut self, slot_index: u32, cool_time: f32, elapsed_time: f32, current_time: f32) {
        self.slots.insert(slot_index, SlotCoolTime {
            cool_time,
            start_time: current_time - elapsed_time,
        });
    }

    pub fn get_cool_time_percentage(&self, slot_index: u32, current_time: f32) -> f32 {
        if let Some(slot) = self.slots.get(&slot_index) {
            if slot.cool_time > 0.0 {
                let percentage = (current_time - slot.start_time) / slot.cool_time;
                return percentage.clamp(0.0, 1.0);
            }
        }
        1.0
    }

    pub fn get_cool_time_angle(&self, slot_index: u32, current_time: f32) -> f32 {
        let percentage = self.get_cool_time_percentage(slot_index, current_time);
        360.0 * (1.0 - percentage)
    }
}
