use std::mem::size_of;
use std::ptr;

// Header constants matching C++ codebase (uint8_t enum) -> No they are uint16_t but values like ITEM_USE=0x0501? Actually let's look at `constexpr uint16_t ITEM_USE = 0x0501;` etc.
// From Packet.h:
// constexpr uint16_t ITEM_USE           = 0x0501;
// constexpr uint16_t ITEM_DROP          = 0x0502;
// constexpr uint16_t ITEM_DROP2         = 0x0503;
// constexpr uint16_t ITEM_MOVE          = 0x0504;

pub const HEADER_CG_ITEM_USE: u16 = 0x0501;
pub const HEADER_CG_ITEM_DROP: u16 = 0x0502;
pub const HEADER_CG_ITEM_DROP2: u16 = 0x0503;
pub const HEADER_CG_ITEM_MOVE: u16 = 0x0504;

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

// typedef struct command_item_use { uint16_t header; uint16_t length; TItemPos pos; } TPacketCGItemUse;
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

// typedef struct command_item_move { uint16_t header; uint16_t length; TItemPos pos; TItemPos change_pos; uint8_t num; } TPacketCGItemMove;
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

// typedef struct command_item_drop { uint16_t header; uint16_t length; TItemPos pos; uint32_t elk; } TPacketCGItemDrop;
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub elk: u32,
}

impl TPacketCGItemDrop {
    pub fn new(window_type: u8, cell: u16, elk: u32) -> Self {
        Self {
            header: HEADER_CG_ITEM_DROP,
            length: size_of::<Self>() as u16,
            pos: TItemPos::new(window_type, cell),
            elk,
        }
    }
}

// typedef struct command_item_drop2 { uint16_t header; uint16_t length; TItemPos pos; uint32_t gold; uint8_t count; } TPacketCGItemDrop2;
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

#[no_mangle]
pub unsafe extern "C" fn m2_ffi_send_item_use(
    window_type: u8,
    cell: u16,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGItemUse::new(window_type, cell);
    let packet_size = size_of::<TPacketCGItemUse>();
    
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );
    
    packet_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_ffi_send_item_move(
    src_w: u8,
    src_c: u16,
    dst_w: u8,
    dst_c: u16,
    num: u8,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGItemMove::new(src_w, src_c, dst_w, dst_c, num);
    let packet_size = size_of::<TPacketCGItemMove>();
    
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );
    
    packet_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_ffi_send_item_drop(
    window_type: u8,
    cell: u16,
    elk: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGItemDrop::new(window_type, cell, elk);
    let packet_size = size_of::<TPacketCGItemDrop>();
    
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );
    
    packet_size
}

#[no_mangle]
pub unsafe extern "C" fn m2_ffi_send_item_drop2(
    window_type: u8,
    cell: u16,
    gold: u32,
    count: u8,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGItemDrop2::new(window_type, cell, gold, count);
    let packet_size = size_of::<TPacketCGItemDrop2>();
    
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );
    
    packet_size
}

// "Destroy" uses ITEM_DROP with 0 elk (as implemented in original python net runtime and requested logic)
#[no_mangle]
pub unsafe extern "C" fn m2_ffi_send_item_destroy(
    window_type: u8,
    cell: u16,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGItemDrop::new(window_type, cell, 0); // Destroy sends elk=0
    let packet_size = size_of::<TPacketCGItemDrop>();
    
    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        packet_size,
    );
    
    packet_size
}
