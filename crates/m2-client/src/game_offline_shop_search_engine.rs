//! Offline private shop indexing, item search (Glass of Search), price sorting, and filtering.

use std::ffi::{CStr, c_char};
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone)]
pub struct OfflineShopItemEntry {
    pub owner_vid: u32,
    pub owner_name: String,
    pub shop_title: String,
    pub item_vnum: u32,
    pub item_count: u32,
    pub price: u64,
}

#[derive(Debug, Default)]
pub struct OfflineShopSearchEngine {
    pub items: Vec<OfflineShopItemEntry>,
    pub current_results: Vec<OfflineShopItemEntry>,
}

impl OfflineShopSearchEngine {
    pub fn clear(&mut self) {
        self.items.clear();
        self.current_results.clear();
    }

    pub fn add_item(&mut self, item: OfflineShopItemEntry) {
        self.items.push(item);
    }

    pub fn search_by_vnum(&mut self, vnum: u32) -> usize {
        self.current_results = self
            .items
            .iter()
            .filter(|it| it.item_vnum == vnum)
            .cloned()
            .collect();
        self.current_results.sort_by_key(|it| it.price);
        self.current_results.len()
    }

    pub fn search_by_name(&mut self, query: &str) -> usize {
        let q = query.to_lowercase();
        self.current_results = self
            .items
            .iter()
            .filter(|it| it.shop_title.to_lowercase().contains(&q) || it.owner_name.to_lowercase().contains(&q))
            .cloned()
            .collect();
        self.current_results.sort_by_key(|it| it.price);
        self.current_results.len()
    }

    pub fn search_by_price_range(&mut self, min_price: u64, max_price: u64) -> usize {
        self.current_results = self
            .items
            .iter()
            .filter(|it| it.price >= min_price && it.price <= max_price)
            .cloned()
            .collect();
        self.current_results.sort_by_key(|it| it.price);
        self.current_results.len()
    }
}

static SEARCH_ENGINE: OnceLock<Mutex<OfflineShopSearchEngine>> = OnceLock::new();

pub fn get_offline_shop_search_engine() -> &'static Mutex<OfflineShopSearchEngine> {
    SEARCH_ENGINE.get_or_init(|| Mutex::new(OfflineShopSearchEngine::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_offline_shop_clear() {
    safe_ffi_call((), || {
        let mut engine = get_offline_shop_search_engine().lock().unwrap();
        engine.clear();
    })
}

#[no_mangle]
pub extern "C" fn m2_offline_shop_register_item(
    owner_vid: u32,
    owner_name: *const c_char,
    shop_title: *const c_char,
    vnum: u32,
    count: u32,
    price: u64,
) -> bool {
    safe_ffi_call(false, || {
        let o_name = if !owner_name.is_null() {
            unsafe { CStr::from_ptr(owner_name) }.to_string_lossy().into_owned()
        } else {
            String::new()
        };
        let s_title = if !shop_title.is_null() {
            unsafe { CStr::from_ptr(shop_title) }.to_string_lossy().into_owned()
        } else {
            String::new()
        };

        let mut engine = get_offline_shop_search_engine().lock().unwrap();
        engine.add_item(OfflineShopItemEntry {
            owner_vid,
            owner_name: o_name,
            shop_title: s_title,
            item_vnum: vnum,
            item_count: count.max(1),
            price,
        });
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_offline_shop_search_vnum(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        let mut engine = get_offline_shop_search_engine().lock().unwrap();
        engine.search_by_vnum(vnum) as u32
    })
}

#[no_mangle]
pub extern "C" fn m2_offline_shop_get_result(idx: u32, out_vid: *mut u32, out_price: *mut u64) -> bool {
    safe_ffi_call(false, || {
        if out_vid.is_null() || out_price.is_null() {
            return false;
        }
        let engine = get_offline_shop_search_engine().lock().unwrap();
        if let Some(entry) = engine.current_results.get(idx as usize) {
            unsafe {
                *out_vid = entry.owner_vid;
                *out_price = entry.price;
            }
            true
        } else {
            false
        }
    })
}
