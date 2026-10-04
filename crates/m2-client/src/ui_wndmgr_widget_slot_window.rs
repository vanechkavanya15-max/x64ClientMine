// ui_wndmgr_widget_slot_window.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub index: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub vnum: u32,
    pub count: u32,
    
    // CoolTime properties
    pub cool_time: f32,
    pub start_cool_time: f32,
    
    pub active: bool,
    pub is_locked: bool,
    
    pub placed_item_width: u8,
    pub placed_item_height: u8,
}

impl Slot {
    pub fn new(index: u32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            index,
            x,
            y,
            width,
            height,
            vnum: 0,
            count: 0,
            cool_time: 0.0,
            start_cool_time: 0.0,
            active: false,
            is_locked: false,
            placed_item_width: 1,
            placed_item_height: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoolTimeVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SlotWindow {
    pub slots: Vec<Slot>,
    pub selected_slots: Vec<u32>,
    
    
    pub pos_x: i32,
    pub pos_y: i32,
}

impl SlotWindow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append_slot(&mut self, index: u32, x: i32, y: i32, width: i32, height: i32) {
        if self.get_slot(index).is_none() {
            self.slots.push(Slot::new(index, x, y, width, height));
        }
    }

    pub fn get_slot(&self, index: u32) -> Option<&Slot> {
        self.slots.iter().find(|s| s.index == index)
    }

    pub fn get_slot_mut(&mut self, index: u32) -> Option<&mut Slot> {
        self.slots.iter_mut().find(|s| s.index == index)
    }

    pub fn set_item(&mut self, index: u32, vnum: u32, count: u32) -> Result<(), &'static str> {
        if let Some(slot) = self.get_slot_mut(index) {
            slot.vnum = vnum;
            slot.count = count;
            Ok(())
        } else {
            Err("Slot not found")
        }
    }

    pub fn set_cool_time(&mut self, index: u32, cool_time: f32, start_cool_time: f32) -> Result<(), &'static str> {
        if let Some(slot) = self.get_slot_mut(index) {
            slot.cool_time = cool_time;
            slot.start_cool_time = start_cool_time;
            Ok(())
        } else {
            Err("Slot not found")
        }
    }
    
    pub fn calculate_cooltime_vertices(&self, center_x: f32, center_y: f32, radius: f32, current_time: f32, start_time: f32, total_time: f32) -> Vec<CoolTimeVertex> {
        if total_time <= 0.0 {
            return vec![];
        }
        
        let mut time = (current_time - start_time) / total_time;
        if time >= 1.0 {
            return vec![];
        }
        if time < 0.0 {
            time = 0.0;
        }

        let box_pos = [
            (-1.0, -1.0),
            (-1.0, 0.0),
            (-1.0, 1.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (1.0, -1.0),
            (0.0, -1.0),
        ];

        let tri_count = (8.0 - 8.0 * time) as i32;
        let last_percentage = (8.0 - 8.0 * time) - (tri_count as f32);

        let mut vertices = Vec::with_capacity(10);
        vertices.push(CoolTimeVertex { x: center_x, y: center_y, z: 0.0 });
        vertices.push(CoolTimeVertex { x: center_x, y: center_y - radius, z: 0.0 });

        for j in 0..tri_count {
            let (bx, by) = box_pos[j as usize];
            vertices.push(CoolTimeVertex {
                x: center_x + bx * radius,
                y: center_y + by * radius,
                z: 0.0,
            });
        }

        if last_percentage > 0.0 {
            let last_idx = ((tri_count - 1 + 8) % 8) as usize;
            let pos_idx = ((tri_count + 8) % 8) as usize;

            let (lx, ly) = box_pos[last_idx];
            let (px, py) = box_pos[pos_idx];

            vertices.push(CoolTimeVertex {
                x: center_x + ((px - lx) * last_percentage + lx) * radius,
                y: center_y + ((py - ly) * last_percentage + ly) * radius,
                z: 0.0,
            });
        }

        vertices
    }

    pub fn get_picked_slot(&self, local_x: i32, local_y: i32) -> Option<&Slot> {
        // According to C++, it searches sequentially and automatic overlapping logic applies
        for slot in &self.slots {
            let item_width = 32;
            let item_height = 32;
            
            let mut cell_width = slot.width;
            let mut cell_height = slot.height;
            
            if slot.vnum != 0 {
                cell_width = cell_width.max(slot.placed_item_width as i32 * item_width);
                cell_height = cell_height.max(slot.placed_item_height as i32 * item_height);
            }
            
            if local_x >= slot.x && local_y >= slot.y && local_x < slot.x + cell_width && local_y < slot.y + cell_height {
                return Some(slot);
            }
        }
        None
    }

    pub fn select_slot(&mut self, index: u32) {
        if !self.selected_slots.contains(&index) {
            if let Some(slot) = self.get_slot(index) {
                if slot.vnum != 0 {
                    self.selected_slots.push(index);
                }
            }
        }
    }

    pub fn clear_selected(&mut self) {
        self.selected_slots.clear();
    }
    
    pub fn is_selected(&self, index: u32) -> bool {
        self.selected_slots.contains(&index)
    }

    pub fn lock_slot(&mut self, index: u32) {
        if let Some(slot) = self.get_slot_mut(index) {
            slot.is_locked = true;
        }
    }
    
    pub fn unlock_slot(&mut self, index: u32) {
        if let Some(slot) = self.get_slot_mut(index) {
            slot.is_locked = false;
        }
    }
    
    pub fn is_locked(&self, index: u32) -> bool {
        if let Some(slot) = self.get_slot(index) {
            slot.is_locked
        } else {
            false
        }
    }

    pub fn get_highlight_rect(&self, index: u32) -> Option<(i32, i32, i32, i32)> {
        if let Some(slot) = self.get_slot(index) {
            Some((
                self.pos_x + slot.x,
                self.pos_y + slot.y,
                self.pos_x + slot.x + slot.width,
                self.pos_y + slot.y + slot.height
            ))
        } else {
            None
        }
    }
}
