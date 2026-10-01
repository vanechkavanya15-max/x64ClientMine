use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use crate::app_timing_clock::m2_get_time; // Using the exported app timing clock

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
}

impl TPacketCGMove {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const HEADER: u16 = 0x0301;

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.header.to_le_bytes());
        buf[2..4].copy_from_slice(&self.length.to_le_bytes());
        buf[4] = self.b_func;
        buf[5] = self.b_arg;
        buf[6] = self.b_rot;
        buf[7..11].copy_from_slice(&self.l_x.to_le_bytes());
        buf[11..15].copy_from_slice(&self.l_y.to_le_bytes());
        buf[15..19].copy_from_slice(&self.dw_time.to_le_bytes());
        buf
    }
}

lazy_static::lazy_static! {
    static ref LAST_MOVE_PACKET_TIME: Mutex<Option<Instant>> = Mutex::new(None);
}

pub fn reset_movement_throttling() {
    if let Ok(mut lock) = LAST_MOVE_PACKET_TIME.lock() {
        *lock = None;
    }
}

#[no_mangle]
pub extern "C" fn m2_send_move_sync(x: f32, y: f32, rot: f32, func: u8) -> bool {
    let now = Instant::now();

    if func != 0 {
        if let Ok(mut last_time) = LAST_MOVE_PACKET_TIME.lock() {
            if let Some(time) = *last_time {
                if now.duration_since(time) < Duration::from_millis(150) {
                    return true;
                }
            }
            *last_time = Some(now);
        }
    } else {
        if let Ok(mut last_time) = LAST_MOVE_PACKET_TIME.lock() {
            *last_time = Some(now);
        }
    }

    let mut f_dst_rot = rot;
    if f_dst_rot < 0.0 {
        f_dst_rot = 360.0 + f_dst_rot;
    } else if f_dst_rot > 360.0 {
        f_dst_rot = f_dst_rot % 360.0;
    }
    
    if f_dst_rot < 0.0 {
        f_dst_rot = f_dst_rot.rem_euclid(360.0);
    }

    let b_rot = (f_dst_rot / 5.0) as u8;
    let b_arg = 0; // Passed as 0 in C++ for SendCharacterStatePacket
    let time = m2_get_time() as u32;

    let packet = TPacketCGMove {
        header: TPacketCGMove::HEADER,
        length: TPacketCGMove::SIZE as u16,
        b_func: func,
        b_arg,
        b_rot,
        l_x: x as i32,
        l_y: y as i32,
        dw_time: time,
    };

    let bytes = packet.to_bytes();

    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        if let Some((stream, cipher)) = guard.as_mut() {
            let mut encrypted_bytes = bytes.to_vec();
            cipher.encrypt_in_place(&mut encrypted_bytes);
            if stream.write_all(&encrypted_bytes).is_err() {
                return false;
            }
        }
    }

    true
}