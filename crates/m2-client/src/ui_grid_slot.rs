//! Modul `ui_grid_slot` — Siatka ekwipunku 5x9 z walidacja wielokrotnych kratek UI::CGridSlotWindow
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonGridSlotWindow.cpp`
//! Opis: Odwzorowanie UI::CGridSlotWindow: sprawdzanie wolnego miejsca dla przedmiotow zajmujacych 1, 2 lub 3 kratki w pionie (np. miecze dwureczne 1x3, zbroje 1x2).

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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TSlot {
    pub dw_state: u32,
    pub dw_slot_number: u32,
    pub dw_center_slot_number: u32,
    pub dw_item_index: u32,
    pub is_item: bool,

    pub f_cool_time: f32,
    pub f_start_cool_time: f32,
    pub b_active: bool,

    pub ix_position: i32,
    pub iy_position: i32,
    pub ix_cell_size: i32,
    pub iy_cell_size: i32,

    pub by_x_placed_item_size: u8,
    pub by_y_placed_item_size: u8,
}

#[derive(Debug, Clone, Default)]
pub struct UiGridSlot {
    pub initialized: bool,
    pub x_count: u32,
    pub y_count: u32,
    pub slot_vector: Vec<TSlot>,
    
    pub width: i32,
    pub height: i32,
    pub slot_style: SlotStyle,
    pub is_use_mode: bool,
}

impl UiGridSlot {
    pub fn new() -> Self {
        Self {
            initialized: true,
            ..Default::default()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn arrange_grid_slot(
        &mut self,
        dw_start_index: u32,
        dw_x_count: u32,
        dw_y_count: u32,
        ix_slot_size: i32,
        iy_slot_size: i32,
        ix_temporary_size: i32,
        iy_temporary_size: i32,
    ) {
        self.x_count = dw_x_count;
        self.y_count = dw_y_count;
        
        self.slot_vector.clear();
        self.slot_vector.resize(
            (dw_x_count * dw_y_count) as usize,
            TSlot::default(),
        );

        for x in 0..dw_x_count {
            for y in 0..dw_y_count {
                let dw_index = dw_start_index + x + y * dw_x_count;
                let ix_position = (x as i32) * (ix_slot_size + ix_temporary_size);
                let iy_position = (y as i32) * (iy_slot_size + iy_temporary_size);

                let slot_idx = (x + y * dw_x_count) as usize;
                let slot = &mut self.slot_vector[slot_idx];
                
                slot.dw_slot_number = dw_index;
                slot.ix_position = ix_position;
                slot.iy_position = iy_position;
                slot.ix_cell_size = ix_slot_size;
                slot.iy_cell_size = iy_slot_size;
            }
        }

        self.width = (dw_x_count as i32) * (ix_slot_size + ix_temporary_size);
        self.height = (dw_y_count as i32) * (iy_slot_size + iy_temporary_size);
    }

    pub fn get_grid_slot_pointer(&self, ix: u32, iy: u32) -> Option<&TSlot> {
        if ix >= self.x_count || iy >= self.y_count {
            return None;
        }
        let dw_slot_index = ix as usize + (iy as usize) * (self.x_count as usize);
        if dw_slot_index >= self.slot_vector.len() {
            return None;
        }
        Some(&self.slot_vector[dw_slot_index])
    }

    pub fn get_grid_slot_pointer_mut(&mut self, ix: u32, iy: u32) -> Option<&mut TSlot> {
        if ix >= self.x_count || iy >= self.y_count {
            return None;
        }
        let dw_slot_index = ix as usize + (iy as usize) * (self.x_count as usize);
        if dw_slot_index >= self.slot_vector.len() {
            return None;
        }
        Some(&mut self.slot_vector[dw_slot_index])
    }

    pub fn get_picked_grid_slot_position(&self, ix_local: i32, iy_local: i32) -> Option<(u32, u32)> {
        for x in 0..self.x_count {
            for y in 0..self.y_count {
                if let Some(slot) = self.get_grid_slot_pointer(x, y) {
                    if ix_local >= slot.ix_position &&
                       iy_local >= slot.iy_position &&
                       ix_local <= slot.ix_position + slot.ix_cell_size &&
                       iy_local <= slot.iy_position + slot.iy_cell_size {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }

    pub fn get_picked_slot_list(&self, i_width: i32, i_height: i32, lx: i32, ly: i32) -> Option<Vec<TSlot>> {
        if lx < 0 || ly < 0 || lx >= self.width || ly >= self.height {
            return None;
        }

        let (ix, iy) = self.get_picked_grid_slot_position(lx, ly)?;
        let mut slot_pointer_list = Vec::new();

        let ix_half_step = i_width / 2;
        let mut ix_start = (ix as i32) - (ix_half_step - (ix_half_step % 2));
        let mut ix_end = (ix as i32) + ix_half_step;

        let mut iy_start = 0;
        let mut iy_end = 0;

        if i_height == 1 {
            iy_start = iy as i32;
            iy_end = iy as i32;
        } else if i_height == 2 {
            iy_start = iy as i32;
            iy_end = (iy as i32) + 1;
        } else if i_height == 3 {
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

        if ix_end >= self.x_count as i32 {
            let ix_temporary = ix_end - (self.x_count as i32) + 1;
            ix_start -= ix_temporary;
            ix_end -= ix_temporary;
        }

        if iy_end >= self.y_count as i32 {
            let iy_temporary = iy_end - (self.y_count as i32) + 1;
            iy_start -= iy_temporary;
            iy_end -= iy_temporary;
        }

        for i in ix_start..=ix_end {
            for j in iy_start..=iy_end {
                if i >= 0 && j >= 0 {
                    if let Some(slot) = self.get_grid_slot_pointer(i as u32, j as u32) {
                        slot_pointer_list.push(slot.clone());
                    }
                }
            }
        }

        if self.is_use_mode && slot_pointer_list.len() == 1 {
            let p_main_slot = slot_pointer_list[0].clone();
            for slot in &self.slot_vector {
                if slot.dw_center_slot_number == p_main_slot.dw_center_slot_number {
                    if !slot_pointer_list.iter().any(|s| s.dw_slot_number == slot.dw_slot_number) {
                        slot_pointer_list.push(slot.clone());
                    }
                }
            }
        }

        if !slot_pointer_list.is_empty() {
            Some(slot_pointer_list)
        } else {
            None
        }
    }

    pub fn on_refresh_slot(&mut self) {
        for x in 0..self.x_count {
            for y in 0..self.y_count {
                if let Some(slot) = self.get_grid_slot_pointer_mut(x, y) {
                    slot.dw_center_slot_number = slot.dw_slot_number;
                }
            }
        }

        let mut changes = Vec::new();

        for x in 0..self.x_count {
            for y in 0..self.y_count {
                if let Some(slot) = self.get_grid_slot_pointer(x, y) {
                    if slot.is_item {
                        for x_sub in 0..slot.by_x_placed_item_size {
                            for y_sub in 0..slot.by_y_placed_item_size {
                                changes.push((x + x_sub as u32, y + y_sub as u32, slot.dw_slot_number, slot.dw_item_index, true));
                            }
                        }
                    } else {
                        changes.push((x, y, slot.dw_slot_number, 0, false));
                    }
                }
            }
        }

        for (cx, cy, dw_slot_number, dw_item_index, is_item) in &changes {
            if *is_item {
                if let Some(sub_slot) = self.get_grid_slot_pointer_mut(*cx, *cy) {
                    sub_slot.dw_center_slot_number = *dw_slot_number;
                    sub_slot.dw_item_index = *dw_item_index;
                }
            }
        }
        
        for (cx, cy, _, _, is_item) in &changes {
            if !*is_item {
                if let Some(sub_slot) = self.get_grid_slot_pointer_mut(*cx, *cy) {
                    if sub_slot.dw_center_slot_number == sub_slot.dw_slot_number {
                        sub_slot.dw_item_index = 0;
                    }
                }
            }
        }
    }

    pub fn check_moving(&self, dw_slot_number: u32, dw_item_index: u32, slot_list: &[TSlot]) -> bool {
        if self.slot_style != SlotStyle::PickUp {
            return true;
        }

        for slot in slot_list {
            if dw_slot_number != slot.dw_center_slot_number {
                if slot.dw_item_index != 0 || slot.dw_center_slot_number != slot.dw_slot_number {
                    if dw_item_index != slot.dw_item_index {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn get_picked_slot_pointer(&self, i_width: i32, i_height: i32, lx: i32, ly: i32, attaching_slot_number: Option<u32>) -> Option<TSlot> {
        let slot_list = self.get_picked_slot_list(i_width, i_height, lx, ly)?;
        
        let mut min_slot: Option<TSlot> = None;
        for slot in slot_list {
            match min_slot.as_ref() {
                None => {
                    min_slot = Some(slot);
                }
                Some(current_min) => {
                    if slot.dw_slot_number < current_min.dw_slot_number {
                        min_slot = Some(slot);
                    } else if !current_min.is_item && slot.is_item {
                        min_slot = Some(slot);
                    }
                }
            }
        }
        
        let min_slot = min_slot?;
        let mut result_slot = None;
        
        for slot in &self.slot_vector {
            if slot.dw_slot_number == min_slot.dw_center_slot_number {
                result_slot = Some(slot.clone());
                break;
            }
        }
        
        let mut final_slot = result_slot?;
        
        if let Some(attaching_slot) = attaching_slot_number {
            if attaching_slot == final_slot.dw_slot_number {
                final_slot = min_slot;
            }
        }
        
        Some(final_slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_grid_slot() {
        let instance = UiGridSlot::new();
        assert!(instance.is_ready());
    }
}
