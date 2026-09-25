use m2_client::phase_handshake::{
    PhaseHandshake, TPacketGCHandshake, TPacketGCBlank, HEADER_CG_TIME_SYNC
};

#[test]
fn test_integration_phase_handshake() {
    let instance = PhaseHandshake::new();
    assert!(instance.is_ready());
}

#[test]
fn test_packet_memory_layout() {
    assert_eq!(
        std::mem::size_of::<TPacketGCHandshake>(),
        1 + 4 + 4,
        "TPacketGCHandshake size should be 9 bytes (packed)"
    );
    assert_eq!(
        std::mem::size_of::<TPacketGCBlank>(),
        1,
        "TPacketGCBlank size should be 1 byte (packed)"
    );

    let packet = TPacketGCHandshake::default();
    let base_ptr = &packet as *const _ as usize;
    let header_ptr = std::ptr::addr_of!(packet.header) as usize;
    let dw_time_ptr = std::ptr::addr_of!(packet.dwTime) as usize;
    let l_delta_ptr = std::ptr::addr_of!(packet.lDelta) as usize;

    assert_eq!(header_ptr - base_ptr, 0);
    assert_eq!(dw_time_ptr - base_ptr, 1);
    assert_eq!(l_delta_ptr - base_ptr, 5);
}

#[test]
fn test_handshake_phase_set() {
    let mut instance = PhaseHandshake::new();
    instance.set_handshake_phase(1000);
    assert_eq!(instance.m_strPhase, "HandShake");
    assert_eq!(instance.m_dwChangingPhaseTime, 1000);
}

#[test]
fn test_process_handshake_packet() {
    let mut instance = PhaseHandshake::new();
    let packet = TPacketGCHandshake {
        header: 40,
        dwTime: 5000,
        lDelta: 250,
    };

    let result = instance.process_handshake_packet(packet).unwrap();
    
    // server_msec = dwTime + lDelta = 5000 + 250 = 5250
    assert_eq!(instance.server_msec, 5250);
    
    // result.dwTime = dwTime + 2 * lDelta = 5000 + 500 = 5500
    let res_dw_time = { result.dwTime };
    let res_l_delta = { result.lDelta };
    assert_eq!(res_dw_time, 5500);
    assert_eq!(res_l_delta, 0);
    assert_eq!(result.header, 40); // Original header maintained
}

#[test]
fn test_process_handshake_packet_wrapping() {
    let mut instance = PhaseHandshake::new();
    let packet = TPacketGCHandshake {
        header: 40,
        dwTime: u32::MAX - 100,
        lDelta: 200,
    };

    let result = instance.process_handshake_packet(packet).unwrap();
    
    // server_msec = u32::MAX - 100 + 200 = 99 (wrapping)
    assert_eq!(instance.server_msec, 99);
    
    // result.dwTime = u32::MAX - 100 + 400 = 299 (wrapping)
    let res_dw_time = { result.dwTime };
    let res_l_delta = { result.lDelta };
    assert_eq!(res_dw_time, 299);
    assert_eq!(res_l_delta, 0);
}

#[test]
fn test_recv_handshake_packet() {
    let mut instance = PhaseHandshake::new();
    let packet = TPacketGCHandshake {
        header: 40,
        dwTime: 10000,
        lDelta: -500,
    };

    let result = instance.recv_handshake_packet(packet, 2000).unwrap();
    
    assert_eq!(instance.m_kServerTimeSync.m_dwChangeServerTime, 9500);
    assert_eq!(instance.m_kServerTimeSync.m_dwChangeClientTime, 2000);
    
    let res_dw_time = { result.dwTime };
    let res_l_delta = { result.lDelta };
    assert_eq!(res_dw_time, 9000);
    assert_eq!(res_l_delta, 0);
    assert_eq!(result.header, HEADER_CG_TIME_SYNC);
}

#[test]
fn test_recv_handshake_ok_packet() {
    let mut instance = PhaseHandshake::new();
    instance.m_kServerTimeSync.m_dwChangeServerTime = 9500;
    instance.m_kServerTimeSync.m_dwChangeClientTime = 2000;

    // Simulate time passing: client time is now 2100 (delta = 100)
    let new_server_msec = instance.recv_handshake_ok_packet(2100);
    
    // new_server_msec = 9500 + 100 = 9600
    assert_eq!(new_server_msec, 9600);
    assert_eq!(instance.server_msec, 9600);
}
