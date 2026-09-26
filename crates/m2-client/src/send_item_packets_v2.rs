use std::mem::size_of;
use std::ptr;

// Header constants
pub const HEADER_CG_ITEM_USE: u16 = 0x0501;
pub const HEADER_CG_ITEM_DROP: u16 = 0x0502;
pub const HEADER_CG_ITEM_DROP2: u16 = 0x0503;
pub const HEADER_CG_ITEM_MOVE: u16 = 0x0504;
pub const HEADER_CG_ITEM_PICKUP: u16 = 0x0505;

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
pub struct TPacketCGItemUse {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

impl TPacketCGItemUse {
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self {
            header: HEADER_CG_ITEM_USE,
            length: size_of::<Self>() as u16,
            pos: TItemPos::new(window_type, cell),
        }
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
    pub fn new(src_w: u8, src_c: u16, dst_w: u8, dst_c: u16, num: u8) -> Self {
        Self {
            header: HEADER_CG_ITEM_MOVE,
            length: size_of::<Self>() as u16,
            pos: TItemPos::new(src_w, src_c),
            change_pos: TItemPos::new(dst_w, dst_c),
            num,
        }
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
    pub fn new(vid: u32) -> Self {
        Self {
            header: HEADER_CG_ITEM_PICKUP,
            length: size_of::<Self>() as u16,
            vid,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub gold: u64, // requested in prompt (HEADER_CG_ITEM_DROP: Cell: (w, c), gold: u64)
}

impl TPacketCGItemDrop {
    pub fn new(window_type: u8, cell: u16, gold: u64) -> Self {
        Self {
            header: HEADER_CG_ITEM_DROP,
            length: size_of::<Self>() as u16,
            pos: TItemPos::new(window_type, cell),
            gold,
        }
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
    pub fn new(window_type: u8, cell: u16, gold: u32, count: u8) -> Self {
        Self {
            header: HEADER_CG_ITEM_DROP2,
            length: size_of::<Self>() as u16,
            pos: TItemPos::new(window_type, cell),
            gold,
            count,
        }
    }
}

/// FFI Export: m2_send_item_move
/// Serializuje pakiet TPacketCGItemMove i kopiuje do podanego bufora.
/// Zwraca wielkosc zapisanych bajtow (0 w przypadku bledu).
#[no_mangle]
pub unsafe extern "C" fn m2_send_item_move(
    src_w: u8,
    src_c: u16,
    dst_w: u8,
    dst_c: u16,
    count: u8,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet = TPacketCGItemMove::new(src_w, src_c, dst_w, dst_c, count);
    let packet_size = size_of::<TPacketCGItemMove>();

    ptr::copy_nonoverlapping(
        &packet as *const TPacketCGItemMove as *const u8,
        out_buf,
        packet_size,
    );

    packet_size
}
