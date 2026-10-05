#![allow(non_snake_case)]
use std::mem;

pub const HEADER_CG_REFINE: u16 = 0x0520;// Wait, the prompt says CG_REFINE (0x0520). I will use it. 
pub const HEADER_CG_DRAGON_SOUL_REFINE: u16 = 0x050D;
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
#[repr(u32)]
pub enum EDragonSoulRefineWindowRefineType {
    DragonSoulRefineWindow_UPGRADE = 0,
    DragonSoulRefineWindow_IMPROVEMENT = 1,
    DragonSoulRefineWindow_REFINE = 2,
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
    pub fn new(bSubType: u8, item_grid: &[TItemPos; DS_REFINE_WINDOW_MAX_NUM]) -> Self {
        Self {
            header: HEADER_CG_DRAGON_SOUL_REFINE,
            length: mem::size_of::<Self>() as u16,
            bSubType,
            ItemGrid: *item_grid,
        }
    }
}

#[derive(Debug)]
pub enum PacketFormatError {
    InvalidRefineType,
}

pub fn format_cg_refine_packet(pos: u8, type_: u8) -> Result<Vec<u8>, PacketFormatError> {
    let packet = TPacketCGRefine::new(pos, type_);
    
    let size = mem::size_of::<TPacketCGRefine>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: TPacketCGRefine is #[repr(C, packed)] and contains only primitive types.
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGRefine as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    
    Ok(buffer)
}

pub fn format_cg_dragon_soul_refine_packet(
    sub_type: u8,
    item_grid: &[TItemPos; DS_REFINE_WINDOW_MAX_NUM],
) -> Result<Vec<u8>, PacketFormatError> {
    let packet = TPacketCGDragonSoulRefine::new(sub_type, item_grid);
    
    let size = mem::size_of::<TPacketCGDragonSoulRefine>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: TPacketCGDragonSoulRefine is #[repr(C, packed)] and contains only primitive types (or arrays of C-packed structs).
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGDragonSoulRefine as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    
    Ok(buffer)
}
