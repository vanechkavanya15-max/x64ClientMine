use std::os::raw::c_char;
use crate::bridge::ffi_guard::safe_ffi_call;
use crate::py_bridge_skill::get_bridge;
use crate::recv_shop_logic_v2::{SHOP_INSTANCE, SHOP_HOST_ITEM_MAX_NUM};

// ============================================================================
// SKILL
// ============================================================================
#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_get_name(skill_id: u32) -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        get_bridge().get_skill_name(skill_id)
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_get_level(skill_id: u32) -> u32 {
    safe_ffi_call(0, || {
        unsafe {
            m2_client_bridge::state::skills::m2_skills_get_global_level(skill_id) as u32
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_get_cool_time(skill_id: u32) -> u32 {
    safe_ffi_call(0, || {
        unsafe {
            let next_use = m2_client_bridge::state::skills::m2_skills_get_global_next_use_time(skill_id);
            let current = m2_client_bridge::timer::get_msec();
            if next_use > current {
                next_use.saturating_sub(current)
            } else {
                0
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_skill_can_use_skill(skill_id: u32) -> bool {
    safe_ffi_call(false, || {
        let skill_level = unsafe { m2_client_bridge::state::skills::m2_skills_get_global_level(skill_id) };
        if skill_level == 0 {
            return false;
        }

        let current_sp = if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
            state.get_sp()
        } else {
            return false;
        };

        let sp_cost = crate::skill_data_cache_v2::m2_skill_get_sp_cost(skill_id, skill_level);

        if (current_sp as u32) < sp_cost {
            return false;
        }

        let next_use = unsafe { m2_client_bridge::state::skills::m2_skills_get_global_next_use_time(skill_id) };
        let current = m2_client_bridge::timer::get_msec();
        if next_use > current {
            return false;
        }

        true
    })
}

// ============================================================================
// SHOP
// ============================================================================
#[no_mangle]
pub extern "C" fn m2_py_gamesys_shop_open() {
    safe_ffi_call((), || {
        if let Ok(mut shop) = SHOP_INSTANCE.write() {
            shop.is_open = true;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_shop_close() {
    safe_ffi_call((), || {
        if let Ok(mut shop) = SHOP_INSTANCE.write() {
            shop.is_open = false;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_shop_get_item_count() -> usize {
    safe_ffi_call(0, || {
        if let Ok(shop) = SHOP_INSTANCE.read() {
            shop.items.iter().filter(|item| item.vnum != 0).count()
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_shop_get_item_id(slot: usize) -> u32 {
    safe_ffi_call(0, || {
        if slot >= SHOP_HOST_ITEM_MAX_NUM {
            return 0;
        }
        if let Ok(shop) = SHOP_INSTANCE.read() {
            shop.items[slot].vnum
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_shop_get_item_price(slot: usize) -> u32 {
    safe_ffi_call(0, || {
        if slot >= SHOP_HOST_ITEM_MAX_NUM {
            return 0;
        }
        if let Ok(shop) = SHOP_INSTANCE.read() {
            shop.items[slot].price
        } else {
            0
        }
    })
}

// ============================================================================
// EXCHANGE
// ============================================================================
use crate::recv_exchange_logic_v2::{get_exchange_logic, ExchangeState, EXCHANGE_ITEM_MAX_NUM};

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_open() {
    safe_ffi_call((), || {
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.state = ExchangeState::Open;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_close() {
    safe_ffi_call((), || {
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.state = ExchangeState::Idle;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_set_item_to_target(slot: usize, vnum: u32, count: u8) {
    safe_ffi_call((), || {
        if slot >= EXCHANGE_ITEM_MAX_NUM {
            return;
        }
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.target_data.item_vnum[slot] = vnum;
            logic.target_data.item_count[slot] = count;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_set_item_to_self(slot: usize, vnum: u32, count: u8) {
    safe_ffi_call((), || {
        if slot >= EXCHANGE_ITEM_MAX_NUM {
            return;
        }
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.self_data.item_vnum[slot] = vnum;
            logic.self_data.item_count[slot] = count;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_set_elk_to_target(elk: u32) {
    safe_ffi_call((), || {
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.target_data.elk = elk;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_set_elk_to_self(elk: u32) {
    safe_ffi_call((), || {
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.self_data.elk = elk;
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_exchange_accept(is_accept: bool) {
    safe_ffi_call((), || {
        if let Ok(mut logic) = get_exchange_logic().write() {
            logic.self_data.accept = if is_accept { 1 } else { 0 };
        }
    })
}

// ============================================================================
// SAFEBOX
// ============================================================================
use crate::safebox_mall_manager_v2::G_SAFEBOX_MALL_MANAGER;

#[no_mangle]
pub extern "C" fn m2_py_gamesys_safebox_get_item_index(slot: usize) -> u32 {
    safe_ffi_call(0, || {
        if let Ok(manager) = G_SAFEBOX_MALL_MANAGER.read() {
            if let Some(item) = manager.get_item_data(slot) {
                return item.vnum;
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_safebox_get_item_count(slot: usize) -> u8 {
    safe_ffi_call(0, || {
        if let Ok(manager) = G_SAFEBOX_MALL_MANAGER.read() {
            if let Some(item) = manager.get_item_data(slot) {
                return item.count;
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_safebox_get_money() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(manager) = G_SAFEBOX_MALL_MANAGER.read() {
            manager.get_money()
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_safebox_get_size() -> usize {
    safe_ffi_call(0, || {
        if let Ok(manager) = G_SAFEBOX_MALL_MANAGER.read() {
            manager.get_current_safebox_size()
        } else {
            0
        }
    })
}

// ============================================================================
// GUILD
// ============================================================================
use crate::recv_guild_logic_v2::get_guild_state;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn m2_py_gamesys_guild_get_guild_id() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = get_guild_state().lock() {
            state.guild_id
        } else {
            0
        }
    })
}

// Memory leak warning: These return dynamically allocated C-strings that C++ needs to free 
// or they'll leak, however in original C++ they return Python string objects.
// Since this is a direct C FFI, we return static or leaked C-strings to avoid segfaults 
// in this emulation layer. A proper way would be a buffer passed by C++. 
// For pure functionality and 1:1 tests in Rust, we can return a Box::into_raw,
// but to be safe without breaking test harness we'll return a pointer to leaked string.
// We'll use a thread_local or static buffer approach if possible, but leaking is safer for tests right now.

thread_local! {
    static LAST_GUILD_NAME: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
    static LAST_GUILD_MASTER_NAME: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
    static LAST_GUILD_MEMBER_NAME: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
    static LAST_QUEST_TITLE: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
    static LAST_QUEST_CLOCK: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_guild_get_guild_name() -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        if let Ok(state) = get_guild_state().lock() {
            let name = state.name.clone();
            LAST_GUILD_NAME.with(|cache| {
                *cache.borrow_mut() = CString::new(name).unwrap_or_default();
                cache.borrow().as_ptr()
            })
        } else {
            std::ptr::null()
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_guild_get_guild_master_name() -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        if let Ok(state) = get_guild_state().lock() {
            let master_pid = state.master_pid;
            let master_name = state.members.get(&master_pid).map(|m| m.name.clone()).unwrap_or_else(|| "Noname".to_string());
            LAST_GUILD_MASTER_NAME.with(|cache| {
                *cache.borrow_mut() = CString::new(master_name).unwrap_or_default();
                cache.borrow().as_ptr()
            })
        } else {
            std::ptr::null()
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_guild_get_member_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = get_guild_state().lock() {
            state.members.len() as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_guild_get_member_name(index: usize) -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        if let Ok(state) = get_guild_state().lock() {
            if let Some(member) = state.members.values().nth(index) {
                let name = member.name.clone();
                return LAST_GUILD_MEMBER_NAME.with(|cache| {
                    *cache.borrow_mut() = CString::new(name).unwrap_or_default();
                    cache.borrow().as_ptr()
                });
            }
        }
        std::ptr::null()
    })
}

// ============================================================================
// QUEST
// ============================================================================
use crate::ui_quest_dialog::get_ui_quest_dialog;

#[no_mangle]
pub extern "C" fn m2_py_gamesys_quest_get_quest_title(_quest_index: usize) -> *const c_char {
    // In our UI quest dialog, there is currently one global dialog title.
    // The C++ python API expects a quest index (for the quest board),
    // but the `ui_quest_dialog` holds the single active title. We'll return that to fulfill the bridge.
    safe_ffi_call(std::ptr::null(), || {
        if let Ok(dialog) = get_ui_quest_dialog().lock() {
            let title = dialog.title.clone();
            LAST_QUEST_TITLE.with(|cache| {
                *cache.borrow_mut() = CString::new(title).unwrap_or_default();
                cache.borrow().as_ptr()
            })
        } else {
            std::ptr::null()
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_quest_get_quest_clock_name(_quest_index: usize) -> *const c_char {
    safe_ffi_call(std::ptr::null(), || {
        if let Ok(dialog) = get_ui_quest_dialog().lock() {
            let clock = dialog.clock_title.clone();
            LAST_QUEST_CLOCK.with(|cache| {
                *cache.borrow_mut() = CString::new(clock).unwrap_or_default();
                cache.borrow().as_ptr()
            })
        } else {
            std::ptr::null()
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_gamesys_quest_get_quest_clock_value(_quest_index: usize) -> u32 {
    safe_ffi_call(0, || {
        if let Ok(dialog) = get_ui_quest_dialog().lock() {
            dialog.clock_value
        } else {
            0
        }
    })
}