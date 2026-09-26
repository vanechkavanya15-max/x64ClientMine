use m2_client::actor_spawn_v2::*;
use std::mem::size_of;

#[test]
fn test_global_to_local() {
    let (lx, ly) = global_to_local(1000, 2000, 500, 1000);
    assert_eq!(lx, 500.0);
    assert_eq!(ly, 1000.0);
}

#[test]
fn test_actor_spawn_packet_size() {
    assert_eq!(size_of::<TPacketGCCharacterAdd>(), 35);
    assert_eq!(size_of::<TPacketGCCharacterAdd2>(), 68);
    assert_eq!(size_of::<TPacketGCCharacterAdditionalInfo>(), 54);
}

#[test]
fn test_parse_character_add_success() {
    let packet = TPacketGCCharacterAdd {
        header: HEADER_GC_CHARACTER_ADD,
        dwVID: 111,
        angle: 45.0,
        x: 1000,
        y: 2000,
        z: 0,
        bType: 2, // NPC
        wRaceNum: 101, // Wild Dog
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 1,
        dwAffectFlag: [2, 3],
    };
    
    let data_ptr = &packet as *const _ as *const u8;
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, size_of::<TPacketGCCharacterAdd>()) };
    
    let actor = parse_character_add_v2(slice, 0, 0).expect("Failed to parse ADD packet");
    
    assert_eq!(actor.vid, 111);
    assert_eq!(actor.actor_type, 2);
    assert_eq!(actor.race, 101);
    assert_eq!(actor.affect_flags[0], 2);
    assert_eq!(actor.affect_flags[1], 3);
}

#[test]
fn test_parse_character_add2_success() {
    let mut packet = TPacketGCCharacterAdd2 {
        header: HEADER_GC_CHARACTER_ADD2,
        dwVID: 12345,
        name: [0; 25],
        angle: 45.0,
        x: 1500,
        y: 2500,
        z: 0,
        bType: 1, // PC
        wRaceNum: 0, // WARRIOR_M
        awPart: [11971, 299, 0, 0],
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
    };
    
    let name = b"TestName\0";
    packet.name[..name.len()].copy_from_slice(name);
    
    let data_ptr = &packet as *const _ as *const u8;
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, size_of::<TPacketGCCharacterAdd2>()) };
    
    let actor = parse_character_add_v2(slice, 500, 1000).expect("Failed to parse ADD2 packet");
    
    assert_eq!(actor.vid, 12345);
    assert_eq!(actor.name, "TestName");
    assert_eq!(actor.world_x, 1500);
    assert_eq!(actor.world_y, 2500);
    assert_eq!(actor.local_x, 1000.0);
    assert_eq!(actor.local_y, 1500.0);
    assert_eq!(actor.parts[0], 11971);
}

#[test]
fn test_parse_character_additional_info_success() {
    let mut packet = TPacketGCCharacterAdditionalInfo {
        header: HEADER_GC_CHARACTER_ADDITIONAL_INFO,
        dwVID: 999,
        name: [0; 25],
        awPart: [1, 2, 3, 4],
        bEmpire: 2, // Chunjo
        dwGuildID: 55,
        dwLevel: 105,
        sAlignment: 12000,
        bPKMode: 1, // Peace
        dwMountVnum: 20110,
    };
    
    let name = b"InfoTest\0";
    packet.name[..name.len()].copy_from_slice(name);
    
    let data_ptr = &packet as *const _ as *const u8;
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, size_of::<TPacketGCCharacterAdditionalInfo>()) };
    
    let actor = parse_character_add_v2(slice, 0, 0).expect("Failed to parse ADD_INFO packet");
    
    assert_eq!(actor.vid, 999);
    assert_eq!(actor.name, "InfoTest");
    assert_eq!(actor.empire_id, 2);
    assert_eq!(actor.guild_id, 55);
    assert_eq!(actor.alignment, 12000);
    assert_eq!(actor.pk_mode, 1);
    assert_eq!(actor.mount_vnum, 20110);
}

#[test]
fn test_parse_character_add_v2_short_buffer() {
    let data = [0u8; 10];
    let result = parse_character_add_v2(&data, 0, 0);
    assert!(result.is_err());
}

#[test]
fn test_m2_actor_spawn_full_ffi() {
    let packet = TPacketGCCharacterAdd {
        header: HEADER_GC_CHARACTER_ADD,
        dwVID: 54321,
        angle: 90.0,
        x: 0,
        y: 0,
        z: 0,
        bType: 0,
        wRaceNum: 0,
        bMovingSpeed: 0,
        bAttackSpeed: 0,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
    };
    
    let data_ptr = &packet as *const _ as *const u8;
    let mut out_vid = 0;
    
    let success = m2_actor_spawn_full(data_ptr, size_of::<TPacketGCCharacterAdd>(), 0, 0, &mut out_vid);
    
    assert!(success);
    assert_eq!(out_vid, 54321);
}

#[test]
fn test_m2_actor_spawn_full_ffi_invalid() {
    let success = m2_actor_spawn_full(std::ptr::null(), 0, 0, 0, std::ptr::null_mut());
    assert!(!success);
}
