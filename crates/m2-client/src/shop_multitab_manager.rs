use std::collections::BTreeMap;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::slice;

pub const SHOP_TAB_COUNT_MAX: u8 = 3;
pub const SHOP_HOST_ITEM_MAX_NUM: u8 = 40;
pub const SHOP_COIN_TYPE_GOLD: u8 = 0;
pub const SHOP_COIN_TYPE_SECONDARY_COIN: u8 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl Default for TItemPos {
    fn default() -> Self {
        Self {
            window_type: 1, // INVENTORY
            cell: 0xffff,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TShopItemTableEx {
    pub vnum: u32,
    pub count: u8,
    pub pos: TItemPos,
    pub price: u32,
    pub display_pos: u8,
    pub coin_type: u8,
}

pub struct ShopMultitabManager {
    tab_count: u8,
    private_shop_stock: BTreeMap<TItemPos, TShopItemTableEx>,
}

impl ShopMultitabManager {
    pub fn new() -> Self {
        Self {
            tab_count: 1,
            private_shop_stock: BTreeMap::new(),
        }
    }

    pub fn set_tab_count(&mut self, count: u8) {
        if count <= SHOP_TAB_COUNT_MAX {
            self.tab_count = count;
        } else {
            self.tab_count = SHOP_TAB_COUNT_MAX;
        }
    }

    pub fn get_tab_count(&self) -> u8 {
        self.tab_count
    }

    pub fn clear_private_shop_stock(&mut self) {
        self.private_shop_stock.clear();
    }

    pub fn add_private_shop_item_stock(
        &mut self,
        item_pos: TItemPos,
        display_pos: u8,
        price: u32,
        coin_type: u8,
    ) {
        self.private_shop_stock.remove(&item_pos);

        let item = TShopItemTableEx {
            vnum: 0,
            count: 0,
            pos: item_pos,
            price,
            display_pos,
            coin_type,
        };
        self.private_shop_stock.insert(item_pos, item);
    }

    pub fn del_private_shop_item_stock(&mut self, item_pos: TItemPos) {
        self.private_shop_stock.remove(&item_pos);
    }

    pub fn get_private_shop_item_price_ex(&self, item_pos: TItemPos) -> Option<(u32, u8)> {
        self.private_shop_stock
            .get(&item_pos)
            .map(|item| (item.price, item.coin_type))
    }

    pub fn calculate_total_price(&self) -> (u64, u64) {
        let mut total_gold = 0u64;
        let mut total_secondary = 0u64;

        for item in self.private_shop_stock.values() {
            if item.coin_type == SHOP_COIN_TYPE_GOLD {
                total_gold += item.price as u64;
            } else if item.coin_type == SHOP_COIN_TYPE_SECONDARY_COIN {
                total_secondary += item.price as u64;
            }
        }

        (total_gold, total_secondary)
    }

    pub fn build_private_shop(&self, _name: &str) -> Vec<TShopItemTableEx> {
        let mut items: Vec<TShopItemTableEx> = self.private_shop_stock.values().cloned().collect();
        items.sort_by_key(|a| a.display_pos);
        items
    }
}

// Global instance for FFI
use once_cell::sync::Lazy;
use std::sync::Mutex;

static SHOP_MANAGER: Lazy<Mutex<ShopMultitabManager>> = Lazy::new(|| {
    Mutex::new(ShopMultitabManager::new())
});

#[no_mangle]
pub extern "C" fn m2_shop_get_tab_count() -> u8 {
    let manager = SHOP_MANAGER.lock().unwrap();
    manager.get_tab_count()
}

#[no_mangle]
pub extern "C" fn m2_shop_set_tab_count(count: u8) {
    let mut manager = SHOP_MANAGER.lock().unwrap();
    manager.set_tab_count(count);
}

#[no_mangle]
pub extern "C" fn m2_shop_clear_private_shop_stock() {
    let mut manager = SHOP_MANAGER.lock().unwrap();
    manager.clear_private_shop_stock();
}

#[no_mangle]
pub extern "C" fn m2_shop_add_private_shop_item_stock(
    window_type: u8,
    cell: u16,
    display_pos: u8,
    price: u32,
    coin_type: u8,
) {
    let pos = TItemPos { window_type, cell };
    let mut manager = SHOP_MANAGER.lock().unwrap();
    manager.add_private_shop_item_stock(pos, display_pos, price, coin_type);
}

#[no_mangle]
pub extern "C" fn m2_shop_del_private_shop_item_stock(window_type: u8, cell: u16) {
    let pos = TItemPos { window_type, cell };
    let mut manager = SHOP_MANAGER.lock().unwrap();
    manager.del_private_shop_item_stock(pos);
}

#[no_mangle]
pub extern "C" fn m2_shop_get_item_price_ex(
    window_type: u8,
    cell: u16,
    out_price: *mut u32,
    out_coin_type: *mut u8,
) -> bool {
    let pos = TItemPos { window_type, cell };
    let manager = SHOP_MANAGER.lock().unwrap();
    if let Some((price, coin_type)) = manager.get_private_shop_item_price_ex(pos) {
        if !out_price.is_null() {
            unsafe { *out_price = price; }
        }
        if !out_coin_type.is_null() {
            unsafe { *out_coin_type = coin_type; }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_shop_calculate_total_price(
    out_total_gold: *mut u64,
    out_total_secondary: *mut u64,
) {
    let manager = SHOP_MANAGER.lock().unwrap();
    let (gold, sec) = manager.calculate_total_price();
    if !out_total_gold.is_null() {
        unsafe { *out_total_gold = gold; }
    }
    if !out_total_secondary.is_null() {
        unsafe { *out_total_secondary = sec; }
    }
}

#[no_mangle]
pub extern "C" fn m2_shop_build_private_shop(
    name: *const c_char,
    out_items: *mut TShopItemTableEx,
    out_count: *mut usize,
) -> bool {
    if name.is_null() || out_items.is_null() || out_count.is_null() {
        return false;
    }
    
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let manager = SHOP_MANAGER.lock().unwrap();
    let items = manager.build_private_shop(name_str);
    
    unsafe {
        *out_count = items.len();
        let out_slice = slice::from_raw_parts_mut(out_items, items.len());
        out_slice.copy_from_slice(&items);
    }
    
    true
}
