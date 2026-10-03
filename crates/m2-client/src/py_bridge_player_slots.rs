use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;

pub const INVENTORY_PAGE_SIZE: u16 = 5 * 9;
pub const INVENTORY_PAGE_COUNT: u16 = 2;
pub const ITEM_SLOT_COUNT: u16 = INVENTORY_PAGE_SIZE * INVENTORY_PAGE_COUNT;
pub const EQUIPMENT_COUNT: u16 = 12;

pub const EQUIPMENT_START: u16 = ITEM_SLOT_COUNT;
pub const WEAR_MAX: u16 = 32;

pub const DRAGON_SOUL_EQUIP_START: u16 = ITEM_SLOT_COUNT + WEAR_MAX;
pub const DRAGON_SOUL_EQUIP_SLOT_MAX: u16 = 6;
pub const DS_DECK_MAX_NUM: u16 = 2;
pub const DRAGON_SOUL_EQUIP_END: u16 = DRAGON_SOUL_EQUIP_START + DRAGON_SOUL_EQUIP_SLOT_MAX * DS_DECK_MAX_NUM;

pub const DRAGON_SOUL_EQUIP_RESERVED_COUNT: u16 = DRAGON_SOUL_EQUIP_SLOT_MAX * 3;

pub const BELT_INVENTORY_SLOT_START: u16 = DRAGON_SOUL_EQUIP_END + DRAGON_SOUL_EQUIP_RESERVED_COUNT;
pub const BELT_INVENTORY_WIDTH: u16 = 4;
pub const BELT_INVENTORY_HEIGHT: u16 = 4;
pub const BELT_INVENTORY_SLOT_COUNT: u16 = BELT_INVENTORY_WIDTH * BELT_INVENTORY_HEIGHT;
pub const BELT_INVENTORY_SLOT_END: u16 = BELT_INVENTORY_SLOT_START + BELT_INVENTORY_SLOT_COUNT;
pub const INVENTORY_COUNT: u16 = BELT_INVENTORY_SLOT_END;

pub const WEAR_COSTUME_BODY: u16 = 19;
pub const COSTUME_SLOT_START: u16 = EQUIPMENT_START + WEAR_COSTUME_BODY;
pub const COSTUME_NUM_TYPES: u16 = 2;
pub const COSTUME_SLOT_END: u16 = COSTUME_SLOT_START + COSTUME_NUM_TYPES;

pub const WEAR_BELT: u16 = 23;
pub const EQUIPMENT_BELT: u16 = EQUIPMENT_START + WEAR_BELT;

pub const WINDOW_RESERVED: u8 = 0;
pub const WINDOW_INVENTORY: u8 = 1;
pub const WINDOW_EQUIPMENT: u8 = 2;
pub const WINDOW_SAFEBOX: u8 = 3;
pub const WINDOW_MALL: u8 = 4;
pub const WINDOW_DRAGON_SOUL_INVENTORY: u8 = 5;
pub const WINDOW_GROUND: u8 = 6;
pub const WINDOW_BELT_INVENTORY: u8 = 7;
pub const WINDOW_MAX: u8 = 8;

pub const SLOT_TYPE_NONE: u8 = 0;
pub const SLOT_TYPE_INVENTORY: u8 = 1;
pub const SLOT_TYPE_SKILL: u8 = 2;
pub const SLOT_TYPE_EMOTION: u8 = 3;
pub const SLOT_TYPE_SHOP: u8 = 4;
pub const SLOT_TYPE_EXCHANGE_OWNER: u8 = 5;
pub const SLOT_TYPE_EXCHANGE_TARGET: u8 = 6;
pub const SLOT_TYPE_QUICK_SLOT: u8 = 7;
pub const SLOT_TYPE_SAFEBOX: u8 = 8;
pub const SLOT_TYPE_PRIVATE_SHOP: u8 = 9;
pub const SLOT_TYPE_MALL: u8 = 10;
pub const SLOT_TYPE_DRAGON_SOUL_INVENTORY: u8 = 11;
pub const SLOT_TYPE_MAX: u8 = 12;

