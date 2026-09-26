use std::sync::atomic::{AtomicU32, Ordering};

#[repr(C, packed)]
pub struct TPacketGCFishing {
    pub sub_header: u8,
    pub vid: u32,
    pub hit_window_ms: u32,
}

pub const FISHING_SUB_HEADER_START: u8 = 0;
pub const FISHING_SUB_HEADER_HIT: u8 = 1;
pub const FISHING_SUB_HEADER_SUCCESS: u8 = 2;
pub const FISHING_SUB_HEADER_FAIL: u8 = 3;

pub static HIT_WINDOW_MS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn m2_fishing_get_hit_window() -> u32 {
    HIT_WINDOW_MS.load(Ordering::Relaxed)
}

pub fn handle_fishing_packet(packet: &TPacketGCFishing) {
    if packet.sub_header == FISHING_SUB_HEADER_HIT {
        HIT_WINDOW_MS.store(packet.hit_window_ms, Ordering::Relaxed);
    }
}
