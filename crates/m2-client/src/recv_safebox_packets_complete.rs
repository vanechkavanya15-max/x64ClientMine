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

// C-FFI callbacks that will be implemented in C++ (mocked in tests)
extern "C" {
    pub fn m2_safebox_open(size: u8);
    pub fn m2_safebox_set_item(
        pos: u16, // using cell only, assuming window_type is SAFEBOX implicitly handled by UI or we pass full struct
        vnum: u32,
        count: u8,
        flags: u32,
        anti_flags: u32,
        sockets: *const i32,
        attrs: *const TPlayerItemAttribute,
    );
    pub fn m2_safebox_del_item(pos: u16);
    pub fn m2_safebox_wrong_password();
    pub fn m2_safebox_set_money(money: i32);
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
                m2_safebox_open(packet.bSize);
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
                m2_safebox_open(packet.bSize);
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
