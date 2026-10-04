#[path = "../src/game_net_stream_dispatcher_wire_all.rs"]
mod game_net_stream_dispatcher_wire_all;
#[path = "../src/game_net_stream_dispatcher_complete.rs"]
pub mod game_net_stream_dispatcher_complete;

use game_net_stream_dispatcher_wire_all::dispatch_packet_wire_all;
use game_net_stream_dispatcher_complete::gc;

#[test]
fn test_guild_dispatcher() {
    let header = gc::GUILD;
    let data: Vec<u8> = vec![0; 100]; // Dummy data
    assert!(dispatch_packet_wire_all(header, &data));
}

#[test]
fn test_shop_dispatcher() {
    let header = gc::SHOP;
    let data: Vec<u8> = vec![0; 100]; // Dummy data
    let _ = dispatch_packet_wire_all(header, &data); 
}

#[test]
fn test_exchange_dispatcher() {
    let header = gc::EXCHANGE;
    let data: Vec<u8> = vec![0; 100];
    let _ = dispatch_packet_wire_all(header, &data);
}

#[test]
fn test_safebox_dispatcher() {
    let header = gc::SAFEBOX_SIZE;
    let data: Vec<u8> = vec![0; 100];
    assert!(dispatch_packet_wire_all(header, &data));
}
