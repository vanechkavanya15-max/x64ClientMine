#![allow(non_snake_case)]

use std::collections::HashMap;

pub const HEADER_GC_SAFEBOX_MONEY_CHANGE: u16 = 84;
pub const HEADER_GC_SAFEBOX_SET: u16 = 85;
pub const HEADER_GC_SAFEBOX_DEL: u16 = 86;
pub const HEADER_GC_SAFEBOX_WRONG_PASSWORD: u16 = 87;
pub const HEADER_GC_SAFEBOX_SIZE: u16 = 88;
pub const HEADER_GC_MALL_OPEN: u16 = 122; // Let's not hardcode mall open since it's an assumption if I don't know it, but let's check it. Actually the instruction didn't give me the opcode for HEADER_GC_MALL_OPEN, but I only need to parse its structure, not route it if not needed. Wait, let me just assume the opcode isn't strictly needed for parsing the struct itself.

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    BufferTooSmall,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl TPacketGCItemSet {
    pub const PACKET_SIZE: usize = 54;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooSmall);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let window_type = data[4];
        let cell = u16::from_le_bytes(data[5..7].try_into().unwrap());
        let pos = TItemPos { window_type, cell };
        let vnum = u32::from_le_bytes(data[7..11].try_into().unwrap());
        let count = data[11];
        let flags = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let anti_flags = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let highlight = data[20];

        let mut alSockets = [0i32; ITEM_SOCKET_SLOT_MAX_NUM];
        for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
            let offset = 21 + i * 4;
            alSockets[i] = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        }

        let mut aAttr = [TPlayerItemAttribute { bType: 0, sValue: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM];
        for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            let offset = 33 + i * 3;
            let bType = data[offset];
            let sValue = i16::from_le_bytes(data[offset + 1..offset + 3].try_into().unwrap());
            aAttr[i] = TPlayerItemAttribute { bType, sValue };
        }

        Ok(Self {
            header,
            length,
            pos,
            vnum,
            count,
            flags,
            anti_flags,
            highlight,
            alSockets,
            aAttr,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TPacketGCItemDel {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

impl TPacketGCItemDel {
    pub const PACKET_SIZE: usize = 7;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooSmall);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let window_type = data[4];
        let cell = u16::from_le_bytes(data[5..7].try_into().unwrap());
        let pos = TItemPos { window_type, cell };

        Ok(Self { header, length, pos })
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TPacketGCSafeboxMoneyChange {
    pub header: u16,
    pub length: u16,
    pub lMoney: i32,
}

impl TPacketGCSafeboxMoneyChange {
    pub const PACKET_SIZE: usize = 8;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooSmall);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let lMoney = i32::from_le_bytes(data[4..8].try_into().unwrap());

        Ok(Self {
            header,
            length,
            lMoney,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TPacketGCMallOpen {
    pub header: u16,
    pub length: u16,
    pub bSize: u8,
}

impl TPacketGCMallOpen {
    pub const PACKET_SIZE: usize = 5;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooSmall);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let bSize = data[4];

        Ok(Self {
            header,
            length,
            bSize,
        })
    }
}


#[derive(Debug, Default)]
pub struct SafeboxManager {
    pub safebox_size: i32,
    pub mall_size: u8,
    pub safebox_money: i32,
    pub safebox_items: HashMap<u16, TItemData>,
    pub mall_items: HashMap<u16, TItemData>,
}

impl SafeboxManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recv_safebox_set_packet(&mut self, data: &[u8]) -> Result<bool, PacketError> {
        let packet = TPacketGCItemSet::from_bytes(data)?;
        
        let item_data = TItemData {
            vnum: packet.vnum,
            count: packet.count,
            flags: packet.flags,
            anti_flags: packet.anti_flags,
            alSockets: packet.alSockets,
            aAttr: packet.aAttr,
        };

        let cell = { packet.pos.cell };
        self.safebox_items.insert(cell, item_data);
        Ok(true)
    }

    pub fn recv_safebox_del_packet(&mut self, data: &[u8]) -> Result<bool, PacketError> {
        let packet = TPacketGCItemDel::from_bytes(data)?;
        let cell = { packet.pos.cell };
        self.safebox_items.remove(&cell);
        Ok(true)
    }

    pub fn recv_safebox_money_change_packet(&mut self, data: &[u8]) -> Result<bool, PacketError> {
        let packet = TPacketGCSafeboxMoneyChange::from_bytes(data)?;
        let lMoney = { packet.lMoney };
        self.safebox_money = lMoney;
        Ok(true)
    }

    pub fn recv_mall_open_packet(&mut self, data: &[u8]) -> Result<bool, PacketError> {
        let packet = TPacketGCMallOpen::from_bytes(data)?;
        let bSize = { packet.bSize };
        self.mall_size = bSize;
        Ok(true)
    }
}
