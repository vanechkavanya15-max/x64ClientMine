#![allow(non_snake_case)]

use std::mem::size_of;
use std::ptr;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, ItemData};

pub const HEADER_GC_ITEM_SET: u16 = 0x0511;
pub const HEADER_GC_ITEM_DEL: u16 = 0x0510;
pub const HEADER_GC_ITEM_UPDATE: u16 = 0x0514;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    BufferTooSmall,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub sockets: [i32; 3],
    pub attrs: [TPlayerItemAttribute; 7],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemDel {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemUpdate {
    pub header: u16,
    pub pos: TItemPos,
    pub count: u8,
    pub sockets: [i32; 3],
    pub attrs: [TPlayerItemAttribute; 7],
}

impl TPacketGCItemSet {
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < size_of::<Self>() {
            return Err(PacketError::BufferTooSmall);
        }
        
        let ptr = data.as_ptr() as *const Self;
        let packet = unsafe { ptr::read_unaligned(ptr) };
        
        if packet.header != HEADER_GC_ITEM_SET {
            return Err(PacketError::InvalidHeader);
        }
        
        Ok(packet)
    }
}

impl TPacketGCItemDel {
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < size_of::<Self>() {
            return Err(PacketError::BufferTooSmall);
        }
        
        let ptr = data.as_ptr() as *const Self;
        let packet = unsafe { ptr::read_unaligned(ptr) };
        
        if packet.header != HEADER_GC_ITEM_DEL {
            return Err(PacketError::InvalidHeader);
        }
        
        Ok(packet)
    }
}

impl TPacketGCItemUpdate {
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < size_of::<Self>() {
            return Err(PacketError::BufferTooSmall);
        }
        
        let ptr = data.as_ptr() as *const Self;
        let packet = unsafe { ptr::read_unaligned(ptr) };
        
        if packet.header != HEADER_GC_ITEM_UPDATE {
            return Err(PacketError::InvalidHeader);
        }
        
        Ok(packet)
    }
}

pub fn handle_item_set(data: &[u8]) -> Result<(), PacketError> {
    let packet = TPacketGCItemSet::from_bytes(data)?;
    
    let window_type = packet.pos.window_type;
    let cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };
    let vnum = unsafe { ptr::addr_of!(packet.vnum).read_unaligned() };
    let count = packet.count;
    let flags = unsafe { ptr::addr_of!(packet.flags).read_unaligned() };
    let anti_flags = unsafe { ptr::addr_of!(packet.anti_flags).read_unaligned() };
    let raw_sockets = unsafe { ptr::addr_of!(packet.sockets).read_unaligned() };
    let raw_attrs = unsafe { ptr::addr_of!(packet.attrs).read_unaligned() };

    let mut attrs = [(0u8, 0i16); 7];
    for (i, attr) in raw_attrs.iter().enumerate() {
        attrs[i] = (attr.bType, unsafe { ptr::addr_of!(attr.sValue).read_unaligned() });
    }

    let item_data = ItemData {
        vnum,
        count,
        flags,
        anti_flags,
        sockets: raw_sockets,
        attrs,
    };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.set_item(window_type, cell, item_data);
    }

    Ok(())
}

pub fn handle_item_del(data: &[u8]) -> Result<(), PacketError> {
    let packet = TPacketGCItemDel::from_bytes(data)?;
    
    let window_type = packet.pos.window_type;
    let cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.del_item(window_type, cell);
    }

    Ok(())
}

pub fn handle_item_update(data: &[u8]) -> Result<(), PacketError> {
    let packet = TPacketGCItemUpdate::from_bytes(data)?;
    
    let window_type = packet.pos.window_type;
    let cell = unsafe { ptr::addr_of!(packet.pos.cell).read_unaligned() };
    let count = packet.count;
    let raw_sockets = unsafe { ptr::addr_of!(packet.sockets).read_unaligned() };
    let raw_attrs = unsafe { ptr::addr_of!(packet.attrs).read_unaligned() };
    
    let mut attrs = [(0u8, 0i16); 7];
    for (i, attr) in raw_attrs.iter().enumerate() {
        attrs[i] = (attr.bType, unsafe { ptr::addr_of!(attr.sValue).read_unaligned() });
    }

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        if let Some(item) = state.player.inventory.get_mut(&(window_type, cell)) {
            item.count = count;
            item.sockets = raw_sockets;
            item.attrs = attrs;
        }
    }

    Ok(())
}
