use m2_client::game_net_stream_dispatcher_wire_all::dispatch_packet_wire_all;
use m2_client::game_net_stream_dispatcher_complete::gc;
use m2_client::recv_guild_packets_complete::TPacketGCGuild;
use m2_client::recv_shop_logic_v2::TPacketGCShop;
use m2_client::recv_exchange_logic_v2::TPacketGCExchange;
use m2_client::recv_safebox_packets_complete::TPacketGCSafeboxSize;
use std::mem::size_of;

#[test]
fn test_guild_dispatcher() {
    let header = gc::GUILD;
    let mut data: Vec<u8> = vec![0; size_of::<TPacketGCGuild>()]; 
    
    let packet = unsafe { &mut *(data.as_mut_ptr() as *mut TPacketGCGuild) };
    packet.header = gc::GUILD;
    packet.length = size_of::<TPacketGCGuild>() as u16;
    packet.subheader = 0; // Not a valid subheader for proper parse, but will trigger parse logic
    
    assert!(dispatch_packet_wire_all(header, &data));
}

#[test]
fn test_shop_dispatcher() {
    let header = gc::SHOP;
    let mut data: Vec<u8> = vec![0; size_of::<TPacketGCShop>()];
    let packet = unsafe { &mut *(data.as_mut_ptr() as *mut TPacketGCShop) };
    packet.header = gc::SHOP;
    packet.length = size_of::<TPacketGCShop>() as u16;
    packet.subheader = 99; // Dummy
    
    let _ = dispatch_packet_wire_all(header, &data); 
}

#[test]
fn test_exchange_dispatcher() {
    let header = gc::EXCHANGE;
    let data: Vec<u8> = vec![0; size_of::<TPacketGCExchange>()];
    let _ = dispatch_packet_wire_all(header, &data);
}

#[test]
fn test_safebox_dispatcher() {
    let header = gc::SAFEBOX_SIZE;
    let mut data: Vec<u8> = vec![0; size_of::<TPacketGCSafeboxSize>()];
    
    let packet = unsafe { &mut *(data.as_mut_ptr() as *mut TPacketGCSafeboxSize) };
    packet.header = gc::SAFEBOX_SIZE;
    packet.length = size_of::<TPacketGCSafeboxSize>() as u16;
    packet.bSize = 0;
    
    assert!(dispatch_packet_wire_all(header, &data));
}
