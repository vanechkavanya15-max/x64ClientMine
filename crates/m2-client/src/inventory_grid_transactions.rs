use crate::send_item_packets_v2::{TPacketCGItemMove, TPacketCGItemUse};

pub const INVENTORY_PAGE_SIZE: u16 = 45; // 5 columns x 9 rows
pub const INVENTORY_PAGE_COLUMN: u16 = 5;
pub const INVENTORY_PAGE_ROW: u16 = 9;
pub const INVENTORY_PAGE_COUNT: u16 = 4;
pub const INVENTORY_MAX_NUM: u16 = INVENTORY_PAGE_SIZE * INVENTORY_PAGE_COUNT;
pub const WINDOW_TYPE_INVENTORY: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemData {
    pub vnum: u32,
    pub count: u8,
    pub size: u8, // 1, 2, or 3 vertical slots
}

#[derive(Debug, Clone)]
pub struct InventoryGrid {
    slots: Vec<Option<ItemData>>,
}

impl InventoryGrid {
    pub fn new() -> Self {
        Self {
            slots: vec![None; INVENTORY_MAX_NUM as usize],
        }
    }

    pub fn get_item(&self, cell: u16) -> Option<ItemData> {
        if cell >= INVENTORY_MAX_NUM {
            return None;
        }
        self.slots[cell as usize]
    }

    pub fn set_item(&mut self, cell: u16, item: Option<ItemData>) {
        if cell >= INVENTORY_MAX_NUM {
            return;
        }
        self.slots[cell as usize] = item;
    }

    pub fn is_empty(&self, cell: u16) -> bool {
        self.get_item(cell).is_none()
    }

    pub fn can_place_item(&self, cell: u16, size: u8) -> bool {
        if size == 0 || size > 3 {
            return false;
        }
        if cell >= INVENTORY_MAX_NUM {
            return false;
        }

        let _page = cell / INVENTORY_PAGE_SIZE;
        let cell_in_page = cell % INVENTORY_PAGE_SIZE;
        let row_in_page = cell_in_page / INVENTORY_PAGE_COLUMN;
        
        // Check if the item crosses the bottom boundary of the page
        if row_in_page + (size as u16) - 1 >= INVENTORY_PAGE_ROW {
            return false;
        }

        // Check if all required cells are empty
        for i in 0..size {
            let target_cell = cell + (i as u16) * INVENTORY_PAGE_COLUMN;
            if target_cell >= INVENTORY_MAX_NUM {
                return false;
            }
            if !self.is_empty(target_cell) {
                return false;
            }
        }

        true
    }
    
    // Check if an item can be placed, optionally ignoring its current position
    // (useful when checking if an item can move to a spot that overlaps its current spot)
    pub fn can_place_item_ignore(&self, cell: u16, size: u8, ignore_cell: u16) -> bool {
        if size == 0 || size > 3 {
            return false;
        }
        if cell >= INVENTORY_MAX_NUM {
            return false;
        }

        let _page = cell / INVENTORY_PAGE_SIZE;
        let cell_in_page = cell % INVENTORY_PAGE_SIZE;
        let row_in_page = cell_in_page / INVENTORY_PAGE_COLUMN;
        
        // Check if the item crosses the bottom boundary of the page
        if row_in_page + (size as u16) - 1 >= INVENTORY_PAGE_ROW {
            return false;
        }
        
        // Find cells occupied by the ignored item
        let mut ignored_cells = Vec::new();
        if ignore_cell < INVENTORY_MAX_NUM {
            if let Some(item) = self.get_item(ignore_cell) {
                for i in 0..item.size {
                    if ignore_cell + (i as u16) * INVENTORY_PAGE_COLUMN < INVENTORY_MAX_NUM {
                        ignored_cells.push(ignore_cell + (i as u16) * INVENTORY_PAGE_COLUMN);
                    }
                }
            }
        }

        // Check if all required cells are empty (or belong to the ignored item)
        for i in 0..size {
            let target_cell = cell + (i as u16) * INVENTORY_PAGE_COLUMN;
            if target_cell >= INVENTORY_MAX_NUM {
                return false;
            }
            if !self.is_empty(target_cell) && !ignored_cells.contains(&target_cell) {
                return false;
            }
        }

        true
    }

    pub fn move_item(&mut self, src_cell: u16, dst_cell: u16) -> Result<TPacketCGItemMove, &'static str> {
        if src_cell >= INVENTORY_MAX_NUM || dst_cell >= INVENTORY_MAX_NUM {
            return Err("Invalid cell");
        }

