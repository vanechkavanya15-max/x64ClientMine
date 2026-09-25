use m2_client::net_account_connector::*;
use std::mem;

#[test]
fn test_state_machine() {
    let mut connector = NetAccountConnector::new();
    assert_eq!(connector.state, State::Offline);

    connector.connect("127.0.0.1", 11000, "127.0.0.1", 11000);
    assert_eq!(connector.state, State::Offline);

    let packet = connector.recv_phase(Phase::PHASE_HANDSHAKE as u8);
    assert!(packet.is_none());
    assert_eq!(connector.state, State::Handshake);

    connector.set_login_info("testuser", "testpass");
    let packet = connector.recv_phase(Phase::PHASE_AUTH as u8);
    assert!(packet.is_some());
    assert_eq!(connector.state, State::Auth);

    let login_packet = packet.unwrap();
    assert_eq!(login_packet.header, HEADER_CG_LOGIN3);
    
    // Check null termination and string content
    let mut name_bytes = [0u8; 31];
    name_bytes[..8].copy_from_slice(b"testuser");
    assert_eq!(login_packet.name, name_bytes);

    let mut pwd_bytes = [0u8; 17];
    pwd_bytes[..8].copy_from_slice(b"testpass");
    assert_eq!(login_packet.pwd, pwd_bytes);

    // Ensure password is cleared
    assert_eq!(connector.password, "");

    // Auth success transitions to offline (disconnect)
    connector.recv_auth_success(TPacketGCAuthSuccess {
        bHeader: HEADER_GC_AUTH_SUCCESS,
        dwLoginKey: 12345,
        bResult: 1,
    });
    assert_eq!(connector.state, State::Offline);
    assert_eq!(connector.login_key, 12345);
}

#[test]
fn test_packet_layout() {
    // Assert lengths match exact packed C++ struct layouts
    assert_eq!(mem::size_of::<TPacketCGLogin3>(), 1 + 31 + 17 + 16);
    assert_eq!(mem::size_of::<TPacketGCPhase>(), 1 + 1);
    assert_eq!(mem::size_of::<TPacketGCHandshake>(), 1 + 4 + 4 + 4);
    assert_eq!(mem::size_of::<TPacketGCAuthSuccess>(), 1 + 4 + 1);
    assert_eq!(mem::size_of::<TPacketGCLoginFailure>(), 1 + 9);
    assert_eq!(mem::size_of::<TPacketGCPing>(), 1);
    assert_eq!(mem::size_of::<TPacketCGPong>(), 1);

    // Assert packed alignment
    assert_eq!(mem::align_of::<TPacketCGLogin3>(), 1);
    assert_eq!(mem::align_of::<TPacketGCHandshake>(), 1);
}

#[test]
fn test_set_login_info() {
    let mut connector = NetAccountConnector::new();
    let long_id = "this_is_a_very_long_id_that_exceeds_30_characters";
    let long_pwd = "this_is_a_very_long_password";

    connector.set_login_info(long_id, long_pwd);
    let packet = connector.recv_phase(Phase::PHASE_AUTH as u8).unwrap();

    // Check truncation
    let mut name_bytes = [0u8; 31];
    name_bytes[..30].copy_from_slice(&long_id.as_bytes()[..30]);
    assert_eq!(packet.name, name_bytes);

    let mut pwd_bytes = [0u8; 17];
    pwd_bytes[..16].copy_from_slice(&long_pwd.as_bytes()[..16]);
    assert_eq!(packet.pwd, pwd_bytes);
}

#[test]
fn test_recv_handshake() {
    let connector = NetAccountConnector::new();
    let packet = TPacketGCHandshake {
        header: HEADER_GC_HANDSHAKE,
        dwHandshake: 999,
        dwTime: 1000,
        lDelta: 50,
    };
    let response = connector.recv_handshake(packet);
    let dw_time = response.dwTime; assert_eq!(dw_time, 1100);
    let l_delta = response.lDelta; assert_eq!(l_delta, 0);
    let dw_handshake = response.dwHandshake; assert_eq!(dw_handshake, 999);
}

#[test]
fn test_recv_ping() {
    let connector = NetAccountConnector::new();
    let response = connector.recv_ping();
    assert_eq!(response.bHeader, HEADER_CG_PONG);
}
