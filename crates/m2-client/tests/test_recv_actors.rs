
use m2_client_bridge::state::game_state::GameState;


use m2_client::recv_actors::{
    TPacketGCCharacterAdd, TPacketGCPointChange, TPacketGCPoints,
    parse_character_add, parse_point_change,
};
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_HP, POINT_MAX_HP};
use std::mem::size_of;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_struct_sizes() {
    assert_eq!(size_of::<TPacketGCCharacterAdd>(), 38);
    assert_eq!(size_of::<TPacketGCPointChange>(), 17);
    assert_eq!(size_of::<TPacketGCPoints>(), 1024);
}

#[test]
fn test_parse_character_add() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let packet = TPacketGCCharacterAdd {
        header: 0x0205,
        length: size_of::<TPacketGCCharacterAdd>() as u16,
        dwVID: 1337,
        angle: 90.0,
        x: 1000,
        y: 2000,
        z: 3000,
        bType: 1,
        wRaceNum: 2,
        bMovingSpeed: 100,
        bAttackSpeed: 100,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
    };

    let slice = unsafe {
        std::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            size_of::<TPacketGCCharacterAdd>(),
        )
    };

    let result = parse_character_add(slice);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1337);

    let state = GLOBAL_GAME_STATE.read().unwrap();
    let actor = state.get_actor(1337).expect("Actor should exist in global state");
    assert_eq!(actor.vid, 1337);
    assert_eq!(actor.x, 1000.0);
    assert_eq!(actor.y, 2000.0);
    assert_eq!(actor.z, 3000.0);
    assert_eq!(actor.rot, 90.0);
    assert_eq!(actor.race, 2);
    assert_eq!(actor.b_type, 1);
}

#[test]
fn test_parse_point_change_player() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_vid(1337);

    let packet = TPacketGCPointChange {
        header: 0x0215,
        length: size_of::<TPacketGCPointChange>() as u16,
        dwVID: 1337,
        Type: POINT_HP,
        amount: 500,
        value: 1500,
    };

    let slice = unsafe {
        std::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            size_of::<TPacketGCPointChange>(),
        )
    };

    let result = parse_point_change(slice);
    assert!(result.is_ok());

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert_eq!(state.get_point(POINT_HP), 1500);
}

#[test]
fn test_parse_point_change_actor() {
    let _guard = TEST_MUTEX.lock().unwrap();
    GLOBAL_GAME_STATE.write().unwrap().reset();
    GLOBAL_GAME_STATE.write().unwrap().set_player_vid(9999);

    // Add an actor first
    let mut actor = m2_client_bridge::state::game_state::Actor::default();
    actor.vid = 1337;
    actor.hp = 100;
    GLOBAL_GAME_STATE.write().unwrap().add_or_update_actor(actor);

    let packet = TPacketGCPointChange {
        header: 0x0215,
        length: size_of::<TPacketGCPointChange>() as u16,
        dwVID: 1337, // Target the actor, not the player
        Type: POINT_HP,
        amount: -50,
        value: 50,
    };

    let slice = unsafe {
        std::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            size_of::<TPacketGCPointChange>(),
        )
    };

    let result = parse_point_change(slice);
    assert!(result.is_ok());

    let state = GLOBAL_GAME_STATE.read().unwrap();
    let updated_actor = state.get_actor(1337).unwrap();
    assert_eq!(updated_actor.hp, 50);
}