        if src_cell == dst_cell {
            return Err("Same cell");
        }

        let src_item = self.get_item(src_cell).ok_or("Source cell is empty")?;

        if !self.can_place_item_ignore(dst_cell, src_item.size, src_cell) {
            return Err("Cannot place item at destination");
        }

        // Perform the move (clear old visually too, just for completeness of our internal logic test)
        for i in 0..src_item.size {
            self.set_item(src_cell + (i as u16) * INVENTORY_PAGE_COLUMN, None);
        }
        for i in 0..src_item.size {
            self.set_item(dst_cell + (i as u16) * INVENTORY_PAGE_COLUMN, Some(src_item));
        }

        Ok(TPacketCGItemMove::new(
            WINDOW_TYPE_INVENTORY, src_cell,
            WINDOW_TYPE_INVENTORY, dst_cell,
            src_item.count
        ))
    }

    pub fn swap_items(&mut self, src_cell: u16, dst_cell: u16) -> Result<TPacketCGItemMove, &'static str> {
        if src_cell >= INVENTORY_MAX_NUM || dst_cell >= INVENTORY_MAX_NUM {
            return Err("Invalid cell");
        }

        if src_cell == dst_cell {
            return Err("Same cell");
        }

        let src_item = self.get_item(src_cell).ok_or("Source cell is empty")?;
        let dst_item = self.get_item(dst_cell).ok_or("Destination cell is empty")?;

        // Temporarily remove both items to check if they can be placed in each other's spots
        for i in 0..src_item.size {
            self.set_item(src_cell + (i as u16) * INVENTORY_PAGE_COLUMN, None);
        }
        for i in 0..dst_item.size {
            self.set_item(dst_cell + (i as u16) * INVENTORY_PAGE_COLUMN, None);
        }

        let src_can_place = self.can_place_item(dst_cell, src_item.size);
        let dst_can_place = self.can_place_item(src_cell, dst_item.size);

        if src_can_place && dst_can_place {
            for i in 0..src_item.size {
                self.set_item(dst_cell + (i as u16) * INVENTORY_PAGE_COLUMN, Some(src_item));
            }
            for i in 0..dst_item.size {
                self.set_item(src_cell + (i as u16) * INVENTORY_PAGE_COLUMN, Some(dst_item));
            }
            Ok(TPacketCGItemMove::new(
                WINDOW_TYPE_INVENTORY, src_cell,
                WINDOW_TYPE_INVENTORY, dst_cell,
                src_item.count
            ))
        } else {
            // Revert
            for i in 0..src_item.size {
                self.set_item(src_cell + (i as u16) * INVENTORY_PAGE_COLUMN, Some(src_item));
            }
            for i in 0..dst_item.size {
                self.set_item(dst_cell + (i as u16) * INVENTORY_PAGE_COLUMN, Some(dst_item));
            }
            Err("Cannot swap items due to size constraints")
        }
    }

    pub fn split_stack(&mut self, src_cell: u16, dst_cell: u16, count: u8) -> Result<TPacketCGItemMove, &'static str> {
        if src_cell >= INVENTORY_MAX_NUM || dst_cell >= INVENTORY_MAX_NUM {
            return Err("Invalid cell");
        }

        if src_cell == dst_cell {
            return Err("Same cell");
        }
        
        if count == 0 {
            return Err("Cannot split 0 items");
        }

        let mut src_item = self.get_item(src_cell).ok_or("Source cell is empty")?;
        
        if count >= src_item.count {
            return Err("Split count must be strictly less than the total count");
        }

        if !self.can_place_item(dst_cell, src_item.size) {
            return Err("Cannot place split item at destination");
        }

        src_item.count -= count;
        self.set_item(src_cell, Some(src_item));
        
        let mut dst_item = src_item.clone();
        dst_item.count = count;
        self.set_item(dst_cell, Some(dst_item));

        Ok(TPacketCGItemMove::new(
            WINDOW_TYPE_INVENTORY, src_cell,
            WINDOW_TYPE_INVENTORY, dst_cell,
            count
        ))
    }

    #[allow(dead_code)]
    pub fn use_item(&mut self, cell: u16) -> Result<TPacketCGItemUse, &'static str> {
        if cell >= INVENTORY_MAX_NUM {
            return Err("Invalid cell");
        }
        
        if self.is_empty(cell) {
            return Err("Cell is empty");
        }

        Ok(TPacketCGItemUse::new(WINDOW_TYPE_INVENTORY, cell))
    }
}
