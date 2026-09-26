use m2_client_bridge::state::phase::{ClientPhase, GLOBAL_PHASE_MANAGER};
use once_cell::sync::Lazy;
use std::mem::size_of;
use std::sync::Mutex;

use crate::warp_transition::WarpTransition;

pub static GLOBAL_WARP_TRANSITION: Lazy<Mutex<WarpTransition>> = Lazy::new(|| {
    Mutex::new(WarpTransition::new())
});

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCWarp {
    pub header: u16,
    pub length: u16,
    pub lX: i32,
    pub lY: i32,
    pub lAddr: u32,
    pub wPort: u16,
}

pub fn parse_warp_packet(data: &[u8]) -> Result<TPacketGCWarp, &'static str> {
    if data.len() < size_of::<TPacketGCWarp>() {
        return Err("Buffer too short");
    }

    let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
    let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
    let lX = i32::from_le_bytes(data[4..8].try_into().unwrap());
    let lY = i32::from_le_bytes(data[8..12].try_into().unwrap());
    let lAddr = u32::from_le_bytes(data[12..16].try_into().unwrap());
    let wPort = u16::from_le_bytes(data[16..18].try_into().unwrap());

    Ok(TPacketGCWarp {
        header,
        length,
        lX,
        lY,
        lAddr,
        wPort,
    })
}

#[no_mangle]
pub extern "C" fn m2_net_recv_warp(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len == 0 {
        return false;
    }

    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    
    match parse_warp_packet(slice) {
        Ok(_) => {
            if let Ok(mut pm) = GLOBAL_PHASE_MANAGER.lock() {
                let _ = pm.set_phase(ClientPhase::Loading);
            }

            if let Ok(mut wt) = GLOBAL_WARP_TRANSITION.lock() {
                wt.physics_locked = true;
            }

            true
        }
        Err(_) => false,
    }
}
