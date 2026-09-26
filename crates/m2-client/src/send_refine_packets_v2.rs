use std::ptr;

pub const HEADER_CG_REFINE: u16 = 0x050C;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefineType {
    Normal = 0,
    BlessingScroll = 1,
    MagicMetal = 2,
}

#[repr(C, packed(1))]
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
            length: std::mem::size_of::<Self>() as u16,
            pos,
            type_,
        }
    }
}

/// Serializes and sends a refine packet.
/// 
/// # Safety
/// `out_buf` must point to a valid, writable memory region of at least 6 bytes.
#[no_mangle]
pub unsafe extern "C" fn m2_send_refine(pos: u16, refine_type: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet = TPacketCGRefine::new(pos as u8, refine_type);
    let size = std::mem::size_of::<TPacketCGRefine>();

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size,
    );

    size
}
