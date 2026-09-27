use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ItemProto {
    pub vnum: u32,
    pub name: String,
    pub item_type: u8,
    pub item_sub_type: u8,
    pub buy_price: i64,
    pub sell_price: i64,
    pub icon_image: String,
    pub limits: [crate::py_bridge_item::ItemLimit; 2],
    pub applies: [crate::py_bridge_item::ItemApply; 3],
}

pub fn load_item_protos() -> HashMap<u32, ItemProto> {
    // In a real application, this would load data from a file or database.
    // For now, we return an empty hash map.
    HashMap::new()
}
