#![allow(non_snake_case)]
//! Modul `phase_game_item` — Obsluga pakietow ekwipunku i przedmiotow na ziemi
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseGameItem.cpp`
//! Opis: Odwzorowanie RecvItemSetPacket, RecvItemUsePacket, RecvItemGroundAddPacket, RecvItemGroundDelPacket, RecvItemOwnership.

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u8,
    pub cell: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet2 {
    pub header: u8,
    pub cell: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: bool,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemUse {
    pub header: u8,
    pub cell: TItemPos,
    pub ch_vid: u32,
    pub victim_vid: u32,
    pub vnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemGroundAdd {
    pub header: u8,
    pub lx: i32,
    pub ly: i32,
    pub lz: i32,
    pub dwVID: u32,
    pub dwVnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemGroundDel {
    pub header: u8,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemOwnership {
    pub header: u8,
    pub dwVID: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[derive(Debug, Clone, Copy)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[derive(Debug, Clone, Default)]
pub struct PhaseGameItem {
    pub initialized: bool,
    pub items: std::collections::HashMap<u16, TItemData>,
    pub ground_items: std::collections::HashMap<u32, TPacketGCItemGroundAdd>,
    pub ownerships: std::collections::HashMap<u32, String>,
}

impl PhaseGameItem {
    pub fn new() -> Self {
        Self {
            initialized: true,
            items: std::collections::HashMap::new(),
            ground_items: std::collections::HashMap::new(),
            ownerships: std::collections::HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn recv_item_set_packet(&mut self, packet: &TPacketGCItemSet) -> bool {
        let mut item_data = TItemData {
            vnum: unsafe { std::ptr::addr_of!(packet.vnum).read_unaligned() },
            count: packet.count,
            flags: 0,
            anti_flags: 0,
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute { bType: 0, sValue: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        };

        unsafe {
            item_data.alSockets.copy_from_slice(&std::ptr::addr_of!(packet.alSockets).read_unaligned());
            item_data.aAttr.copy_from_slice(&std::ptr::addr_of!(packet.aAttr).read_unaligned());
        }

        let cell = unsafe { std::ptr::addr_of!(packet.cell).read_unaligned() };
        self.items.insert(unsafe { std::ptr::addr_of!(cell.cell).read_unaligned() }, item_data);

        true
    }

    pub fn recv_item_set_packet2(&mut self, packet: &TPacketGCItemSet2) -> bool {
        let mut item_data = TItemData {
            vnum: unsafe { std::ptr::addr_of!(packet.vnum).read_unaligned() },
            count: packet.count,
            flags: unsafe { std::ptr::addr_of!(packet.flags).read_unaligned() },
            anti_flags: unsafe { std::ptr::addr_of!(packet.anti_flags).read_unaligned() },
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute { bType: 0, sValue: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        };

        unsafe {
            item_data.alSockets.copy_from_slice(&std::ptr::addr_of!(packet.alSockets).read_unaligned());
            item_data.aAttr.copy_from_slice(&std::ptr::addr_of!(packet.aAttr).read_unaligned());
        }

        let cell = unsafe { std::ptr::addr_of!(packet.cell).read_unaligned() };
        self.items.insert(unsafe { std::ptr::addr_of!(cell.cell).read_unaligned() }, item_data);

        true
    }

    pub fn recv_item_use_packet(&mut self, _packet: &TPacketGCItemUse) -> bool {
        // Implementacja odswiezania okna ekwipunku i obslugi uzycia
        true
    }

    pub fn recv_item_ground_add_packet(&mut self, packet: &TPacketGCItemGroundAdd) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        self.ground_items.insert(vid, *packet);
        true
    }

    pub fn recv_item_ownership(&mut self, packet: &TPacketGCItemOwnership) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        
        // Wyodrebnianie null-terminated string z bufora o stalym rozmiarze
        let mut name_len = 0;
        let sz_name = unsafe { std::ptr::addr_of!(packet.szName).read_unaligned() };
        while name_len < sz_name.len() && sz_name[name_len] != 0 {
            name_len += 1;
        }
        let name = String::from_utf8_lossy(&sz_name[..name_len]).into_owned();

        self.ownerships.insert(vid, name);
        true
    }

    pub fn recv_item_ground_del_packet(&mut self, packet: &TPacketGCItemGroundDel) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        self.ground_items.remove(&vid);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_phase_game_item() {
        let instance = PhaseGameItem::new();
        assert!(instance.is_ready());
    }
}
