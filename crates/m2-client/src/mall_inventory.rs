use m2_client_bridge::state::inventory::Inventory;
use m2_client_bridge::state::ItemData;

pub const MALL_INVENTORY_SIZE: usize = 45;

pub struct MallInventory {
    slots: Vec<Option<ItemData>>,
}

impl MallInventory {
    pub fn new() -> Self {
        Self {
            slots: vec![None; MALL_INVENTORY_SIZE],
        }
    }

    pub fn set_item(&mut self, slot: u8, item: ItemData) -> Result<(), &'static str> {
        let slot_idx = slot as usize;
        if slot_idx >= MALL_INVENTORY_SIZE {
            return Err("Invalid slot index");
        }
        self.slots[slot_idx] = Some(item);
        Ok(())
    }

    pub fn checkout_mall_item(
        &mut self,
        mall_slot: u8,
        inventory: &mut Inventory,
    ) -> Result<ItemData, &'static str> {
        let slot_idx = mall_slot as usize;
        if slot_idx >= MALL_INVENTORY_SIZE {
            return Err("Invalid mall slot index");
        }

        if self.slots[slot_idx].is_none() {
            return Err("No item at the specified mall slot");
        }

        let mut empty_inv_slot = None;
        for i in 0..180 {
            if let Ok(0) = inventory.get(i) {
                empty_inv_slot = Some(i);
                break;
            }
        }

        if let Some(inv_slot) = empty_inv_slot {
            let item = self.slots[slot_idx].take().unwrap();
            if inventory.set(inv_slot, item.vnum).is_err() {
                // If setting fails for some unexpected reason, restore item to mall
                self.slots[slot_idx] = Some(item);
                return Err("Failed to set item in inventory");
            }
            Ok(item)
        } else {
            Err("Brak miejsca w inwentarzu")
        }
    }

    pub fn insert_from_inventory(
        &mut self,
        _mall_slot: u8,
        _inventory: &mut Inventory,
        _inv_slot: usize,
    ) -> Result<(), &'static str> {
        Err("Przedmioty w Mall moga byc wylacznie wyciagane do ekwipunku")
    }
}
