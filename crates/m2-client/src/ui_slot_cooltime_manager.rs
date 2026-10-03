use std::collections::HashMap;

pub const SKILL_TYPE_ACTIVE: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct StoreCoolDown {
    pub cool_time: f32,
    pub elapsed_time: f32,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct SlotState {
    pub cool_time: f32,
    pub start_cool_time: f32,
    pub active: bool,
}

impl Default for SlotState {
    fn default() -> Self {
        Self {
            cool_time: 0.0,
            start_cool_time: 0.0,
            active: false,
        }
    }
}

pub struct SlotCooltimeManager {
    pub cooldown_store: HashMap<u32, HashMap<u32, StoreCoolDown>>,
    pub slots: HashMap<u32, SlotState>,
}

impl SlotCooltimeManager {
    pub fn new() -> Self {
        Self {
            cooldown_store: HashMap::new(),
            slots: HashMap::new(),
        }
    }

    pub fn get_or_create_slot(&mut self, slot_index: u32) -> &mut SlotState {
        self.slots.entry(slot_index).or_insert_with(SlotState::default)
    }

    pub fn get_slot(&self, slot_index: u32) -> Option<&SlotState> {
        self.slots.get(&slot_index)
    }

    pub fn set_slot_cool_time(&mut self, slot_index: u32, cool_time: f32, elapsed_time: f32, current_time: f32) {
        let slot = self.get_or_create_slot(slot_index);
        slot.cool_time = cool_time;
        slot.start_cool_time = current_time - elapsed_time;
    }

    pub fn store_slot_cool_time(&mut self, key: u32, slot_index: u32, cool_time: f32, elapsed_time: f32, current_time: f32) {
        let store = self.cooldown_store.entry(key).or_insert_with(HashMap::new);
        let stored = store.entry(slot_index).or_insert(StoreCoolDown {
            cool_time: 0.0,
            elapsed_time: 0.0,
            active: false,
        });
        stored.cool_time = cool_time;
        stored.elapsed_time = current_time - elapsed_time;
        stored.active = false;
    }

    pub fn restore_slot_cool_time(&mut self, key: u32) {
        if let Some(store) = self.cooldown_store.get(&key) {
            let entries: Vec<(u32, StoreCoolDown)> = store.iter().map(|(&k, &v)| (k, v)).collect();
            for (slot_index, stored) in entries {
                let slot = self.get_or_create_slot(slot_index);
                slot.cool_time = stored.cool_time;
                slot.start_cool_time = stored.elapsed_time;
                
                if stored.active {
                    self.activate_slot(slot_index);
                } else {
                    self.deactivate_slot(slot_index);
                }
            }
        }
    }

    pub fn transfer_slot_cool_time(&mut self, index1: u32, index2: u32, dest_skill_level: i32) {
        let active_store = self.cooldown_store.entry(SKILL_TYPE_ACTIVE).or_insert_with(HashMap::new);
        
        if let Some(slot_cooldown) = active_store.remove(&index1) {
            active_store.insert(index2, slot_cooldown);
            
            if let Some(slot1) = self.slots.get_mut(&index1) {
                slot1.cool_time = 0.0;
                slot1.start_cool_time = 0.0;
            }

            if slot_cooldown.active {
                self.activate_slot(index2);
            }

            if dest_skill_level > 0 {
                let slot2 = self.get_or_create_slot(index2);
                slot2.cool_time = slot_cooldown.cool_time;
                slot2.start_cool_time = slot_cooldown.elapsed_time;
            }
        }
    }

    pub fn clear_stored_slot_cool_time(&mut self, key: u32, slot_index: u32) {
        if let Some(store) = self.cooldown_store.get_mut(&key) {
            store.remove(&slot_index);
        }
    }

    pub fn activate_slot(&mut self, index: u32) {
        let slot = self.get_or_create_slot(index);
        slot.active = true;

        if let Some(store) = self.cooldown_store.get_mut(&1) {
            if let Some(stored) = store.get_mut(&index) {
                stored.active = true;
            }
        }
    }

    pub fn deactivate_slot(&mut self, index: u32) {
        let slot = self.get_or_create_slot(index);
        slot.active = false;

        if let Some(store) = self.cooldown_store.get_mut(&1) {
            if let Some(stored) = store.get_mut(&index) {
                stored.active = false;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct CoolTimeVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn render_cool_time_box_math(center_x: f32, center_y: f32, radius: f32, mut time: f32) -> Vec<CoolTimeVertex> {
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
