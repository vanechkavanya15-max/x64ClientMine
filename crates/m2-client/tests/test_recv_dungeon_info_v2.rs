#[path = "../src/recv_dungeon_info_v2.rs"]
mod recv_dungeon_info_v2;

use recv_dungeon_info_v2::*;
use std::sync::Mutex;
use std::sync::OnceLock;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_parse_dungeon_packet_success() {
    let _guard = get_test_mutex().lock().unwrap();
    let packet = TPacketGCDungeonState {
        header: 200,
        floor_index: 5,
        time_remaining_sec: 120,
        monster_count: 50,
        gate_open: true,
    };
    
    let data: [u8; std::mem::size_of::<TPacketGCDungeonState>()] = unsafe { std::mem::transmute(packet) };
    assert_eq!(parse_dungeon_packet(&data), true);
    
    let (floor_index, time, monsters, gate) = m2_dungeon_get_state();
    assert_eq!(floor_index, 5);
    assert_eq!(time, 120);
    assert_eq!(monsters, 50);
    assert_eq!(gate, true);
}

#[test]
fn test_parse_dungeon_packet_too_short() {
    let _guard = get_test_mutex().lock().unwrap();
    let data = [0u8; std::mem::size_of::<TPacketGCDungeonState>() - 1];
    assert_eq!(parse_dungeon_packet(&data), false);
}

#[test]
fn test_parse_dungeon_packet_max_values() {
    let _guard = get_test_mutex().lock().unwrap();
    let packet = TPacketGCDungeonState {
        header: 200,
        floor_index: 255,
        time_remaining_sec: u32::MAX,
        monster_count: u32::MAX,
        gate_open: false,
    };
    
    let data: [u8; std::mem::size_of::<TPacketGCDungeonState>()] = unsafe { std::mem::transmute(packet) };
    assert_eq!(parse_dungeon_packet(&data), true);
    
    let (floor_index, time, monsters, gate) = m2_dungeon_get_state();
    assert_eq!(floor_index, 255);
    assert_eq!(time, u32::MAX);
    assert_eq!(monsters, u32::MAX);
    assert_eq!(gate, false);
}

#[test]
fn test_parse_dungeon_packet_sequential_updates() {
    let _guard = get_test_mutex().lock().unwrap();
    let mut packet = TPacketGCDungeonState {
        header: 200,
        floor_index: 1,
        time_remaining_sec: 10,
        monster_count: 10,
        gate_open: false,
    };
    
    let mut data: [u8; std::mem::size_of::<TPacketGCDungeonState>()] = unsafe { std::mem::transmute_copy(&packet) };
    assert_eq!(parse_dungeon_packet(&data), true);
    
    let (floor_index, time, monsters, gate) = m2_dungeon_get_state();
    assert_eq!(floor_index, 1);
    assert_eq!(time, 10);
    assert_eq!(monsters, 10);
    assert_eq!(gate, false);

    packet.floor_index = 2;
    packet.time_remaining_sec = 20;
    packet.monster_count = 5;
    packet.gate_open = true;

    data = unsafe { std::mem::transmute_copy(&packet) };
    assert_eq!(parse_dungeon_packet(&data), true);

    let (floor_index, time, monsters, gate) = m2_dungeon_get_state();
    assert_eq!(floor_index, 2);
    assert_eq!(time, 20);
    assert_eq!(monsters, 5);
    assert_eq!(gate, true);
}

#[test]
fn test_parse_dungeon_packet_unaligned() {
    let _guard = get_test_mutex().lock().unwrap();
    let packet = TPacketGCDungeonState {
        header: 200,
        floor_index: 3,
        time_remaining_sec: 60,
        monster_count: 2,
        gate_open: false,
    };
    
    let data: [u8; std::mem::size_of::<TPacketGCDungeonState>()] = unsafe { std::mem::transmute(packet) };
    let mut unaligned_data = vec![0u8; data.len() + 1];
    unaligned_data[1..].copy_from_slice(&data);
    
    assert_eq!(parse_dungeon_packet(&unaligned_data[1..]), true);
    
    let (floor_index, time, monsters, gate) = m2_dungeon_get_state();
    assert_eq!(floor_index, 3);
    assert_eq!(time, 60);
    assert_eq!(monsters, 2);
    assert_eq!(gate, false);
}
