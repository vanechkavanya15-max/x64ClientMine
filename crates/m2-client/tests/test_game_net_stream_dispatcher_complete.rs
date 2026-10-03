#[path = "../src/game_net_stream_dispatcher_complete.rs"]
mod game_net_stream_dispatcher_complete;

#[test]
fn test_game_dispatcher_too_short() {
    let buf = [0u8; 1];
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, 0);
}

#[test]
fn test_game_dispatcher_valid_phase() {
    let mut buf = [0u8; 4];
    let header_val = 0x0008; // PHASE
    buf[0] = (header_val & 0xFF) as u8;
    buf[1] = ((header_val >> 8) & 0xFF) as u8;
    let packet_len = 4;
    buf[2] = (packet_len & 0xFF) as u8;
    buf[3] = ((packet_len >> 8) & 0xFF) as u8;
    
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, header_val);
}

#[test]
fn test_game_dispatcher_invalid_len() {
    let mut buf = [0u8; 4];
    let header_val = 0x0008; // PHASE
    buf[0] = (header_val & 0xFF) as u8;
    buf[1] = ((header_val >> 8) & 0xFF) as u8;
    let packet_len = 3;
    buf[2] = (packet_len & 0xFF) as u8;
    buf[3] = ((packet_len >> 8) & 0xFF) as u8;
    
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, 0);
}

#[test]
fn test_game_dispatcher_partial_packet() {
    let mut buf = [0u8; 8];
    let header_val = 0x000B; // KEY_CHALLENGE
    buf[0] = (header_val & 0xFF) as u8;
    buf[1] = ((header_val >> 8) & 0xFF) as u8;
    let packet_len = 10;
    buf[2] = (packet_len & 0xFF) as u8;
    buf[3] = ((packet_len >> 8) & 0xFF) as u8;
    
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, 0);
}

#[test]
fn test_game_dispatcher_unknown_header() {
    let mut buf = [0u8; 4];
    let header_val = 0xFFFF; // UNKNOWN
    buf[0] = (header_val & 0xFF) as u8;
    buf[1] = ((header_val >> 8) & 0xFF) as u8;
    let packet_len = 4;
    buf[2] = (packet_len & 0xFF) as u8;
    buf[3] = ((packet_len >> 8) & 0xFF) as u8;
    
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, header_val);
}

#[test]
fn test_game_dispatcher_valid_item_set() {
    let mut buf = [0u8; 4];
    let header_val = 0x0511; // ITEM_SET
    buf[0] = (header_val & 0xFF) as u8;
    buf[1] = ((header_val >> 8) & 0xFF) as u8;
    let packet_len = 4;
    buf[2] = (packet_len & 0xFF) as u8;
    buf[3] = ((packet_len >> 8) & 0xFF) as u8;
    
    let res = game_net_stream_dispatcher_complete::m2_net_dispatch_game_packet(buf.as_ptr(), buf.len());
    assert_eq!(res, header_val);
}
