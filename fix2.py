with open("crates/m2-client/src/game_item_refine_socket_kd_manager.rs", "w") as f:
    f.write("""use crate::inventory_grid_v2::{get_inventory_sys, WindowType};
use crate::py_bridge_item_native::get_item_native_manager;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::py_bridge_player_items::{
    ATTACH_METIN_CANT, ATTACH_METIN_OK, ATTACH_METIN_NOT_MATCHABLE_ITEM,
    ATTACH_METIN_NO_MATCHABLE_SOCKET, ATTACH_METIN_NOT_EXIST_GOLD_SOCKET,
    ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT, ITEM_TYPE_ROD, DETACH_METIN_CANT, DETACH_METIN_OK,
    ITEM_TYPE_USE, USE_DETACHMENT, ITEM_FLAG_REFINEABLE
};

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const METIN_SOCKET_COUNT: usize = 6;
pub const METIN_SOCKET_TYPE_NONE: i32 = 0;
pub const METIN_SOCKET_TYPE_SILVER: i32 = 1;
pub const METIN_SOCKET_TYPE_GOLD: i32 = 2;

pub const METIN_NORMAL: u8 = 0;
pub const METIN_GOLD: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachMetinError {
    Cant,
    NotMatchableItem,
    NoMatchableSocket,
    NotExistGoldSocket,
    CantAttachToEquipment,
    Ok,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetachMetinError {
    Cant,
    Ok,
}

pub struct ItemMetinSocketManager {
    pub sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
}

impl ItemMetinSocketManager {
    pub fn new() -> Self {
        Self {
            sockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
        }
    }
    
    pub fn get_item_metin_socket(&self, index: usize) -> i32 {
        if index >= ITEM_SOCKET_SLOT_MAX_NUM {
            return 0;
        }
        self.sockets[index]
    }
    
    pub fn set_item_metin_socket(&mut self, index: usize, value: i32) {
        if index >= ITEM_SOCKET_SLOT_MAX_NUM {
            return;
        }
        self.sockets[index] = value;
    }
    
    pub fn find_item_slot(&self, socket_value: i32) -> Option<usize> {
        for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
            if self.sockets[i] == socket_value {
                return Some(i);
            }
        }
        None
    }
    
    pub fn find_empty_socket(&self) -> Option<usize> {
        self.find_item_slot(METIN_SOCKET_TYPE_NONE)
    }

    pub fn find_empty_socket_silver_or_gold(&self) -> Option<usize> {
        if let Some(pos) = self.find_item_slot(METIN_SOCKET_TYPE_SILVER) {
            return Some(pos);
        }
        if let Some(pos) = self.find_item_slot(METIN_SOCKET_TYPE_GOLD) {
            return Some(pos);
        }
        None
    }
    
    pub fn find_empty_socket_gold(&self) -> Option<usize> {
        self.find_item_slot(METIN_SOCKET_TYPE_GOLD)
    }
    
    pub fn can_attach_metin(&self, is_equipment_slot: bool, is_rod: bool, metin_wear_flags: u32, target_wear_flags: u32, metin_sub_type: u8, metin_normal: u8, metin_gold: u8) -> AttachMetinError {
        if is_equipment_slot {
            return AttachMetinError::CantAttachToEquipment;
        }
        
        if (metin_wear_flags & target_wear_flags) == 0 {
            return AttachMetinError::NotMatchableItem;
        }
        
        if is_rod {
            return AttachMetinError::Cant;
        }
        
        let mut not_exist_gold_socket = false;
        
        for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
            let socket_type = self.get_item_metin_socket(i);
            
            if metin_sub_type == metin_normal {
                if socket_type == METIN_SOCKET_TYPE_SILVER || socket_type == METIN_SOCKET_TYPE_GOLD {
                    return AttachMetinError::Ok;
                }
            } else if metin_sub_type == metin_gold {
                if socket_type == METIN_SOCKET_TYPE_GOLD {
                    return AttachMetinError::Ok;
                } else if socket_type == METIN_SOCKET_TYPE_SILVER {
                    not_exist_gold_socket = true;
                }
            }
        }
        
        if not_exist_gold_socket {
            return AttachMetinError::NotExistGoldSocket;
        }
        
        AttachMetinError::NoMatchableSocket
    }
    
    pub fn can_detach(&self, is_use_type: bool, is_detachment_subtype: bool, is_refineable: bool) -> DetachMetinError {
        if !is_use_type {
            return DetachMetinError::Cant;
        }
        if !is_detachment_subtype {
            return DetachMetinError::Cant;
        }
        
        if is_refineable {
            for i in 0..METIN_SOCKET_COUNT {
                if i < ITEM_SOCKET_SLOT_MAX_NUM && self.get_item_metin_socket(i) > 2 {
                    return DetachMetinError::Ok;
                }
            }
        }
        
        DetachMetinError::Cant
    }
}

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

        let mut sock_mgr = ItemMetinSocketManager::new();
        sock_mgr.sockets = target_sockets.map(|s| s as i32);
        
        match sock_mgr.can_attach_metin(
            false,
            target_type == ITEM_TYPE_ROD,
            metin_wear_flags,
            target_wear_flags,
            metin_sub_type,
            METIN_NORMAL,
            METIN_GOLD
        ) {
            AttachMetinError::Cant => ATTACH_METIN_CANT,
            AttachMetinError::CantAttachToEquipment => ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT,
            AttachMetinError::NotMatchableItem => ATTACH_METIN_NOT_MATCHABLE_ITEM,
            AttachMetinError::NoMatchableSocket => ATTACH_METIN_NO_MATCHABLE_SOCKET,
            AttachMetinError::NotExistGoldSocket => ATTACH_METIN_NOT_EXIST_GOLD_SOCKET,
            AttachMetinError::Ok => ATTACH_METIN_OK,
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_player_can_detach(scroll_vnum: u32, target_w: u8, target_c: u16) -> i32 {
    safe_ffi_call(DETACH_METIN_CANT, || {
        let twin = match WindowType::from_u8(target_w) {
            Some(w) => w,
            None => return DETACH_METIN_CANT,
        };

        let mut mgr = get_item_native_manager().lock().unwrap();
        
        mgr.selected_vnum = scroll_vnum;
        let scroll_type;
        let scroll_sub_type;
        if let Some(scroll) = mgr.get_selected_item() {
            scroll_type = scroll.bType;
            scroll_sub_type = scroll.bSubType;
        } else {
            return DETACH_METIN_CANT;
        }
        
        let mut target_vnum = 0;
        let mut target_sockets = [0; 3];
        if let Ok(sys) = get_inventory_sys().lock() {
            if let Some(item) = sys.get_item(twin, target_c) {
                target_vnum = item.vnum;
                target_sockets = item.sockets;
            } else {
                return DETACH_METIN_CANT;
            }
        }
        
        mgr.selected_vnum = target_vnum;
        let target_flags;
        if let Some(target) = mgr.get_selected_item() {
            target_flags = target.dwFlags;
        } else {
            return DETACH_METIN_CANT;
        }
        
        let mut sock_mgr = ItemMetinSocketManager::new();
        sock_mgr.sockets = target_sockets.map(|s| s as i32);
        
        match sock_mgr.can_detach(
            scroll_type == ITEM_TYPE_USE,
            scroll_sub_type == USE_DETACHMENT,
            (target_flags & ITEM_FLAG_REFINEABLE) != 0
        ) {
            DetachMetinError::Cant => DETACH_METIN_CANT,
            DetachMetinError::Ok => DETACH_METIN_OK,
        }
    })
}
""")
