#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotWindowType {
    Inventory,
    Equipment,
    QuickSlot,
    SafeBox,
    Exchange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DraggedItem {
    pub window_type: SlotWindowType,
    pub slot_index: u32,
    pub item_vnum: u32,
    pub count: u8,
}

pub struct DragDropSlotController {
    dragged_item: Option<DraggedItem>,
}

impl DragDropSlotController {
    pub fn new() -> Self {
        Self { dragged_item: None }
    }

    pub fn pick_up_item(
        &mut self,
        window_type: SlotWindowType,
        slot_index: u32,
        item_vnum: u32,
        count: u8,
    ) -> Result<(), &'static str> {
        if self.dragged_item.is_some() {
            return Err("An item is already being dragged");
        }
        self.dragged_item = Some(DraggedItem {
            window_type,
            slot_index,
            item_vnum,
            count,
        });
        Ok(())
    }

    pub fn drop_on_slot(
        &mut self,
        target_window: SlotWindowType,
        target_slot_index: u32,
    ) -> Result<DraggedItem, &'static str> {
        if let Some(item) = self.dragged_item {
            if item.window_type == target_window && item.slot_index == target_slot_index {
                // Cannot drop on the exact same slot, reset state and return error
                self.dragged_item = None;
                return Err("Cannot drop on the same slot");
            }
            let dropped = self.dragged_item.take().unwrap();
            Ok(dropped)
        } else {
            Err("No item is currently being dragged")
        }
    }

    pub fn drop_on_ground(&mut self) -> Result<DraggedItem, &'static str> {
        if self.dragged_item.is_some() {
            Ok(self.dragged_item.take().unwrap())
        } else {
            Err("No item is currently being dragged")
        }
    }

    pub fn cancel(&mut self) {
        self.dragged_item = None;
    }

    pub fn get_dragged_item(&self) -> Option<&DraggedItem> {
        self.dragged_item.as_ref()
    }
}
