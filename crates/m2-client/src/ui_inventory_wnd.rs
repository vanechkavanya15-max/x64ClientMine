//! Modul `ui_inventory_wnd` — Interaktywne Okno Ekwipunku i Zalozonego Sprzetu (I)
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonGridSlotWindow.cpp`
//! Opis: Zbudowanie Okna Ekwipunku (klawisz I): 2 zakladki po 45 slotow (5x9), sylwetka postaci ze slotami na bron, zbroje, helm, tarcze, buty, bransolete, naszyjnik, kolczyki oraz licznik Yang.

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
        Self::PickUp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotState {
    Lock = 1 << 0,
    CantUse = 1 << 1,
    Disable = 1 << 2,
    AlwaysRenderCover = 1 << 3,
}

#[derive(Debug, Clone, Default)]
pub struct Slot {
    pub state: u32,
    pub slot_number: u32,
    pub center_slot_number: u32,
    pub item_index: u32,
    pub is_item: bool,

    pub cool_time: f32,
    pub start_cool_time: f32,

    pub active: bool,

    pub x_position: i32,
    pub y_position: i32,

    pub x_cell_size: i32,
    pub y_cell_size: i32,

    pub x_placed_item_size: u8,
    pub y_placed_item_size: u8,
    
    pub render_base_slot_image: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Helmet,
    Shield,
    Shoes,
    Bracelet,
    Necklace,
    Earrings,
}

#[derive(Debug, Clone, Default)]
pub struct UiInventoryWnd {
    pub initialized: bool,
    pub x_count: u32,
    pub y_count: u32,
    pub slot_vector: Vec<Option<usize>>,
    pub slot_list: Vec<Slot>,
    
    pub slot_style: SlotStyle,
    pub width: i32,
    pub height: i32,

    pub current_tab: u8,
    pub equipment: std::collections::HashMap<EquipmentSlot, u32>,
    pub yang: u64,
}

impl UiInventoryWnd {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            x_count: 0,
            y_count: 0,
            slot_vector: Vec::new(),
            slot_list: Vec::new(),
            slot_style: SlotStyle::PickUp,
            width: 0,
            height: 0,
            current_tab: 0,
            equipment: std::collections::HashMap::new(),
            yang: 0,
        }
    }

    pub fn set_tab(&mut self, tab: u8) {
        if tab <= 1 {
            self.current_tab = tab;
        }
    }

    pub fn get_tab(&self) -> u8 {
        self.current_tab
    }

    pub fn set_equipment_item(&mut self, slot: EquipmentSlot, item_index: u32) {
        self.equipment.insert(slot, item_index);
    }

    pub fn get_equipment_item(&self, slot: EquipmentSlot) -> Option<u32> {
        self.equipment.get(&slot).copied()
    }

    pub fn set_yang(&mut self, amount: u64) {
        self.yang = amount;
    }

    pub fn get_yang(&self) -> u64 {
        self.yang
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
    
    pub fn append_slot(&mut self, index: u32, x_position: i32, y_position: i32, x_cell_size: i32, y_cell_size: i32) {
        let mut slot = Slot::default();
        slot.slot_number = index;
        slot.x_position = x_position;
        slot.y_position = y_position;
        slot.x_cell_size = x_cell_size;
        slot.y_cell_size = y_cell_size;
        
        self.slot_list.push(slot);
    }
    
    pub fn arrange_grid_slot(&mut self, start_index: u32, x_count: u32, y_count: u32, x_slot_size: i32, y_slot_size: i32, x_temporary_size: i32, y_temporary_size: i32) {
        self.destroy();
        
        self.x_count = x_count;
        self.y_count = y_count;
        
        self.slot_vector.clear();
        self.slot_vector.resize((x_count * y_count) as usize, None);
        
        for y in 0..y_count {
            for x in 0..x_count {
                let index = start_index + x + y * x_count;
                let x_position = (x as i32) * (x_slot_size + x_temporary_size);
                let y_position = (y as i32) * (y_slot_size + y_temporary_size);
                
                self.append_slot(index, x_position, y_position, x_slot_size, y_slot_size);
                
                let list_index = self.slot_list.len() - 1;
                self.slot_vector[(x + y * x_count) as usize] = Some(list_index);
            }
        }
        
        let width = (x_count as i32) * (x_slot_size + x_temporary_size);
        let height = (y_count as i32) * (y_slot_size + y_temporary_size);
        self.set_size(width, height);
    }
    
    pub fn get_grid_slot_pointer(&self, x: u32, y: u32) -> Option<&Slot> {
        let slot_index = x + y * self.x_count;
        if slot_index >= self.slot_vector.len() as u32 {
            return None;
        }
        
        self.slot_vector[slot_index as usize].map(|list_index| &self.slot_list[list_index])
    }

    pub fn get_grid_slot_pointer_mut(&mut self, x: u32, y: u32) -> Option<&mut Slot> {
        let slot_index = x + y * self.x_count;
        if slot_index >= self.slot_vector.len() as u32 {
            return None;
        }
        
        if let Some(list_index) = self.slot_vector[slot_index as usize] {
            Some(&mut self.slot_list[list_index])
        } else {
            None
        }
    }

    pub fn get_picked_grid_slot_position(&self, local_x: i32, local_y: i32) -> Option<(u32, u32)> {
        for x in 0..self.x_count {
            for y in 0..self.y_count {
                if let Some(slot) = self.get_grid_slot_pointer(x, y) {
                    if local_x >= slot.x_position
                        && local_y >= slot.y_position
                        && local_x <= slot.x_position + slot.x_cell_size
                        && local_y <= slot.y_position + slot.y_cell_size
                    {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }

    pub fn get_picked_slot_list(&self, width: i32, height: i32, local_x: i32, local_y: i32) -> Option<Vec<&Slot>> {
        if local_x < 0 || local_y < 0 || local_x >= self.width || local_y >= self.height {
            return None;
        }

        let mut slot_pointer_list = Vec::new();

        if let Some((ix, iy)) = self.get_picked_grid_slot_position(local_x, local_y) {
            let x_half_step = width / 2;
            let mut ix_start = (ix as i32) - (x_half_step - (x_half_step % 2));
            let mut ix_end = (ix as i32) + x_half_step;

            let mut iy_start = 0;
            let mut iy_end = 0;

            if height == 1 {
                iy_start = iy as i32;
                iy_end = iy as i32;
            } else if height == 2 {
                iy_start = iy as i32;
                iy_end = (iy as i32) + 1;
            } else if height == 3 {
                iy_start = (iy as i32) - 1;
                iy_end = (iy as i32) + 1;
            }

            if ix_start < 0 {
                ix_end += -ix_start;
                ix_start = 0;
            }

            if iy_start < 0 {
                iy_end += -iy_start;
                iy_start = 0;
            }

            if ix_end as u32 >= self.x_count {
                let ix_temporary = (ix_end as u32) - self.x_count + 1;
                ix_start -= ix_temporary as i32;
                ix_end -= ix_temporary as i32;
            }

            if iy_end as u32 >= self.y_count {
                let iy_temporary = (iy_end as u32) - self.y_count + 1;
                iy_start -= iy_temporary as i32;
                iy_end -= iy_temporary as i32;
            }

            for i in ix_start..=ix_end {
                for j in iy_start..=iy_end {
                    if let Some(slot) = self.get_grid_slot_pointer(i as u32, j as u32) {
                        slot_pointer_list.push(slot);
                    }
                }
            }
            
            if !slot_pointer_list.is_empty() {
                return Some(slot_pointer_list);
            }
        }

        None
    }

    pub fn check_moving(&self, slot_number: u32, item_index: u32, slot_list: &[&Slot]) -> bool {
        if self.slot_style != SlotStyle::PickUp {
            return true;
        }

        for slot in slot_list {
            if slot_number != slot.center_slot_number {
                if slot.item_index != 0 || slot.center_slot_number != slot.slot_number {
                    if item_index != slot.item_index {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn on_refresh_slot(&mut self) {
        let x_count = self.x_count;
        let y_count = self.y_count;
        
        for x in 0..x_count {
            for y in 0..y_count {
                if let Some(slot) = self.get_grid_slot_pointer_mut(x, y) {
                    slot.center_slot_number = slot.slot_number;
                }
            }
        }

        let mut updates: Vec<(u32, u32, u32, u32)> = Vec::new(); // (x, y, center_slot_number, item_index)

        // Record which slots are filled by items
        let mut filled_by_items = std::collections::HashSet::new();
        
        for x in 0..x_count {
            for y in 0..y_count {
                if let Some(slot) = self.get_grid_slot_pointer(x, y) {
                    if slot.is_item {
                        for x_sub in 0..(slot.x_placed_item_size as u32) {
                            for y_sub in 0..(slot.y_placed_item_size as u32) {
                                let target_x = x + x_sub;
                                let target_y = y + y_sub;
                                updates.push((target_x, target_y, slot.slot_number, slot.item_index));
                                filled_by_items.insert((target_x, target_y));
                            }
                        }
                    }
                }
            }
        }
        
        for x in 0..x_count {
            for y in 0..y_count {
                if !filled_by_items.contains(&(x, y)) {
                    if let Some(slot) = self.get_grid_slot_pointer(x, y) {
                        updates.push((x, y, slot.slot_number, 0));
                    }
                }
            }
        }
        
        for (x, y, center_slot_number, item_index) in updates {
            if let Some(slot) = self.get_grid_slot_pointer_mut(x, y) {
                if item_index != 0 {
                    slot.center_slot_number = center_slot_number;
                    slot.item_index = item_index;
                } else {
                    slot.item_index = 0;
                }
            }
        }
    }

    pub fn destroy(&mut self) {
        self.slot_list.clear();
        self.slot_vector.clear();
        self.x_count = 0;
        self.y_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_inventory_wnd() {
        let instance = UiInventoryWnd::new();
        assert!(instance.is_ready());
    }
}
