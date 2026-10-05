use crate::inventory_grid_v2::{get_inventory_sys, WindowType};
use crate::py_bridge_item_native::get_item_native_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::py_bridge_player_items::{
    ATTACH_METIN_CANT, ATTACH_METIN_OK, ATTACH_METIN_NOT_MATCHABLE_ITEM,
    ATTACH_METIN_NO_MATCHABLE_SOCKET, ATTACH_METIN_NOT_EXIST_GOLD_SOCKET,
    ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT, ITEM_TYPE_ROD
};

pub const METIN_SOCKET_TYPE_NONE: i32 = 0;
pub const METIN_SOCKET_TYPE_SILVER: i32 = 1;
pub const METIN_SOCKET_TYPE_GOLD: i32 = 2;

pub const METIN_NORMAL: u8 = 0;
pub const METIN_GOLD: u8 = 1;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;

#[no_mangle]
pub extern "C" fn m2_py_player_can_attach_metin(metin_vnum: u32, target_w: u8, target_c: u16) -> i32 {
    safe_ffi_call(ATTACH_METIN_CANT, || {
        let twin = match WindowType::from_u8(target_w) {
            Some(w) => w,
            None => return ATTACH_METIN_CANT,
        };

        if twin == WindowType::EQUIPMENT {
            return ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT;
        }

        let mut mgr = get_item_native_manager().lock().unwrap();

        mgr.selected_vnum = metin_vnum;
        let metin_wear_flags;
        let metin_sub_type;
        if let Some(metin) = mgr.get_selected_item() {
            metin_wear_flags = metin.dwWearFlags;
            metin_sub_type = metin.bSubType;
        } else {
            return ATTACH_METIN_CANT;
        }

        let mut target_vnum = 0;
        let mut target_sockets = [0; 3];
        if let Ok(sys) = get_inventory_sys().lock() {
            if let Some(item) = sys.get_item(twin, target_c) {
                target_vnum = item.vnum;
                target_sockets = item.sockets;
            } else {
                return ATTACH_METIN_CANT;
            }
        }

        mgr.selected_vnum = target_vnum;
        let target_wear_flags;
        let target_type;
        if let Some(target) = mgr.get_selected_item() {
            target_wear_flags = target.dwWearFlags;
            target_type = target.bType;
        } else {
            return ATTACH_METIN_CANT;
        }

        if (metin_wear_flags & target_wear_flags) == 0 {
            return ATTACH_METIN_NOT_MATCHABLE_ITEM;
        }

        if target_type == ITEM_TYPE_ROD {
            return ATTACH_METIN_CANT;
        }

        let mut b_not_exist_gold_socket = false;

        for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
            let dw_socket_type = if i >= 3 {
                0
            } else {
                target_sockets[i]
            };

            if metin_sub_type == METIN_NORMAL {
                if dw_socket_type == METIN_SOCKET_TYPE_SILVER || dw_socket_type == METIN_SOCKET_TYPE_GOLD {
                    return ATTACH_METIN_OK;
                }
            } else if metin_sub_type == METIN_GOLD {
                if dw_socket_type == METIN_SOCKET_TYPE_GOLD {
                    return ATTACH_METIN_OK;
                } else if dw_socket_type == METIN_SOCKET_TYPE_SILVER {
                    b_not_exist_gold_socket = true;
                }
            }
        }

        if b_not_exist_gold_socket {
            return ATTACH_METIN_NOT_EXIST_GOLD_SOCKET;
        }

        ATTACH_METIN_NO_MATCHABLE_SOCKET
    })
}
