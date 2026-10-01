#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotItem {
    pub vnum: u32,
    pub count: u8,
    pub icon_path: String,
    pub is_locked: bool,
}

#[derive(Debug, Clone)]
pub struct SlotCell {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub item: Option<SlotItem>,
}

pub struct SlotWindow {
    pub columns: usize,
    pub rows: usize,
    pub cell_size: (i32, i32),
    pub cells: Vec<SlotCell>,
    pub selected_slot: Option<usize>,
    pub dragged_slot: Option<usize>,
}

impl SlotWindow {
    pub fn new(columns: usize, rows: usize, cell_size: (i32, i32)) -> Self {
        let mut cells = Vec::with_capacity(columns * rows);
        for row in 0..rows {
            for col in 0..columns {
                cells.push(SlotCell {
                    x: col as i32 * cell_size.0,
                    y: row as i32 * cell_size.1,
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
        }
    }

    pub fn set_item(&mut self, index: usize, item: SlotItem) -> Result<(), &'static str> {
        if index >= self.cells.len() {
            return Err("Index out of bounds");
        }
        self.cells[index].item = Some(item);
        Ok(())
    }

    pub fn clear_slot(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.cells.len() {
            return Err("Index out of bounds");
        }
        self.cells[index].item = None;
        Ok(())
    }

    pub fn get_cell_index_at_position(&self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let col = mouse_x / self.cell_size.0;
        let row = mouse_y / self.cell_size.1;
        
        // Ensure within window bounds
        if mouse_x < 0 || mouse_y < 0 || col >= self.columns as i32 || row >= self.rows as i32 {
            return None;
        }

        let index = row as usize * self.columns + col as usize;
        if index < self.cells.len() {
            Some(index)
        } else {
            None
        }
    }

    pub fn on_mouse_over(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let index = self.get_cell_index_at_position(mouse_x, mouse_y);
        self.selected_slot = index;
        index
    }

    pub fn on_mouse_left_down(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        let index = self.get_cell_index_at_position(mouse_x, mouse_y);
        if let Some(idx) = index {
            if self.cells[idx].item.is_some() {
                self.dragged_slot = Some(idx);
                return Some(idx);
            }
        }
        self.dragged_slot = None;
        None
    }

    pub fn on_mouse_left_up(&mut self, mouse_x: i32, mouse_y: i32) -> Result<Option<(usize, usize)>, &'static str> {
        if let Some(dragged_idx) = self.dragged_slot {
            let target_idx = self.get_cell_index_at_position(mouse_x, mouse_y);
            
            self.dragged_slot = None;

            if let Some(target) = target_idx {
                if dragged_idx != target {
                    // Swap items
                    let temp = self.cells[dragged_idx].item.take();
                    self.cells[dragged_idx].item = self.cells[target].item.take();
                    self.cells[target].item = temp;
                    return Ok(Some((dragged_idx, target)));
                }
            } else {
                 // Drop out of bounds
                 return Ok(None);
            }
        }
        Ok(None)
    }

    pub fn on_mouse_right_click(&mut self, mouse_x: i32, mouse_y: i32) -> Option<usize> {
        self.get_cell_index_at_position(mouse_x, mouse_y)
    }
}
