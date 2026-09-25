use std::panic::catch_unwind;
use std::ptr;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    Eof,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct TPacketGCCharacterAdd {
    pub header: u8,          // HEADER_GC_CHARACTER_ADD = 1
    pub dw_vid: u32,         // VID aktora
    pub angle: f32,          // kat obrotu
    pub x: i32,              // pozycja X
    pub y: i32,              // pozycja Y
    pub z: i32,              // pozycja Z
    pub b_type: u8,          // typ: CHAR_TYPE_PC, MONSTER, NPC
    pub w_race_num: u16,     // identyfikator rasy / modelu
    pub b_moving_speed: u8,  // predkosc poruszania
    pub b_attack_speed: u8,  // predkosc ataku
    pub b_state_flag: u8,    // flagi stanow (ogluszenie, itp.)
    pub dw_affect_flag: [u32; 2], // aktywne efekty / aury
}

impl TPacketGCCharacterAdd {
    pub const PACKET_SIZE: usize = 35;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, PacketError> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(PacketError::Eof);
        }

        let header = buf[0];
        let dw_vid = u32::from_le_bytes(buf[1..5].try_into().unwrap());
        let angle = f32::from_le_bytes(buf[5..9].try_into().unwrap());
        let x = i32::from_le_bytes(buf[9..13].try_into().unwrap());
        let y = i32::from_le_bytes(buf[13..17].try_into().unwrap());
        let z = i32::from_le_bytes(buf[17..21].try_into().unwrap());
        let b_type = buf[21];
        let w_race_num = u16::from_le_bytes(buf[22..24].try_into().unwrap());
        let b_moving_speed = buf[24];
        let b_attack_speed = buf[25];
        let b_state_flag = buf[26];
        let affect_flag_0 = u32::from_le_bytes(buf[27..31].try_into().unwrap());
        let affect_flag_1 = u32::from_le_bytes(buf[31..35].try_into().unwrap());

        Ok(TPacketGCCharacterAdd {
            header,
            dw_vid,
            angle,
            x,
            y,
            z,
            b_type,
            w_race_num,
            b_moving_speed,
            b_attack_speed,
            b_state_flag,
            dw_affect_flag: [affect_flag_0, affect_flag_1],
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_char_add(
    data: *const u8,
    len: u32,
    out_vid: *mut u32,
    out_x: *mut i32,
    out_y: *mut i32,
    out_race: *mut u16,
) -> bool {
    let result = catch_unwind(std::panic::AssertUnwindSafe(|| {
        if data.is_null() || out_vid.is_null() || out_x.is_null() || out_y.is_null() || out_race.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCCharacterAdd::from_bytes(slice) {
            Ok(packet) => {
                ptr::write(out_vid, ptr::addr_of!(packet.dw_vid).read_unaligned());
                ptr::write(out_x, ptr::addr_of!(packet.x).read_unaligned());
                ptr::write(out_y, ptr::addr_of!(packet.y).read_unaligned());
                ptr::write(out_race, ptr::addr_of!(packet.w_race_num).read_unaligned());
                true
            }
            Err(_) => false,
        }
    }));

    result.unwrap_or(false)
}
