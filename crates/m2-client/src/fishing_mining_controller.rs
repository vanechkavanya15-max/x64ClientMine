use std::sync::{Mutex, OnceLock};
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;

// --- Networking ---
pub const HEADER_CG_FISHING: u8 = 42;

pub const FISHING_SUB_HEADER_START: u8 = 0;
pub const FISHING_SUB_HEADER_HIT: u8 = 1;
pub const FISHING_SUB_HEADER_SUCCESS: u8 = 2;
pub const FISHING_SUB_HEADER_FAIL: u8 = 3;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketCGFishing {
    pub header: u8,
    pub dir: u8,
}

impl TPacketCGFishing {
    pub fn new(dir: u8) -> Self {
        Self {
            header: HEADER_CG_FISHING,
            dir,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.header, self.dir]
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCFishing {
    pub header: u8,
    pub subheader: u8,
    pub info: u32,
    pub dir: u8,
}

// --- State Machines ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingState {
    Idle,
    Casting,
    Waiting,
    Hit,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiningState {
    Idle,
    Mining { start_time: u64, duration: u64, target_vid: u32 },
    Success,
    Fail,
}

pub struct FishingMiningController {
    pub fishing_state: FishingState,
    pub hit_window_ms: u32,
    
    pub mining_state: MiningState,
    pub pickaxe_points: u32,
}

impl Default for FishingMiningController {
    fn default() -> Self {
        Self::new()
    }
}

impl FishingMiningController {
    pub fn new() -> Self {
        Self {
            fishing_state: FishingState::Idle,
            hit_window_ms: 0,
            mining_state: MiningState::Idle,
            pickaxe_points: 0,
        }
    }
    
    // -- Fishing Methods --
    pub fn send_cast_fishing(&mut self, dir: u8) {
        self.fishing_state = FishingState::Casting;
        let packet = TPacketCGFishing::new(dir);
        enqueue_outgoing_packet(packet.to_bytes());
    }
    
    pub fn handle_gc_fishing_packet(&mut self, packet: &TPacketGCFishing) {
        match {packet.subheader} {
            FISHING_SUB_HEADER_START => {
                self.fishing_state = FishingState::Waiting;
            }
            FISHING_SUB_HEADER_HIT => {
                self.fishing_state = FishingState::Hit;
                self.hit_window_ms = {packet.info};
            }
            FISHING_SUB_HEADER_SUCCESS => {
                self.fishing_state = FishingState::Done;
                self.hit_window_ms = 0;
            }
            FISHING_SUB_HEADER_FAIL => {
                self.fishing_state = FishingState::Done;
                self.hit_window_ms = 0;
            }
            _ => {}
        }
    }
    
    pub fn pull_fishing_rod(&mut self) -> bool {
        // Zgodnie z logika Metin2, gracz moze wyciagnac wedke w trakcie oczekiwania (Waiting) lub brania (Hit)
        if self.fishing_state == FishingState::Hit || self.fishing_state == FishingState::Waiting {
            self.fishing_state = FishingState::Idle;
            return true;
        }
        false
    }
    
    // -- Mining Methods --
    pub fn start_mining(&mut self, target_vid: u32, now_ms: u64, duration: u64) -> bool {
        if self.mining_state != MiningState::Idle {
            return false;
        }
        self.mining_state = MiningState::Mining {
            start_time: now_ms,
            duration,
            target_vid,
        };
        true
    }
    
    pub fn update_mining(&mut self, now_ms: u64, success: bool) -> bool {
        if let MiningState::Mining { start_time, duration, .. } = self.mining_state {
            if now_ms >= start_time + duration {
                if success {
                    self.mining_state = MiningState::Success;
                    self.pickaxe_points += 1;
                } else {
                    self.mining_state = MiningState::Fail;
                }
                return true;
            }
        }
        false
    }
    
    pub fn stop_mining(&mut self) {
        self.mining_state = MiningState::Idle;
    }
}

static CONTROLLER: OnceLock<Mutex<FishingMiningController>> = OnceLock::new();

pub fn get_controller() -> &'static Mutex<FishingMiningController> {
    CONTROLLER.get_or_init(|| Mutex::new(FishingMiningController::new()))
}

// -- FFI Exports --

#[no_mangle]
pub extern "C" fn m2_fm_controller_send_cast_fishing(dir: u8) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut ctrl) = get_controller().lock() {
            ctrl.send_cast_fishing(dir);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_handle_fishing_packet(packet: *const TPacketGCFishing) {
    safe_ffi_call((), || {
        if packet.is_null() { return; }
        if let Ok(mut ctrl) = get_controller().lock() {
            unsafe {
                ctrl.handle_gc_fishing_packet(&*packet);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_pull_fishing_rod() -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut ctrl) = get_controller().lock() {
            ctrl.pull_fishing_rod()
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_get_fishing_state() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(ctrl) = get_controller().lock() {
            ctrl.fishing_state as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_start_mining(target_vid: u32, now_ms: u64, duration: u64) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut ctrl) = get_controller().lock() {
            ctrl.start_mining(target_vid, now_ms, duration)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_update_mining(now_ms: u64, success: bool) -> bool {
    safe_ffi_call(false, || {
        if let Ok(mut ctrl) = get_controller().lock() {
            ctrl.update_mining(now_ms, success)
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_fm_controller_stop_mining() {
    safe_ffi_call((), || {
        if let Ok(mut ctrl) = get_controller().lock() {
            ctrl.stop_mining();
        }
    });
}
