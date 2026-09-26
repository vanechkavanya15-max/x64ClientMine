#[path = "../src/recv_actors.rs"]
mod recv_actors;

use recv_actors::{CharacterManager};
use m2_client_bridge::net::gc::char_add::{TPacketGCCharacterAdd, TPacketGCCharacterAdd2, TPacketGCCharacterAdditionalInfo};
use m2_client_bridge::net::gc::char_del::TPacketGCCharacterDelete;

#[test]
fn test_character_add() {
    let mut manager = CharacterManager::new();
    
    let packet = TPacketGCCharacterAdd {
        header: 0x0205,
        length: TPacketGCCharacterAdd::PACKET_SIZE as u16,
        dwVID: 1001,
        angle: 45.0,
        x: 1000,
        y: 2000,
        z: 3000,
        bType: 1,
        wRaceNum: 0, // Warrior
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
    };

    manager.handle_character_add(&packet);

    let actor = manager.get_actor(1001).expect("Actor should exist");
    assert_eq!(actor.vid, 1001);
    assert_eq!(actor.x, 1000);
    assert_eq!(actor.y, 2000);
    assert_eq!(actor.z, 3000);
    assert_eq!(actor.rotation, 45.0);
    assert_eq!(actor.race, 0);
    assert_eq!(actor.b_type, 1);
    assert_eq!(actor.moving_speed, 100);
    assert_eq!(actor.attack_speed, 100);
    assert_eq!(actor.state_flag, 0);
    assert_eq!(actor.affect_flag, [0, 0]);
}

#[test]
fn test_character_add2() {
    let mut manager = CharacterManager::new();

    let mut name = [0u8; 65];
    let name_str = b"TestActor";
    name[..name_str.len()].copy_from_slice(name_str);

    let packet = TPacketGCCharacterAdd2 {
        header: 0x0206,
        length: TPacketGCCharacterAdd2::PACKET_SIZE as u16,
        dwVID: 2002,
        name,
        angle: 90.0,
        x: 5000,
        y: 6000,
        z: 7000,
        bType: 1,
        wRaceNum: 1, // Ninja
        awPart: [10, 20, 30, 40],
        bMovingSpeed: 110,
        bAttackSpeed: 120,
        bStateFlag: 1,
        dwAffectFlag: [1, 2],
        bEmpire: 2,
        dwGuild: 99,
        sAlignment: 1000,
        bPKMode: 0,
        dwMountVnum: 0,
    };

    manager.handle_character_add2(&packet);

    let actor = manager.get_actor(2002).expect("Actor should exist");
    assert_eq!(actor.vid, 2002);
    assert_eq!(actor.name, "TestActor");
    assert_eq!(actor.x, 5000);
    assert_eq!(actor.y, 6000);
    assert_eq!(actor.z, 7000);
    assert_eq!(actor.rotation, 90.0);
    assert_eq!(actor.race, 1);
    assert_eq!(actor.b_type, 1);
    assert_eq!(actor.parts, [10, 20, 30, 40]);
    assert_eq!(actor.empire, 2);
    assert_eq!(actor.guild_id, 99);
    assert_eq!(actor.alignment, 1000);
    assert_eq!(actor.pk_mode, 0);
    assert_eq!(actor.mount_vnum, 0);
}

#[test]
fn test_character_del() {
    let mut manager = CharacterManager::new();

    // Add actor first
    let add_packet = TPacketGCCharacterAdd {
        header: 0x0205,
        length: TPacketGCCharacterAdd::PACKET_SIZE as u16,
        dwVID: 3003,
        angle: 0.0,
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
    manager.handle_character_add(&add_packet);
    assert!(manager.get_actor(3003).is_some());

    // Delete actor
    let del_packet = TPacketGCCharacterDelete {
        header: 0x0208,
        length: TPacketGCCharacterDelete::PACKET_SIZE as u16,
        dwVID: 3003,
    };
    manager.handle_character_del(&del_packet);
    assert!(manager.get_actor(3003).is_none());
}

#[test]
fn test_character_additional_info() {
    let mut manager = CharacterManager::new();

    // First add a character without name/guild
    let add_packet = TPacketGCCharacterAdd {
        header: 0x0205,
        length: TPacketGCCharacterAdd::PACKET_SIZE as u16,
        dwVID: 4004,
        angle: 0.0,
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
    manager.handle_character_add(&add_packet);

    let mut name = [0u8; 65];
    let name_str = b"UpdatedName";
    name[..name_str.len()].copy_from_slice(name_str);

    let info_packet = TPacketGCCharacterAdditionalInfo {
        header: 0x0207,
        length: TPacketGCCharacterAdditionalInfo::PACKET_SIZE as u16,
        dwVID: 4004,
        name,
        awPart: [1, 2, 3, 4],
        bEmpire: 3,
        dwGuildID: 55,
        dwLevel: 99,
        sAlignment: -2000,
        bPKMode: 1,
        dwMountVnum: 20110,
    };

    manager.handle_character_additional_info(&info_packet);

    let actor = manager.get_actor(4004).expect("Actor should exist");
    assert_eq!(actor.name, "UpdatedName");
    assert_eq!(actor.parts, [1, 2, 3, 4]);
    assert_eq!(actor.empire, 3);
    assert_eq!(actor.guild_id, 55);
    assert_eq!(actor.level, 99);
    assert_eq!(actor.alignment, -2000);
    assert_eq!(actor.pk_mode, 1);
    assert_eq!(actor.mount_vnum, 20110);
}

#[test]
fn test_character_additional_info_missing_actor() {
    let mut manager = CharacterManager::new();

    let mut name = [0u8; 65];
    let name_str = b"GhostActor";
    name[..name_str.len()].copy_from_slice(name_str);

    let info_packet = TPacketGCCharacterAdditionalInfo {
        header: 0x0207,
        length: TPacketGCCharacterAdditionalInfo::PACKET_SIZE as u16,
        dwVID: 9999, // Actor does not exist
        name,
        awPart: [0, 0, 0, 0],
        bEmpire: 0,
        dwGuildID: 0,
        dwLevel: 0,
        sAlignment: 0,
        bPKMode: 0,
        dwMountVnum: 0,
    };

    // This should just return without panicking
    manager.handle_character_additional_info(&info_packet);
    
    // Actor still shouldn't exist
    assert!(manager.get_actor(9999).is_none());
}
