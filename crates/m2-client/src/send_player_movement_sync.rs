#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketCgMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
}

pub const HEADER_CG_MOVE: u16 = 0x0301;

pub const MOVE_TYPE_WAIT: u8 = 0;
pub const MOVE_TYPE_WALK: u8 = 1;
pub const MOVE_TYPE_RUN: u8 = 2; // In C++ this is often mapped, but let's just expose constants.

/// Generuje strukture pakietu ruchu z zachowaniem 1:1 oryginalnej logiki obrotu w 8-bitach 
/// (rot / 360.0 * 255.0 to matematyczny przyklad, oryginal: fDstRot / 5.0f).
pub fn create_character_state_packet(
    pos_x: f32,
    pos_y: f32,
    mut dst_rot: f32,
    func: u8,
    arg: u8,
    time: u32,
) -> PacketCgMove {
    if dst_rot < 0.0 {
        dst_rot = 360.0 + dst_rot;
    } else if dst_rot > 360.0 {
        dst_rot = dst_rot % 360.0;
    }

    let b_rot = (dst_rot / 5.0) as u8;

    PacketCgMove {
        header: HEADER_CG_MOVE,
        length: std::mem::size_of::<PacketCgMove>() as u16,
        b_func: func,
        b_arg: arg,
        b_rot,
        l_x: pos_x as i32,
        l_y: pos_y as i32,
        dw_time: time,
    }
}
