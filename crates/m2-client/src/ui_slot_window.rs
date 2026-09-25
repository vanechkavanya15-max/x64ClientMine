//! Modul `ui_slot_window` — Kontrolka slotow przedmiotow i skilli UI::CSlotWindow
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonSlotWindow.cpp`
//! Opis: Odwzorowanie UI::CSlotWindow: siatka slotow, przypisywanie ikony przedmiotu, wyswietlanie licznika sztuk (count), animacja zegarowa odnowienia (cooldown).

use std::collections::{HashMap, VecDeque};

pub const ITEM_WIDTH: i32 = 32;
pub const ITEM_HEIGHT: i32 = 32;
pub const SLOT_NUMBER_NONE: u32 = 0xffffffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotStyle {
    None,
    PickUp,
    Select,
}

impl Default for SlotStyle {
    fn default() -> Self {
        SlotStyle::PickUp
    }
}

pub const SLOT_STATE_LOCK: u32 = 1 << 0;
pub const SLOT_STATE_CANT_USE: u32 = 1 << 1;
pub const SLOT_STATE_DISABLE: u32 = 1 << 2;
pub const SLOT_STATE_ALWAYS_RENDER_COVER: u32 = 1 << 3;

#[derive(Debug, Clone, Default)]
pub struct Slot {
    pub state: u32,
    pub slot_number: u32,
    pub center_slot_number: u32,
    pub item_index: u32,
    pub is_item: bool,

    // CoolTime
    pub cool_time: f32,
    pub start_cool_time: f32,

    // Toggle
    pub active: bool,

    pub x_position: i32,
    pub y_position: i32,

    pub x_cell_size: i32,
    pub y_cell_size: i32,

    pub x_placed_item_size: u8,
    pub y_placed_item_size: u8,

    pub render_base_slot_image: bool,
    pub count: u32,
    pub count_grade: Option<u32>,
}

impl Slot {
    pub fn new() -> Self {
        Self {
            state: 0,
            slot_number: SLOT_NUMBER_NONE,
            center_slot_number: SLOT_NUMBER_NONE,
            item_index: 0,
            is_item: false,
            cool_time: 0.0,
            start_cool_time: 0.0,
            active: false,
            x_position: 0,
            y_position: 0,
            x_cell_size: 0,
            y_cell_size: 0,
            x_placed_item_size: 1,
            y_placed_item_size: 1,
            render_base_slot_image: true,
            count: 0,
            count_grade: None,
        }
    }

    pub fn clear(&mut self) {
        self.active = false;
        self.x_placed_item_size = 1;
        self.y_placed_item_size = 1;

        self.is_item = false;
        self.state = 0;
        self.cool_time = 0.0;
        self.start_cool_time = 0.0;
        self.center_slot_number = SLOT_NUMBER_NONE;

        self.item_index = 0;
        self.render_base_slot_image = true;
        self.count = 0;
        self.count_grade = None;
    }
}

#[derive(Debug, Clone, Default)]
pub struct UiSlotWindow {
    pub initialized: bool,
    
    pub slot_type: u32,
    pub slot_style: SlotStyle,
    pub selected_slot_index_list: Vec<u32>,
    pub slot_list: HashMap<u32, Slot>,
    pub tooltip_slot_number: u32,

    pub is_use_mode: bool,
    pub is_usable_item: bool,

    pub reserve_destroy_effect_deque: VecDeque<u32>,
}

impl UiSlotWindow {
    pub fn new() -> Self {
        let mut window = Self::default();
        window.initialize();
        window
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    fn initialize(&mut self) {
        self.initialized = true;
        self.slot_type = 0;
        self.slot_style = SlotStyle::PickUp;
        self.tooltip_slot_number = SLOT_NUMBER_NONE;

        self.is_use_mode = false;
        self.is_usable_item = false;
    }

    pub fn destroy(&mut self) {
        for slot in self.slot_list.values_mut() {
            slot.clear();
        }
        self.slot_list.clear();
        self.initialize();
    }

    pub fn set_slot_type(&mut self, slot_type: u32) {
        self.slot_type = slot_type;
    }

    pub fn set_slot_style(&mut self, style: SlotStyle) {
        self.slot_style = style;
    }

    pub fn append_slot(&mut self, index: u32, x_position: i32, y_position: i32, x_cell_size: i32, y_cell_size: i32) {
        let mut slot = Slot::new();
        slot.slot_number = index;
        slot.center_slot_number = index;
        slot.x_position = x_position;
        slot.y_position = y_position;
        slot.x_cell_size = x_cell_size;
        slot.y_cell_size = y_cell_size;
        
        self.slot_list.insert(index, slot);
    }
    
    pub fn set_always_render_cover_button(&mut self, index: u32, always_render: bool) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            if always_render {
                slot.state |= SLOT_STATE_ALWAYS_RENDER_COVER;
            } else {
                slot.state &= !SLOT_STATE_ALWAYS_RENDER_COVER;
            }
        }
    }

    pub fn show_slot_base_image(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.render_base_slot_image = true;
        }
    }

    pub fn hide_slot_base_image(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.render_base_slot_image = false;
        }
    }

    pub fn has_slot(&self, index: u32) -> bool {
        self.slot_list.contains_key(&index)
    }

    pub fn clear_all_slot(&mut self) {
        self.destroy();
    }

    pub fn clear_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.clear();
        }
    }

    pub fn set_slot(&mut self, index: u32, virtual_number: u32, width: u8, height: u8) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            if slot.is_item && slot.item_index == virtual_number {
                slot.state = 0;
                slot.is_item = true;
                return;
            }
            
            slot.clear();
            slot.state = 0;
            slot.is_item = true;
            slot.item_index = virtual_number;
            
            slot.x_placed_item_size = width;
            slot.y_placed_item_size = height;
        }
    }
    
    pub fn set_slot_count(&mut self, index: u32, count: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.count = count;
            slot.count_grade = None;
        }
    }
    
    pub fn set_slot_count_new(&mut self, index: u32, grade: u32, count: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.count = count;
            slot.count_grade = Some(grade);
        }
    }
    
    pub fn set_slot_cool_time(&mut self, index: u32, cool_time: f32, start_time: f32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.cool_time = cool_time;
            slot.start_cool_time = start_time;
        }
    }
    
    pub fn activate_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.active = true;
        }
    }
    
    pub fn deactivate_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.active = false;
        }
    }

    pub fn get_slot_count(&self) -> usize {
        self.slot_list.len()
    }
    
    pub fn lock_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state |= SLOT_STATE_LOCK;
        }
    }
    
    pub fn unlock_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state &= !SLOT_STATE_LOCK;
        }
    }
    
    pub fn is_lock_slot(&self, index: u32) -> bool {
        if let Some(slot) = self.slot_list.get(&index) {
            return (slot.state & SLOT_STATE_LOCK) != 0;
        }
        false
    }
    
    pub fn set_cant_use_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state |= SLOT_STATE_CANT_USE;
        }
    }
    
    pub fn set_use_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state &= !SLOT_STATE_CANT_USE;
        }
    }
    
    pub fn is_cant_use_slot(&self, index: u32) -> bool {
        if let Some(slot) = self.slot_list.get(&index) {
            return (slot.state & SLOT_STATE_CANT_USE) != 0;
        }
        false
    }
    
    pub fn enable_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state &= !SLOT_STATE_DISABLE;
        }
    }
    
    pub fn disable_slot(&mut self, index: u32) {
        if let Some(slot) = self.slot_list.get_mut(&index) {
            slot.state |= SLOT_STATE_DISABLE;
        }
    }
    
    pub fn is_enable_slot(&self, index: u32) -> bool {
        if let Some(slot) = self.slot_list.get(&index) {
            return (slot.state & SLOT_STATE_DISABLE) == 0;
        }
        false
    }

    pub fn clear_selected(&mut self) {
        self.selected_slot_index_list.clear();
    }
    
    pub fn select_slot(&mut self, index: u32) {
        if let Some(pos) = self.selected_slot_index_list.iter().position(|&x| x == index) {
            self.selected_slot_index_list.remove(pos);
            return;
        }

        if let Some(slot) = self.slot_list.get(&index) {
            if slot.is_item {
                self.selected_slot_index_list.push(index);
            }
        }
    }
    
    pub fn is_selected_slot(&self, index: u32) -> bool {
        self.selected_slot_index_list.contains(&index)
    }
    
    pub fn get_selected_slot_count(&self) -> usize {
        self.selected_slot_index_list.len()
    }
    
    pub fn get_selected_slot_number(&self, index: usize) -> u32 {
        *self.selected_slot_index_list.get(index).unwrap_or(&SLOT_NUMBER_NONE)
    }
    
    pub fn set_use_mode(&mut self, flag: bool) {
        self.is_use_mode = flag;
    }
    
    pub fn set_usable_item(&mut self, flag: bool) {
        self.is_usable_item = flag;
    }
    
    pub fn reserve_destroy_cool_time_finish_effect(&mut self, index: u32) {
        self.reserve_destroy_effect_deque.push_back(index);
    }
    
    pub fn on_update(&mut self) {
        self.reserve_destroy_effect_deque.clear();
    }

    pub fn get_slot(&self, index: u32) -> Option<&Slot> {
        self.slot_list.get(&index)
    }
}
