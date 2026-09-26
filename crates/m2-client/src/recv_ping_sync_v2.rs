use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const GC_PING: u16 = 0x0007;
pub const CG_PONG: u16 = 0x0006;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketGCPing {
    pub header: u16,
    pub length: u16,
    pub server_time: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGPong {
    pub header: u16,
    pub length: u16,
}

#[derive(Debug)]
pub struct PingState {
    pub last_ping_client_time: u32,
    pub server_time_at_ping: u32,
    pub last_ping_system_time: Option<Instant>,
}

impl Default for PingState {
    fn default() -> Self {
        Self {
            last_ping_client_time: 0,
            server_time_at_ping: 0,
            last_ping_system_time: None,
        }
    }
}

static PING_STATE: OnceLock<Mutex<PingState>> = OnceLock::new();

pub fn get_ping_state() -> &'static Mutex<PingState> {
    PING_STATE.get_or_init(|| Mutex::new(PingState::default()))
}

/// Parsuje pakiet TPacketGCPing i zwraca serialized TPacketCGPong.
/// Symuluje dzialanie CPythonNetworkStream::RecvPingPacket() z Metin2 C++.
pub fn recv_ping_packet(data: &[u8], current_client_time: u32) -> Option<Vec<u8>> {
    let packet_size = std::mem::size_of::<TPacketGCPing>();
    if data.len() < packet_size {
        return None; // Pakiet zbyt krotki
    }

    let ping_packet: TPacketGCPing = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCPing) };

    if ping_packet.header != GC_PING {
        return None;
    }

    let mut state = get_ping_state().lock().unwrap();
    state.last_ping_client_time = current_client_time;
    state.server_time_at_ping = ping_packet.server_time;
    state.last_ping_system_time = Some(Instant::now());

    let pong_packet = TPacketCGPong {
        header: CG_PONG,
        length: std::mem::size_of::<TPacketCGPong>() as u16,
    };

    let pong_size = std::mem::size_of::<TPacketCGPong>();
    let mut pong_data = vec![0u8; pong_size];
    unsafe {
        std::ptr::write_unaligned(pong_data.as_mut_ptr() as *mut TPacketCGPong, pong_packet);
    }

    Some(pong_data)
}

/// Eksport FFI: Zwraca latencje obliczona od czasu otrzymania ostatniego pingu.
#[no_mangle]
pub extern "C" fn m2_net_get_latency(current_client_time: u32) -> u32 {
    safe_ffi_call(0, || {
        let state = get_ping_state().lock().unwrap();
        if state.last_ping_client_time > 0 {
            let elapsed = current_client_time.wrapping_sub(state.last_ping_client_time);
            if elapsed > u32::MAX / 2 { 
                0
            } else {
                elapsed
            }
        } else {
            0
        }
    })
}
