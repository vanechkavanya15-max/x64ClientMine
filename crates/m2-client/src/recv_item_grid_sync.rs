pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl Default for TItemPos {
    fn default() -> Self {
        Self { window_type: 0, cell: 0 }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

impl Default for TPlayerItemAttribute {
    fn default() -> Self {
        Self { bType: 0, sValue: 0 }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Default for TItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u8,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemDel {
    pub header: u8,
    pub pos: TItemPos,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemUse {
    pub header: u8,
    pub Cell: TItemPos,
    pub ch_vid: u32,
    pub victim_vid: u32,
    pub vnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemUpdate {
    pub header: u8,
    pub Cell: TItemPos,
    pub count: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

pub trait AbstractPlayer {
    fn set_item_data(&mut self, item_pos: TItemPos, item_data: TItemData);
    fn set_item_count(&mut self, item_pos: TItemPos, count: u8);
    fn set_item_metin_socket(&mut self, item_pos: TItemPos, socket_index: usize, metin_number: i32);
    fn set_item_attribute(&mut self, item_pos: TItemPos, attr_index: usize, attr_type: u8, attr_value: i16);
}

pub fn recv_item_set_packet(
    packet: &TPacketGCItemSet,
    player: &mut dyn AbstractPlayer,
) -> Result<(), &'static str> {
    let mut item_data = TItemData {
        vnum: packet.vnum,
        count: packet.count,
        flags: packet.flags,
        anti_flags: packet.anti_flags,
        alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
        aAttr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    };

    let mut socks = [0; ITEM_SOCKET_SLOT_MAX_NUM];
    for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
        socks[i] = packet.alSockets[i];
    }
    item_data.alSockets = socks;
    let mut attrs = [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];
    for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
        attrs[i] = packet.aAttr[i];
    }
    item_data.aAttr = attrs;

    let pos = packet.pos;
    player.set_item_data(pos, item_data);

    // Python call for BINARY_Highlight_Item is handled via UI abstractions (omitted from pure logic)
    // RefreshInventoryWindow is also UI update

    Ok(())
}

pub fn recv_item_del_packet(
    packet: &TPacketGCItemDel,
    player: &mut dyn AbstractPlayer,
) -> Result<(), &'static str> {
    let item_data = TItemData::default();
    let pos = packet.pos;
    player.set_item_data(pos, item_data);

    // RefreshInventoryWindow

    Ok(())
}

pub fn recv_item_use_packet(
    _packet: &TPacketGCItemUse,
    _player: &mut dyn AbstractPlayer,
) -> Result<(), &'static str> {
    // C++ logic only calls __RefreshInventoryWindow()
    Ok(())
}

pub fn recv_item_update_packet(
    packet: &TPacketGCItemUpdate,
    player: &mut dyn AbstractPlayer,
) -> Result<(), &'static str> {
    let cell = packet.Cell;
    player.set_item_count(cell, packet.count);
    
    for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
        let cell = packet.Cell;
        let sock = packet.alSockets[i];
        player.set_item_metin_socket(cell, i, sock);
    }
    
    for j in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
        let cell = packet.Cell;
        let b_type = packet.aAttr[j].bType;
        let s_val = packet.aAttr[j].sValue;
        player.set_item_attribute(cell, j, b_type, s_val);
    }

    // RefreshInventoryWindow

    Ok(())
}
