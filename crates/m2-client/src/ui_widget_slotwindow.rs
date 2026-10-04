#[derive(Debug, Clone, PartialEq)]
pub struct SlotItem {
    pub vnum: u32,
    pub count: u32,
    pub icon_path: String,
    pub is_locked: bool,
    pub color: [f32; 4],
    pub cool_time: f32,
    pub elapsed_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlotCell {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub item: Option<SlotItem>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SlotWindow {
    pub columns: usize,
    pub rows: usize,
    pub cell_size: (i32, i32),
    pub cells: Vec<SlotCell>,
    pub selected_slot: Option<usize>,
    pub dragged_slot: Option<usize>,
    pub over_in_event: Option<u32>,
    pub over_out_event: Option<u32>,
    pub select_item_event: Option<u32>,
}

impl SlotItem {
    pub fn new(vnum: u32, count: u32, icon_path: String, is_locked: bool) -> Self {
        Self {
            vnum,
            count,
            icon_path,
            is_locked,
            color: [1.0, 1.0, 1.0, 1.0],
            cool_time: 0.0,
            elapsed_time: 0.0,
        }
    }
}

impl SlotWindow {
    pub fn new(columns: usize, rows: usize, cell_size: (i32, i32)) -> Self {
        let mut cells = Vec::with_capacity(columns * rows);
        for y in 0..rows {
            for x in 0..columns {
                cells.push(SlotCell {
                    id: y * columns + x,
                    x: (x as i32) * cell_size.0,
                    y: (y as i32) * cell_size.1,
                    w: cell_size.0,
                    h: cell_size.1,
                    item: None,
                });
            }
        }
        Self {
            columns,
            rows,
            cell_size,
            cells,
            selected_slot: None,
            dragged_slot: None,
            over_in_event: None,
            over_out_event: None,
            select_item_event: None,
        }
    }

    pub fn append_slot(&mut self, id: usize, x: i32, y: i32, w: i32, h: i32) {
        if !self.has_slot(id) {
            self.cells.push(SlotCell {
                id,
                x,
                y,
                w,
                h,
                item: None,
            });
        }
    }

    pub fn has_slot(&self, id: usize) -> bool {
        self.cells.iter().any(|c| c.id == id)
    }

    pub fn set_item(&mut self, id: usize, item: SlotItem) -> Result<(), &'static str> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            cell.item = Some(item);
            Ok(())
        } else {
            Err("Slot not found")
        }
    }

    pub fn set_slot(&mut self, id: usize, vnum: u32, count: u32, color: [f32; 4]) -> Result<(), &'static str> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            cell.item = Some(SlotItem {
                vnum,
                count,
                icon_path: String::new(),
                is_locked: false,
                color,
                cool_time: 0.0,
                elapsed_time: 0.0,
            });
            Ok(())
        } else {
            Err("Slot not found")
        }
    }

    pub fn set_slot_cool_time(&mut self, id: usize, cool_time: f32, elapsed_time: f32) -> Result<(), &'static str> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            if let Some(ref mut item) = cell.item {
                item.cool_time = cool_time;
                item.elapsed_time = elapsed_time;
                return Ok(());
            }
        }
        Err("Slot or item not found")
    }

    pub fn set_slot_count(&mut self, id: usize, count: u32) -> Result<(), &'static str> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            if let Some(item) = &mut cell.item {
                item.count = count;
            }
            Ok(())
        } else {
            Err("Slot not found")
        }
    }

    pub fn clear_slot(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            cell.item = None;
            Ok(())
        } else {
            Err("Slot not found")
        }
    }

    pub fn clear_all_slots(&mut self) {
        for cell in &mut self.cells {
            cell.item = None;
        }
    }

    pub fn get_cell_index_at_position(&self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        for cell in &self.cells {
            if mouse_x >= cell.x && mouse_x < cell.x + cell.w && mouse_y >= cell.y && mouse_y < cell.y + cell.h {
                return Some(cell.id);
            }
        }
        None
    }

    pub fn on_mouse_over(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let index = self.get_cell_index_at_position(mouse_x, mouse_y);
        self.selected_slot = index;
        index
    }

    pub fn on_mouse_left_down(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let index = self.get_cell_index_at_position(mouse_x, mouse_y);
        if let Some(idx) = index {
            if let Some(cell) = self.cells.iter().find(|c| c.id == idx) {
                if cell.item.is_some() {
                    self.dragged_slot = Some(idx);
                    return Some(idx);
                }
            }
        }
        self.dragged_slot = None;
        None
    }

    pub fn on_mouse_left_up(&mut self, mouse_x: i32, mouse_y: i32) -> Result<Option<(usize, usize)>, &'static str> {
        if let Some(dragged_idx) = self.dragged_slot.take() {
            let target_idx = self.get_cell_index_at_position(mouse_x, mouse_y);

            if let Some(target) = target_idx {
                if dragged_idx != target {
                    let mut dragged_cell_idx = None;
                    let mut target_cell_idx = None;
                    
                    for (i, cell) in self.cells.iter().enumerate() {
                        if cell.id == dragged_idx { dragged_cell_idx = Some(i); }
                        if cell.id == target { target_cell_idx = Some(i); }
                    }
                    
                    if let (Some(d_idx), Some(t_idx)) = (dragged_cell_idx, target_cell_idx) {
                        let temp = self.cells[d_idx].item.take();
                        self.cells[d_idx].item = self.cells[t_idx].item.take();
                        self.cells[t_idx].item = temp;
                        return Ok(Some((dragged_idx, target)));
                    }
                }
            } else {
                 return Ok(None);
            }
        }
        Ok(None)
    }

    pub fn on_mouse_right_click(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        self.get_cell_index_at_position(mouse_x, mouse_y)
    }
}
