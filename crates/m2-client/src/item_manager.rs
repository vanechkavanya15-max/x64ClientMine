use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use crate::item_proto_loader::{ItemProto, load_item_protos};

pub struct ItemManager {
    pub items: HashMap<u32, ItemProto>,
    pub selected_vnum: u32,
}

impl ItemManager {
    pub fn new() -> Self {
        let items = load_item_protos();
        Self {
            items,
            selected_vnum: 0,
        }
    }

    pub fn get_selected_item(&self) -> Option<&ItemProto> {
        self.items.get(&self.selected_vnum)
    }
}

static ITEM_MANAGER: OnceLock<Mutex<ItemManager>> = OnceLock::new();

pub fn get_item_manager() -> &'static Mutex<ItemManager> {
    ITEM_MANAGER.get_or_init(|| Mutex::new(ItemManager::new()))
}
