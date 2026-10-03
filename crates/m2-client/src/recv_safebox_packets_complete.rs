#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use std::mem::size_of;
use std::ptr;
use std::slice;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: SItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TPacketGCItemDel {
    pub header: u16,
    pub length: u16,
    pub pos: SItemPos,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TPacketGCSafeboxWrongPassword {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TPacketGCSafeboxSize {
    pub header: u16,
    pub length: u16,
    pub bSize: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TPacketGCSafeboxMoneyChange {
    pub header: u16,
    pub length: u16,
    pub lMoney: i32,
}

use crate::safebox_mall_manager_v2::{G_SAFEBOX_MALL_MANAGER, TItemData, TPlayerItemAttribute as SafeBoxAttr};

#[no_mangle]
pub extern "C" fn m2_safebox_recv_open(size: u8) {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        mgr.open_safebox(size as usize);
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "OpenSafeboxWindow", size as i32);
    }
}

#[no_mangle]
pub extern "C" fn m2_safebox_set_item(
    pos: u16,
    vnum: u32,
    count: u8,
    flags: u32,
    anti_flags: u32,
    sockets: *const i32,
    attrs: *const TPlayerItemAttribute,
) {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        let mut item = TItemData {
            vnum,
            count,
            flags,
            anti_flags,
            alSockets: [0; 3],
            aAttr: [SafeBoxAttr::default(); 7],
        };
        if !sockets.is_null() {
            unsafe {
                for i in 0..3 {
                    item.alSockets[i] = *sockets.add(i);
                }
            }
        }
        if !attrs.is_null() {
            unsafe {
                for i in 0..7 {
                    let a = &*attrs.add(i);
                    item.aAttr[i] = SafeBoxAttr {
                        bType: a.bType,
                        sValue: a.sValue,
                    };
                }
            }
        }
        mgr.set_item_data(pos as usize, item);
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method0(game_wnd, "RefreshSafebox");
    }
}

#[no_mangle]
pub extern "C" fn m2_safebox_del_item(pos: u16) {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        mgr.del_item_data(pos as usize);
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method0(game_wnd, "RefreshSafebox");
    }
}

#[no_mangle]
pub extern "C" fn m2_safebox_wrong_password() {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        mgr.try_open_with_password("");
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method0(game_wnd, "OnSafeBoxError");
    }
}

#[no_mangle]
pub extern "C" fn m2_safebox_set_money(money: i32) {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        mgr.set_money(money.max(0) as u32);
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method0(game_wnd, "RefreshSafeboxMoney");
    }
}

pub const GC_SAFEBOX_OPEN: u16 = 0x0830;
pub const GC_SAFEBOX_SET: u16 = 0x0831;
pub const GC_SAFEBOX_DEL: u16 = 0x0832;
pub const GC_SAFEBOX_WRONG_PASSWORD: u16 = 0x0833;
pub const GC_SAFEBOX_SIZE: u16 = 0x0833;
pub const GC_SAFEBOX_MONEY_CHANGE: u16 = 0x0834;

pub fn parse_safebox_packet(data: &[u8]) -> Result<usize, &'static str> {
    if data.len() < 2 {
        return Err("Packet too short for header");
    }

    let header = u16::from_le_bytes([data[0], data[1]]);

    match header {
        GC_SAFEBOX_OPEN => {
            if data.len() < size_of::<TPacketGCSafeboxSize>() {
                return Err("Packet too short for GC_SAFEBOX_OPEN");
            }
            let packet = unsafe { &*(data.as_ptr() as *const TPacketGCSafeboxSize) };
            
            unsafe {
                m2_safebox_recv_open(packet.bSize);
            }
            Ok(size_of::<TPacketGCSafeboxSize>())
        }
        GC_SAFEBOX_SET => {
            if data.len() < size_of::<TPacketGCItemSet>() {
                return Err("Packet too short for GC_SAFEBOX_SET");
            }
            let packet = unsafe { &*(data.as_ptr() as *const TPacketGCItemSet) };
            
            unsafe {
                m2_safebox_set_item(
                    packet.pos.cell,
                    packet.vnum,
                    packet.count,
                    packet.flags,
                    packet.anti_flags,
                    ptr::addr_of!(packet.alSockets) as *const i32,
                    ptr::addr_of!(packet.aAttr) as *const TPlayerItemAttribute,
                );
            }
            Ok(size_of::<TPacketGCItemSet>())
        }
        GC_SAFEBOX_DEL => {
            if data.len() < size_of::<TPacketGCItemDel>() {
                return Err("Packet too short for GC_SAFEBOX_DEL");
            }
            let packet = unsafe { &*(data.as_ptr() as *const TPacketGCItemDel) };
            
            unsafe {
                m2_safebox_del_item(packet.pos.cell);
            }
            Ok(size_of::<TPacketGCItemDel>())
        }
        GC_SAFEBOX_WRONG_PASSWORD => {
            if data.len() < size_of::<TPacketGCSafeboxWrongPassword>() {
                return Err("Packet too short for GC_SAFEBOX_WRONG_PASSWORD");
            }
            
            unsafe {
                m2_safebox_wrong_password();
            }
            Ok(size_of::<TPacketGCSafeboxWrongPassword>())
        }
        GC_SAFEBOX_SIZE => {
            if data.len() < size_of::<TPacketGCSafeboxSize>() {
                return Err("Packet too short for GC_SAFEBOX_SIZE");
            }
            let packet = unsafe { &*(data.as_ptr() as *const TPacketGCSafeboxSize) };
            
            unsafe {
                m2_safebox_recv_open(packet.bSize);
            }
            Ok(size_of::<TPacketGCSafeboxSize>())
        }
        GC_SAFEBOX_MONEY_CHANGE => {
            if data.len() < size_of::<TPacketGCSafeboxMoneyChange>() {
                return Err("Packet too short for GC_SAFEBOX_MONEY_CHANGE");
            }
            let packet = unsafe { &*(data.as_ptr() as *const TPacketGCSafeboxMoneyChange) };
            
            unsafe {
                m2_safebox_set_money(packet.lMoney);
            }
            Ok(size_of::<TPacketGCSafeboxMoneyChange>())
        }
        _ => Err("Unknown safebox packet header"),
    }
}
