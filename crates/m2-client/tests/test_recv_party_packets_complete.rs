#[path = "../src/recv_party_packets_complete.rs"]
mod recv_party_packets_complete;

use recv_party_packets_complete::*;
use std::mem::size_of;

#[test]
fn test_packet_party_add_layout() {
    assert_eq!(size_of::<TPacketGCPartyAdd>(), 2 + 2 + 4 + 65);
}

#[test]
fn test_packet_party_update_layout() {
    assert_eq!(size_of::<TPacketGCPartyUpdate>(), 2 + 2 + 4 + 1 + 1 + 2 * 7);
}

#[test]
fn test_packet_party_remove_layout() {
    assert_eq!(size_of::<TPacketGCPartyRemove>(), 2 + 2 + 4);
}

#[test]
fn test_packet_party_link_layout() {
    assert_eq!(size_of::<TPacketGCPartyLink>(), 2 + 2 + 4 + 4);
}

#[test]
fn test_packet_party_unlink_layout() {
    assert_eq!(size_of::<TPacketGCPartyUnlink>(), 2 + 2 + 4 + 4);
}

#[test]
fn test_packet_party_parameter_layout() {
    assert_eq!(size_of::<TPacketGCPartyParameter>(), 2 + 2 + 1);
}

#[test]
fn test_parse_packet_add_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyAdd>()];
    
    // header
    data[0] = 0x11;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyAdd>() as u8;
    data[3] = 0x00;
    
    // pid
    data[4] = 0x10;
    data[5] = 0x27;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // name
    let name = b"Jules";
    data[8..8 + name.len()].copy_from_slice(name);
    
    let packet = parse_packet_add(&data).unwrap();
    assert_eq!({packet.header}, 0x0711);
    assert_eq!({packet.length}, size_of::<TPacketGCPartyAdd>() as u16);
    assert_eq!({packet.pid}, 10000);
    assert_eq!(&packet.name[..5], b"Jules");
}

#[test]
fn test_parse_packet_update_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyUpdate>()];
    
    // header
    data[0] = 0x12;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyUpdate>() as u8;
    data[3] = 0x00;
    
    // pid
    data[4] = 0x10;
    data[5] = 0x27;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // state
    data[8] = 2; // e.g. state
    
    // percent_hp
    data[9] = 100;
    
    // affects
    data[10] = 0x50; // 80
    data[11] = 0x00;
    
    let packet = parse_packet_update(&data).unwrap();
    assert_eq!({packet.header}, 0x0712);
    assert_eq!({packet.length}, size_of::<TPacketGCPartyUpdate>() as u16);
    assert_eq!({packet.pid}, 10000);
    assert_eq!({packet.state}, 2);
    assert_eq!({packet.percent_hp}, 100);
    assert_eq!({packet.affects[0]}, 80);
}

#[test]
fn test_parse_packet_remove_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyRemove>()];
    
    // header
    data[0] = 0x13;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyRemove>() as u8;
    data[3] = 0x00;
    
    // pid
    data[4] = 0x10;
    data[5] = 0x27;
    data[6] = 0x00;
    data[7] = 0x00;
    
    let packet = parse_packet_remove(&data).unwrap();
    assert_eq!({packet.header}, 0x0713);
    assert_eq!({packet.length}, size_of::<TPacketGCPartyRemove>() as u16);
    assert_eq!({packet.pid}, 10000);
}

#[test]
fn test_parse_packet_buffer_too_small() {
    let data = vec![0u8; size_of::<TPacketGCPartyAdd>() - 1];
    let result = parse_packet_add(&data);
    assert!(matches!(result, Err(PartyPacketParseError::BufferTooSmall)));
}

#[test]
fn test_parse_packet_link_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyLink>()];
    
    // header
    data[0] = 0x14;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyLink>() as u8;
    data[3] = 0x00;
    
    // pid
    data[4] = 0x10;
    data[5] = 0x27;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // vid
    data[8] = 0x20;
    data[9] = 0x4e;
    data[10] = 0x00;
    data[11] = 0x00;
    
    let packet = parse_packet_link(&data).unwrap();
    assert_eq!({packet.header}, 0x0714);
    assert_eq!({packet.length}, size_of::<TPacketGCPartyLink>() as u16);
    assert_eq!({packet.pid}, 10000);
    assert_eq!({packet.vid}, 20000);
}

#[test]
fn test_parse_packet_unlink_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyUnlink>()];
    
    // header
    data[0] = 0x15;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyUnlink>() as u8;
    data[3] = 0x00;
    
    // pid
    data[4] = 0x10;
    data[5] = 0x27;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // vid
    data[8] = 0x20;
    data[9] = 0x4e;
    data[10] = 0x00;
    data[11] = 0x00;
    
    let packet = parse_packet_unlink(&data).unwrap();
    assert_eq!({packet.header}, 0x0715);
    assert_eq!({packet.length}, size_of::<TPacketGCPartyUnlink>() as u16);
    assert_eq!({packet.pid}, 10000);
    assert_eq!({packet.vid}, 20000);
}

#[test]
fn test_parse_packet_parameter_success() {
    let mut data = vec![0u8; size_of::<TPacketGCPartyParameter>()];
    
    // header
    data[0] = 0x16;
    data[1] = 0x07;
    
    // length
    data[2] = size_of::<TPacketGCPartyParameter>() as u8;
    data[3] = 0x00;
    
    // b_distribute_mode
    data[4] = 1; // parity
    
    let packet = parse_packet_parameter(&data).unwrap();
    assert_eq!({packet.header}, 0x0716); // PARTY_PARAMETER
    assert_eq!({packet.length}, size_of::<TPacketGCPartyParameter>() as u16);
    assert_eq!({packet.b_distribute_mode}, 1);
}
