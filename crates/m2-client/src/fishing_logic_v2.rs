use std::sync::atomic::{AtomicU32, Ordering};
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingState {
    Idle,
    Casting,
    Waiting,
    Catching,
    Done,
}

pub static FISHING_STATE: std::sync::RwLock<FishingState> = std::sync::RwLock::new(FishingState::Idle);
pub static HIT_WINDOW_MS: AtomicU32 = AtomicU32::new(0);

pub const HEADER_CG_FISHING: u16 = 0x0B01;

#[repr(C, packed)]
pub struct TPacketCGFishing {
    pub header: u16,
    pub length: u16,
    pub dir: u8,
}

impl TPacketCGFishing {
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;
    
    pub fn new(dir: u8) -> Self {
        Self {
            header: HEADER_CG_FISHING,
            length: Self::SIZE,
            dir,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.dir);
        buf
    }
}

pub const FISHING_SUB_HEADER_START: u8 = 0;
pub const FISHING_SUB_HEADER_HIT: u8 = 1;
pub const FISHING_SUB_HEADER_SUCCESS: u8 = 2;
pub const FISHING_SUB_HEADER_FAIL: u8 = 3;

#[repr(C, packed)]
pub struct TPacketGCFishing {
    pub header: u16,
    pub length: u16,
    pub sub_header: u8,
    pub info: u32,
    pub dir: u8,
}

#[no_mangle]
pub extern "C" fn m2_fishing_get_hit_window() -> u32 {
    HIT_WINDOW_MS.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn m2_fishing_send_cast(dir: u8) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut state) = FISHING_STATE.write() {
            *state = FishingState::Casting;
        }
        let packet = TPacketCGFishing::new(dir);
        enqueue_outgoing_packet(packet.to_bytes())
    })
}

pub fn handle_fishing_packet(packet: &TPacketGCFishing) {
    if let Ok(mut state) = FISHING_STATE.write() {
        match packet.sub_header {
            FISHING_SUB_HEADER_START => {
                *state = FishingState::Waiting;
            },
            FISHING_SUB_HEADER_HIT => {
                *state = FishingState::Catching;
                HIT_WINDOW_MS.store(packet.info, Ordering::Relaxed);
            },
            FISHING_SUB_HEADER_SUCCESS | FISHING_SUB_HEADER_FAIL => {
                *state = FishingState::Done;
                HIT_WINDOW_MS.store(0, Ordering::Relaxed);
            },
            _ => {}
        }
    }
}
