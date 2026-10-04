//! Python `shop` multi-tab system bridge (multiple shop pages, currency coin types, tab names).

use std::ffi::{CStr, c_char};
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const SHOP_TAB_NAME_MAX: usize = 32;
pub const SHOP_TAB_COUNT_MAX: u8 = 8;

#[derive(Debug, Clone)]
pub struct ShopTabInfo {
    pub name: String,
    pub coin_type: u8, // 0 = Gold/Yang, 1 = Cheque/Won
    pub item_count: u32,
}

impl Default for ShopTabInfo {
    fn default() -> Self {
        Self {
            name: "Shop".to_string(),
            coin_type: 0,
            item_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct ShopMultiTabState {
    pub tabs: Vec<ShopTabInfo>,
    pub selected_tab: u8,
}

impl Default for ShopMultiTabState {
    fn default() -> Self {
        Self {
            tabs: vec![ShopTabInfo::default()],
            selected_tab: 0,
        }
    }
}

impl ShopMultiTabState {
    pub fn clear(&mut self) {
        self.tabs.clear();
        self.tabs.push(ShopTabInfo::default());
        self.selected_tab = 0;
    }

    pub fn set_tab_count(&mut self, count: u8) {
        let cnt = count.clamp(1, SHOP_TAB_COUNT_MAX) as usize;
        self.tabs.resize_with(cnt, ShopTabInfo::default);
        if self.selected_tab as usize >= cnt {
            self.selected_tab = 0;
        }
    }

    pub fn set_tab_name(&mut self, tab_idx: u8, name: String) -> bool {
        if let Some(tab) = self.tabs.get_mut(tab_idx as usize) {
            tab.name = name;
            true
        } else {
            false
        }
    }

    pub fn set_tab_coin_type(&mut self, tab_idx: u8, coin_type: u8) -> bool {
        if let Some(tab) = self.tabs.get_mut(tab_idx as usize) {
            tab.coin_type = coin_type;
            true
        } else {
            false
        }
    }

    pub fn select_tab(&mut self, tab_idx: u8) -> bool {
        if (tab_idx as usize) < self.tabs.len() {
            self.selected_tab = tab_idx;
            true
        } else {
            false
        }
    }
}

static SHOP_TAB_STATE: OnceLock<Mutex<ShopMultiTabState>> = OnceLock::new();

pub fn get_shop_tab_state() -> &'static Mutex<ShopMultiTabState> {
    SHOP_TAB_STATE.get_or_init(|| Mutex::new(ShopMultiTabState::default()))
}

// C-FFI exports for Python `shop`
#[no_mangle]
pub extern "C" fn m2_py_shop_get_tab_count() -> u8 {
    safe_ffi_call(1, || {
        let state = get_shop_tab_state().lock().unwrap();
        state.tabs.len() as u8
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_set_tab_count(count: u8) {
    safe_ffi_call((), || {
        let mut state = get_shop_tab_state().lock().unwrap();
        state.set_tab_count(count);
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_tab_name(tab_idx: u8, out_buf: *mut c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }
        let state = get_shop_tab_state().lock().unwrap();
        if let Some(tab) = state.tabs.get(tab_idx as usize) {
            let bytes = tab.name.as_bytes();
            let copy_len = bytes.len().min(max_len - 1);
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, copy_len);
                *out_buf.add(copy_len) = 0;
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_set_tab_name(tab_idx: u8, name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
        let mut state = get_shop_tab_state().lock().unwrap();
        state.set_tab_name(tab_idx, name_str)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_tab_coin_type(tab_idx: u8) -> u8 {
    safe_ffi_call(0, || {
        let state = get_shop_tab_state().lock().unwrap();
        state.tabs.get(tab_idx as usize).map(|t| t.coin_type).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_set_tab_coin_type(tab_idx: u8, coin_type: u8) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_shop_tab_state().lock().unwrap();
        state.set_tab_coin_type(tab_idx, coin_type)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_select_tab(tab_idx: u8) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_shop_tab_state().lock().unwrap();
        state.select_tab(tab_idx)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_selected_tab() -> u8 {
    safe_ffi_call(0, || {
        let state = get_shop_tab_state().lock().unwrap();
        state.selected_tab
    })
}
