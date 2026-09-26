use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use m2_client::recv_ping_sync_v2::*;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn clear_state() {
    let mut state = get_ping_state().lock().unwrap();
    state.last_ping_client_time = 0;
    state.server_time_at_ping = 0;
    state.last_ping_system_time = None;
}

#[test]
fn test_recv_ping_packet_too_small() {
    let _lock = get_test_lock();
    clear_state();

    let data = vec![0u8; 4]; // Zbyt mały pakiet
    let result = recv_ping_packet(&data, 1000);
    assert!(result.is_none());
}

#[test]
fn test_recv_ping_packet_invalid_header() {
    let _lock = get_test_lock();
    clear_state();

    let packet = TPacketGCPing {
        header: 0x99, // Błędny nagłówek
        length: 8,
        server_time: 123456,
    };
    
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCPing>()];
    unsafe {
        std::ptr::write_unaligned(data.as_mut_ptr() as *mut TPacketGCPing, packet);
    }

    let result = recv_ping_packet(&data, 1000);
    assert!(result.is_none());
}

#[test]
fn test_recv_ping_packet_success() {
    let _lock = get_test_lock();
    clear_state();

    let packet = TPacketGCPing {
        header: GC_PING,
        length: 8,
        server_time: 888888,
    };

    let mut data = vec![0u8; std::mem::size_of::<TPacketGCPing>()];
    unsafe {
        std::ptr::write_unaligned(data.as_mut_ptr() as *mut TPacketGCPing, packet);
    }

    let current_client_time = 2500;
    let result = recv_ping_packet(&data, current_client_time);
    
    assert!(result.is_some());
    let pong_data = result.unwrap();
    
    // Sprawdzenie poprawności zwróconego pakietu TPacketCGPong
    assert_eq!(pong_data.len(), std::mem::size_of::<TPacketCGPong>());
    
    let pong_packet: TPacketCGPong = unsafe { std::ptr::read_unaligned(pong_data.as_ptr() as *const TPacketCGPong) };
    let header = pong_packet.header;
    assert_eq!(header, CG_PONG);
    let length = pong_packet.length;
    assert_eq!(length, 4);
    
    // Weryfikacja zmiany stanu
    let state = get_ping_state().lock().unwrap();
    assert_eq!(state.last_ping_client_time, current_client_time);
    assert_eq!(state.server_time_at_ping, 888888);
    assert!(state.last_ping_system_time.is_some());
}

#[test]
fn test_m2_net_get_latency_no_ping() {
    let _lock = get_test_lock();
    clear_state();

    // Jeśli nie było jeszcze pingu, latencja powinna wynosić 0
    let latency = m2_net_get_latency(1000);
    assert_eq!(latency, 0);
}

#[test]
fn test_m2_net_get_latency_after_ping() {
    let _lock = get_test_lock();
    clear_state();

    let packet = TPacketGCPing {
        header: GC_PING,
        length: 8,
        server_time: 111111,
    };

    let mut data = vec![0u8; std::mem::size_of::<TPacketGCPing>()];
    unsafe {
        std::ptr::write_unaligned(data.as_mut_ptr() as *mut TPacketGCPing, packet);
    }

    let _ = recv_ping_packet(&data, 3000);
    
    let latency = m2_net_get_latency(3015);
    assert_eq!(latency, 15);
}

#[test]
fn test_m2_net_get_latency_overflow_protection() {
    let _lock = get_test_lock();
    clear_state();

    {
        let mut state = get_ping_state().lock().unwrap();
        state.last_ping_client_time = 1000;
        state.last_ping_system_time = Some(Instant::now());
    }
    
    // Wrap around condition
    let current_time = 1000u32.wrapping_sub(10);
    let latency = m2_net_get_latency(current_time);
    assert_eq!(latency, 0); 
}
