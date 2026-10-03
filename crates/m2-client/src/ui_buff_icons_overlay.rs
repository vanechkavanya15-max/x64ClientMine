#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuffType {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone)]
pub struct Buff {
    pub id: u32,
    pub buff_type: BuffType,
    pub max_duration_ms: u32,
    pub remaining_duration_ms: u32,
    pub icon_path: String,
}

impl Buff {
    pub fn new(id: u32, buff_type: BuffType, max_duration_ms: u32, icon_path: String) -> Self {
        Self {
            id,
            buff_type,
            max_duration_ms,
            remaining_duration_ms: max_duration_ms,
            icon_path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuffIconLayout {
    pub buff_id: u32,
    pub x: f32,
    pub y: f32,
    pub bar_progress: f32,
}

pub struct BuffIconsOverlay {
    pub buffs: Vec<Buff>,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl BuffIconsOverlay {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            buffs: Vec::new(),
            screen_width,
            screen_height,
        }
    }

    pub fn resize(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
    }

    pub fn add_buff(&mut self, buff: Buff) {
        if let Some(existing) = self.buffs.iter_mut().find(|b| b.id == buff.id) {
            existing.remaining_duration_ms = buff.max_duration_ms;
            existing.max_duration_ms = buff.max_duration_ms;
        } else {
            self.buffs.push(buff);
        }
    }

    pub fn remove_buff(&mut self, buff_id: u32) {
        self.buffs.retain(|b| b.id != buff_id);
    }

    pub fn update(&mut self, delta_time_ms: u32) {
        for buff in &mut self.buffs {
            buff.remaining_duration_ms = buff.remaining_duration_ms.saturating_sub(delta_time_ms);
        }
        self.buffs.retain(|b| b.remaining_duration_ms > 0);
    }

    pub fn get_layouts(&self) -> Vec<BuffIconLayout> {
        let mut layouts = Vec::new();
        let icon_size = 32.0;
        let padding_x = 4.0;
        let padding_y = 4.0;
        let top_margin = 10.0;
        let right_margin = 10.0;

        let mut current_x = self.screen_width - right_margin - icon_size;
        let mut current_y = top_margin;

        for buff in &self.buffs {
            let bar_progress = if buff.max_duration_ms > 0 {
                buff.remaining_duration_ms as f32 / buff.max_duration_ms as f32
            } else {
                0.0
            };

            layouts.push(BuffIconLayout {
                buff_id: buff.id,
                x: current_x,
                y: current_y,
                bar_progress,
            });

            current_x -= icon_size + padding_x;
            if current_x < 0.0 {
                current_x = self.screen_width - right_margin - icon_size;
                current_y += icon_size + padding_y;
            }
        }

        layouts
    }
}
