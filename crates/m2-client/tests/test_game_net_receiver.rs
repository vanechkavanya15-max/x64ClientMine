use m2_client::game_net_receiver::{parse_packets, drain_events, GameEvent};

#[test]
fn test_route_item_set() {
    let mut pkt = vec![0u8; 10];
    pkt[0..2].copy_from_slice(&0x0511u16.to_le_bytes()); // gc_opcode::ITEM_SET = 0x0511
    pkt[2..4].copy_from_slice(&10u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 10);
}

#[test]
fn test_route_item_del() {
    let mut pkt = vec![0u8; 8];
    pkt[0..2].copy_from_slice(&0x0510u16.to_le_bytes()); // gc_opcode::ITEM_DEL = 0x0510
    pkt[2..4].copy_from_slice(&8u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 8);
}

#[test]
fn test_route_move() {
    let mut pkt = vec![0u8; 27];
    pkt[0..2].copy_from_slice(&0x0302u16.to_le_bytes()); // gc_opcode::MOVE = 0x0302
    pkt[2..4].copy_from_slice(&27u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 27);
}

#[test]
fn test_route_chat() {
    let mut pkt = vec![0u8; 20];
    pkt[0..2].copy_from_slice(&0x0603u16.to_le_bytes()); // gc_opcode::CHAT = 0x0603
    pkt[2..4].copy_from_slice(&20u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 20);
}

#[test]
fn test_route_damage_info() {
    let mut pkt = vec![0u8; 16];
    pkt[0..2].copy_from_slice(&0x0410u16.to_le_bytes()); // gc_opcode::DAMAGE_INFO = 0x0410
    pkt[2..4].copy_from_slice(&16u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 16);
}

#[test]
fn test_route_target() {
    let mut pkt = vec![0u8; 9];
    pkt[0..2].copy_from_slice(&0x0A10u16.to_le_bytes()); // gc_opcode::TARGET = 0x0A10
    pkt[2..4].copy_from_slice(&9u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 9);
}

#[test]
fn test_route_shop() {
    let mut pkt = vec![0u8; 16];
    pkt[0..2].copy_from_slice(&0x0810u16.to_le_bytes()); // gc_opcode::SHOP = 0x0810
    pkt[2..4].copy_from_slice(&16u16.to_le_bytes()); // length
    
    let consumed = parse_packets(&pkt);
    assert_eq!(consumed, 16);
}
