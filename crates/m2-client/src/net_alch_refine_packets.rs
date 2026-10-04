use std::mem;

pub const HEADER_CG_REFINE: u16 = 0x050C;
pub const HEADER_CG_DRAGON_SOUL_REFINE: u16 = 0x050D;
pub const HEADER_CG_REFINE_CANCEL: u16 = 0x050E;
pub const DS_REFINE_WINDOW_MAX_NUM: usize = 15;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(C, packed)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl TItemPos {
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self { window_type, cell }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketCGRefine {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
    pub type_: u8,
}

impl TPacketCGRefine {
    pub fn new(pos: u8, type_: u8) -> Self {
        Self {
            header: HEADER_CG_REFINE,
            length: mem::size_of::<Self>() as u16,
            pos,
            type_,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketCGRefineCancel {
    pub header: u16,
    pub length: u16,
}

impl TPacketCGRefineCancel {
    pub fn new() -> Self {
        Self {
            header: HEADER_CG_REFINE_CANCEL,
            length: mem::size_of::<Self>() as u16,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDragonSoulRefineWindowRefineType {
    Upgrade = 0,
    Improvement = 1,
    Refine = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketCGDragonSoulRefine {
    pub header: u16,
    pub length: u16,
    pub bSubType: u8,
    pub ItemGrid: [TItemPos; DS_REFINE_WINDOW_MAX_NUM],
}

impl TPacketCGDragonSoulRefine {
    pub fn new(sub_type: u8, item_grid: &[TItemPos; DS_REFINE_WINDOW_MAX_NUM]) -> Self {
        Self {
            header: HEADER_CG_DRAGON_SOUL_REFINE,
            length: mem::size_of::<Self>() as u16,
            bSubType: sub_type,
            ItemGrid: *item_grid,
        }
    }
}

pub fn serialize_refine_packet(pos: u8, type_: u8) -> Vec<u8> {
    let packet = TPacketCGRefine::new(pos, type_);
    let size = mem::size_of::<TPacketCGRefine>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: Struct is #[repr(C, packed)] and contains only primitive types.
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGRefine as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    buffer
}

pub fn serialize_refine_cancel_packet() -> Vec<u8> {
    let packet = TPacketCGRefineCancel::new();
    let size = mem::size_of::<TPacketCGRefineCancel>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: Struct is #[repr(C, packed)] and contains only primitive types.
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGRefineCancel as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    buffer
}

pub fn serialize_dragon_soul_refine_packet(
    sub_type: u8,
    item_grid: &[TItemPos; DS_REFINE_WINDOW_MAX_NUM],
) -> Vec<u8> {
    let packet = TPacketCGDragonSoulRefine::new(sub_type, item_grid);
    let size = mem::size_of::<TPacketCGDragonSoulRefine>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: Struct is #[repr(C, packed)] and contains only primitive types.
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGDragonSoulRefine as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    buffer
}
