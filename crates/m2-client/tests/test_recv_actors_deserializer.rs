#[path = "../src/recv_actors_deserializer.rs"]
mod recv_actors_deserializer;

use m2_client_bridge::state::game_state::{Actor, GLOBAL_GAME_STATE};
use recv_actors_deserializer::{parse_character_add, parse_character_update, TPacketGCCharacterAdd, TPacketGCCharacterUpdate};
use std::mem::size_of;

#[test]
fn test_parse_character_add() {
    let packet = TPacketGCCharacterAdd {
        header: 1,
        length: size_of::<TPacketGCCharacterAdd>() as u16,
        dwVID: 12345,
        angle: 45.0,
        x: 100,
        y: 200,
        z: 300,
        bType: 2,
        wRaceNum: 3,
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
    };

    let slice = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCCharacterAdd>())
    };

    assert_eq!(parse_character_add(slice), Ok(12345));

    let state = GLOBAL_GAME_STATE.read().unwrap();
    let actor = state.get_actor(12345).unwrap();
    
    assert_eq!(actor.vid, 12345);
    assert_eq!(actor.x, 100.0);
    assert_eq!(actor.y, 200.0);
    assert_eq!(actor.z, 300.0);
    assert_eq!(actor.rot, 45.0);
    assert_eq!(actor.race, 3);
    assert_eq!(actor.b_type, 2);
}

#[test]
fn test_parse_character_update() {
    // Need to insert an actor first
    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.add_or_update_actor(Actor {
            vid: 999,
            ..Default::default()
        });
    }

    let packet = TPacketGCCharacterUpdate {
        header: 2,
        length: size_of::<TPacketGCCharacterUpdate>() as u16,
        dwVID: 999,
        awPart: [1, 2, 3, 4],
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
        dwGuildID: 1,
        sAlignment: 1,
        bPKMode: 0,
        dwMountVnum: 0,
    };

    let slice = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCCharacterUpdate>())
    };

    assert_eq!(parse_character_update(slice), Ok(999));
}
