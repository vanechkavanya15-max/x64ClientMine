use m2_client::recv_warp_phase_v2::{
    m2_net_recv_warp, parse_warp_packet, TPacketGCWarp, GLOBAL_WARP_TRANSITION,
};

use m2_client_bridge::state::phase::{ClientPhase, GLOBAL_PHASE_MANAGER};
use std::mem::size_of;

#[test]
fn test_struct_sizes() {
    assert_eq!(size_of::<TPacketGCWarp>(), 18);
}

#[test]
fn test_parse_warp_packet_success() {
    let packet = TPacketGCWarp {
        header: 0x0306,
        length: 18,
        lX: 12345,
        lY: 67890,
        lAddr: 0x7F000001,
        wPort: 13000,
    };

    let slice = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCWarp>())
    };

    let result = parse_warp_packet(slice);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    
    // Copy out fields from packed struct to verify properly
    let header = unsafe { std::ptr::addr_of!(parsed.header).read_unaligned() };
    let length = unsafe { std::ptr::addr_of!(parsed.length).read_unaligned() };
    let lX = unsafe { std::ptr::addr_of!(parsed.lX).read_unaligned() };
    let lY = unsafe { std::ptr::addr_of!(parsed.lY).read_unaligned() };
    let lAddr = unsafe { std::ptr::addr_of!(parsed.lAddr).read_unaligned() };
    let wPort = unsafe { std::ptr::addr_of!(parsed.wPort).read_unaligned() };
    
    assert_eq!(header, 0x0306);
    assert_eq!(length, 18);
    assert_eq!(lX, 12345);
    assert_eq!(lY, 67890);
    assert_eq!(lAddr, 0x7F000001);
    assert_eq!(wPort, 13000);
}

#[test]
fn test_parse_warp_packet_short_buffer() {
    let buf = [0u8; 17]; // One byte short
    let result = parse_warp_packet(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Buffer too short");
}

#[test]
fn test_m2_net_recv_warp_success() {
    // Reset global states for isolation
    {
        let mut pm = GLOBAL_PHASE_MANAGER.lock().unwrap();
        let _ = pm.set_phase(ClientPhase::Game);
    }
    {
        let mut wt = GLOBAL_WARP_TRANSITION.lock().unwrap();
        wt.physics_locked = false;
    }

    let packet = TPacketGCWarp {
        header: 0x0306,
        length: 18,
        lX: 100,
        lY: 200,
        lAddr: 0,
        wPort: 0,
    };

    let slice = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCWarp>())
    };

    let success = m2_net_recv_warp(slice.as_ptr(), slice.len());
    assert!(success);

    // Verify side effects
    {
        let pm = GLOBAL_PHASE_MANAGER.lock().unwrap();
        assert_eq!(pm.get_phase(), ClientPhase::Loading);
    }
    {
        let wt = GLOBAL_WARP_TRANSITION.lock().unwrap();
        assert!(wt.physics_locked);
    }
}

#[test]
fn test_m2_net_recv_warp_null_ptr() {
    let success = m2_net_recv_warp(std::ptr::null(), 18);
    assert!(!success);
}

#[test]
fn test_m2_net_recv_warp_zero_len() {
    let buf = [0u8; 18];
    let success = m2_net_recv_warp(buf.as_ptr(), 0);
    assert!(!success);
}