pub const SLOT_TYPE_TO_INVEN_TYPE: [u8; SLOT_TYPE_MAX as usize] = [
    WINDOW_RESERVED,                // SLOT_TYPE_NONE
    WINDOW_INVENTORY,               // SLOT_TYPE_INVENTORY
    WINDOW_RESERVED,                // SLOT_TYPE_SKILL
    WINDOW_RESERVED,                // SLOT_TYPE_EMOTION
    WINDOW_RESERVED,                // SLOT_TYPE_SHOP
    WINDOW_RESERVED,                // SLOT_TYPE_EXCHANGE_OWNER
    WINDOW_RESERVED,                // SLOT_TYPE_EXCHANGE_TARGET
    WINDOW_RESERVED,                // SLOT_TYPE_QUICK_SLOT
    WINDOW_RESERVED,                // SLOT_TYPE_SAFEBOX
    WINDOW_RESERVED,                // SLOT_TYPE_PRIVATE_SHOP
    WINDOW_RESERVED,                // SLOT_TYPE_MALL
    WINDOW_DRAGON_SOUL_INVENTORY,   // SLOT_TYPE_DRAGON_SOUL_INVENTORY
];

pub fn slot_type_to_inven_type(slot_type: u8) -> u8 {
    if slot_type >= SLOT_TYPE_MAX {
        WINDOW_RESERVED
    } else {
        SLOT_TYPE_TO_INVEN_TYPE[slot_type as usize]
    }
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_equipment_slot(slot_index: i32) -> i32 {
    safe_ffi_call(0, || {
        if slot_index >= EQUIPMENT_START as i32 && slot_index <= DRAGON_SOUL_EQUIP_END as i32 {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_costume_slot(slot_index: i32) -> i32 {
    safe_ffi_call(0, || {
        if slot_index >= COSTUME_SLOT_START as i32 && slot_index <= COSTUME_SLOT_END as i32 {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_belt_inventory_slot(slot_index: i32) -> i32 {
    safe_ffi_call(0, || {
        if slot_index >= BELT_INVENTORY_SLOT_START as i32 && slot_index < BELT_INVENTORY_SLOT_END as i32 {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_is_ds_equipment_slot(window_type: u8, slot_index: i32) -> i32 {
    safe_ffi_call(0, || {
        if window_type == WINDOW_INVENTORY {
            if slot_index >= DRAGON_SOUL_EQUIP_START as i32 && slot_index <= DRAGON_SOUL_EQUIP_END as i32 {
                return 1;
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_slot_type_to_inven_type(slot_type: u8) -> i32 {
    safe_ffi_call(0, || {
        slot_type_to_inven_type(slot_type) as i32
    })
}

pub const AVAILABLE_RULE_BY_GRADE: [u8; BELT_INVENTORY_SLOT_COUNT as usize] = [
    1, 2, 4, 6,
    3, 3, 4, 6,
    5, 5, 5, 6,
    7, 7, 7, 7,
];

#[no_mangle]
pub extern "C" fn m2_py_player_is_available_belt_inventory_cell(pos: i32) -> bool {
    safe_ffi_call(false, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        // In python player module, it queries TItemPos(EQUIPMENT, c_Equipment_Belt)
        // Equipment is represented by window 2 in state? But in TItemPos, EQUIPMENT = 2, INVENTORY = 1.
        // Wait, looking at get_item_count in existing bridge:
        // CPythonPlayer::IsBeltInventorySlot checks Cell.IsBeltInventoryCell().
        // TItemPos has INVENTORY = 1, EQUIPMENT = 2.
        
        if let Some(item) = state.get_item(WINDOW_EQUIPMENT, EQUIPMENT_BELT) {
            if item.count > 0 {
                let vnum = item.vnum;
                // Get belt grade from item.
                let mut belt_grade = 0;
                
                // Get item from ItemManager
                let mgr = m2_client::py_bridge_item_nonplayer_runtime::get_item_manager_runtime().lock().unwrap();
                if let Some(item_proto) = mgr.items.get(&vnum) {
                    belt_grade = item_proto.alValues[0];
                }
                
                let cell = pos - BELT_INVENTORY_SLOT_START as i32;
                if cell >= 0 && cell < BELT_INVENTORY_SLOT_COUNT as i32 {
                    return AVAILABLE_RULE_BY_GRADE[cell as usize] <= belt_grade as u8;
                }
            }
        }
        
        false
    })
}
