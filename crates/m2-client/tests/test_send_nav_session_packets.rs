use m2_client::send_nav_session_packets::*;
use std::mem::size_of;

#[test]
fn test_packet_sizes() {
    assert_eq!(size_of::<TPacketCGWarp>(), 4, "TPacketCGWarp should be exactly 4 bytes");
    assert_eq!(size_of::<TPacketCGLogout>(), 4, "TPacketCGLogout should be exactly 4 bytes");
    assert_eq!(size_of::<TPacketCGExitGame>(), 4, "TPacketCGExitGame should be exactly 4 bytes");
    assert_eq!(size_of::<TPacketCGPosition>(), 5, "TPacketCGPosition should be exactly 5 bytes");
}

#[test]
fn test_build_warp_packet() {
    let packet_bytes = build_warp_packet();
    
    assert_eq!(packet_bytes.len(), 4);
    
    // Header (u16, little endian)
    let header = u16::from_le_bytes([packet_bytes[0], packet_bytes[1]]);
    assert_eq!(header, HEADER_CG_WARP);
    
    // Length (u16, little endian)
    let length = u16::from_le_bytes([packet_bytes[2], packet_bytes[3]]);
    assert_eq!(length, 4);
}

#[test]
fn test_build_logout_packet() {
    let packet_bytes = build_logout_packet();
    
    assert_eq!(packet_bytes.len(), 4);
    
    let header = u16::from_le_bytes([packet_bytes[0], packet_bytes[1]]);
    assert_eq!(header, HEADER_CG_LOGOUT);
    
    let length = u16::from_le_bytes([packet_bytes[2], packet_bytes[3]]);
    assert_eq!(length, 4);
}

#[test]
fn test_build_exit_game_packet() {
    let packet_bytes = build_exit_game_packet();
    
    assert_eq!(packet_bytes.len(), 4);
    
    let header = u16::from_le_bytes([packet_bytes[0], packet_bytes[1]]);
    assert_eq!(header, HEADER_CG_EXIT_GAME);
    
    let length = u16::from_le_bytes([packet_bytes[2], packet_bytes[3]]);
    assert_eq!(length, 4);
}

#[test]
fn test_build_character_position_packet() {
    let position = 42;
    let packet_bytes = build_character_position_packet(position);
    
    assert_eq!(packet_bytes.len(), 5);
    
    let header = u16::from_le_bytes([packet_bytes[0], packet_bytes[1]]);
    assert_eq!(header, HEADER_CG_CHARACTER_POSITION);
    
    let length = u16::from_le_bytes([packet_bytes[2], packet_bytes[3]]);
    assert_eq!(length, 5);
    
    assert_eq!(packet_bytes[4], position);
}

#[test]
fn test_packet_layouts() {
    let warp = TPacketCGWarp { header: HEADER_CG_WARP, length: 4 };
    assert_eq!({ warp.header }, HEADER_CG_WARP);
    
    let logout = TPacketCGLogout { header: HEADER_CG_LOGOUT, length: 4 };
    assert_eq!({ logout.header }, HEADER_CG_LOGOUT);
    
    let exit = TPacketCGExitGame { header: HEADER_CG_EXIT_GAME, length: 4 };
    assert_eq!({ exit.header }, HEADER_CG_EXIT_GAME);
    
    let position = TPacketCGPosition { header: HEADER_CG_CHARACTER_POSITION, length: 5, position: 2 };
    assert_eq!({ position.position }, 2);
}
