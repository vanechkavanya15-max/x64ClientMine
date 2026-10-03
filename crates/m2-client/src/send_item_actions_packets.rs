// Zaimplementowano wylacznie w celach zaspokojenia interfejsu sieciowego (CG) klienta gry Metin2
// Odpowiada za wysylanie pakietow akcji zwiazanych z przedmiotami.

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl TItemPos {
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self { window_type, cell }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub elk: u32,
}

impl TPacketCGItemDrop {
    pub const HEADER: u16 = 0x0503;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(window_type: u8, cell: u16, elk: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos::new(window_type, cell),
            elk,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.extend_from_slice(&self.elk.to_le_bytes());
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop2 {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub gold: u32,
    pub count: u8,
}

impl TPacketCGItemDrop2 {
    pub const HEADER: u16 = 0x050A;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(window_type: u8, cell: u16, gold: u32, count: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos::new(window_type, cell),
            gold,
            count,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.extend_from_slice(&self.gold.to_le_bytes());
        buf.push(self.count);
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemMove {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub change_pos: TItemPos,
    pub num: u8,
}

impl TPacketCGItemMove {
    pub const HEADER: u16 = 0x0504;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16, num: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos::new(src_win, src_cell),
            change_pos: TItemPos::new(dst_win, dst_cell),
            num,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.push(self.change_pos.window_type);
        buf.extend_from_slice(&self.change_pos.cell.to_le_bytes());
        buf.push(self.num);
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemPickUp {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

impl TPacketCGItemPickUp {
    pub const HEADER: u16 = 0x0502;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(vid: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            vid,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.vid.to_le_bytes());
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemUseToItem {
    pub header: u16,
    pub length: u16,
    pub source_pos: TItemPos,
    pub target_pos: TItemPos,
}

impl TPacketCGItemUseToItem {
    pub const HEADER: u16 = 0x050B;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            source_pos: TItemPos::new(src_win, src_cell),
            target_pos: TItemPos::new(dst_win, dst_cell),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.source_pos.window_type);
        buf.extend_from_slice(&self.source_pos.cell.to_le_bytes());
        buf.push(self.target_pos.window_type);
        buf.extend_from_slice(&self.target_pos.cell.to_le_bytes());
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGGiveItem {
    pub header: u16,
    pub length: u16,
    pub dwTargetVID: u32,
    pub ItemPos: TItemPos,
    pub byItemCount: u8,
}

impl TPacketCGGiveItem {
    pub const HEADER: u16 = 0x0507;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(target_vid: u32, window_type: u8, cell: u16, count: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            dwTargetVID: target_vid,
            ItemPos: TItemPos::new(window_type, cell),
            byItemCount: count,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.dwTargetVID.to_le_bytes());
        buf.push(self.ItemPos.window_type);
        buf.extend_from_slice(&self.ItemPos.cell.to_le_bytes());
        buf.push(self.byItemCount);
        buf
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_drop_action(window_type: u8, cell: u16, elk: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemDrop::new(window_type, cell, elk);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_drop_new_action(window_type: u8, cell: u16, gold: u32, count: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemDrop2::new(window_type, cell, gold, count);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_move_action(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16, count: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemMove::new(src_win, src_cell, dst_win, dst_cell, count);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_pickup_action(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemPickUp::new(vid);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_use_to_item_action(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemUseToItem::new(src_win, src_cell, dst_win, dst_cell);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_give_item_action(target_vid: u32, window_type: u8, cell: u16, count: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGGiveItem::new(target_vid, window_type, cell, count);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}
