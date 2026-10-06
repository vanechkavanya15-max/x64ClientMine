#![allow(non_snake_case)]

use std::mem;

pub const HEADER_CG_REFINE: u16 = 0x050C;
pub const HEADER_CG_ITEM_USE_TO_ITEM: u16 = 0x0506;
pub const REFINE_MATERIAL_MAX_NUM: usize = 5;

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
pub struct TPacketCGItemUseToItem {
    pub header: u16,
    pub length: u16,
    pub source_pos: TItemPos,
    pub target_pos: TItemPos,
}

impl TPacketCGItemUseToItem {
    pub fn new(source_pos: TItemPos, target_pos: TItemPos) -> Self {
        Self {
            header: HEADER_CG_ITEM_USE_TO_ITEM,
            length: mem::size_of::<Self>() as u16,
            source_pos,
            target_pos,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TMaterial {
    pub vnum: u32,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TRefineTable {
    pub src_vnum: u32,
    pub result_vnum: u32,
    pub material_count: u8,
    pub cost: i32,
    pub prob: i32,
    pub materials: [TMaterial; REFINE_MATERIAL_MAX_NUM],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketGCRefineInformation {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketGCRefineInformationNew {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
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

pub fn format_cg_item_use_to_item_packet(
    source_pos: TItemPos,
    target_pos: TItemPos,
) -> Result<Vec<u8>, PacketFormatError> {
    let packet = TPacketCGItemUseToItem::new(source_pos, target_pos);
    
    let size = mem::size_of::<TPacketCGItemUseToItem>();
    let mut buffer = vec![0u8; size];
    
    // SAFETY: TPacketCGItemUseToItem is #[repr(C, packed)] and contains only primitive types (or arrays of C-packed structs).
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGItemUseToItem as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    
    Ok(buffer)
}

pub fn handle_refine_dialog(
    refine_table: &TRefineTable,
    has_enough_money: bool,
    has_enough_materials: bool,
) -> bool {
    // Basic verification of required refine items (weryfikacja wymaganych ulepszaczy)
    // The dialog accept logic will only allow true if the user has enough money and materials.
    if !has_enough_money || !has_enough_materials {
        return false;
    }
    
    // If material_count is 0, it means no materials are required, but money still could be.
    // Ensure that material logic checks correctly for the client side.
    if refine_table.material_count > REFINE_MATERIAL_MAX_NUM as u8 {
        return false; // Malformed packet or data
    }
    
    true
}
