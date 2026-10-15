use m2_client_bridge::api::actor_distance::m2_actor_get_distance;
use m2_client_bridge::api::nearest_target::m2_find_nearest_actor;
use m2_client_bridge::api::player_item::{m2_player_get_item_count, m2_player_get_item_vnum};
use m2_client_bridge::api::player_pos::m2_player_get_position;
use m2_client_bridge::api::player_status::m2_player_get_status;
use m2_client_bridge::state::dragon_soul::m2_ds_get_vnum;
use m2_client_bridge::state::equipment::m2_equipment_get_vnum;
use m2_client_bridge::state::game_state::{
    Actor, GameState, ItemData, PlayerState, GLOBAL_GAME_STATE, POINT_HP, POINT_MAX_HP,
};
use m2_client_bridge::state::player_stats::{m2_player_get_hp, m2_player_get_max_hp, m2_player_is_alive};
use std::ffi::c_void;

#[test]
fn test_item_data_default_and_new() {
    let item = ItemData::new(10, 5);
    assert_eq!(item.vnum, 10);
    assert_eq!(item.count, 5);
    assert_eq!(item.flags, 0);
    assert_eq!(item.anti_flags, 0);
    assert_eq!(item.sockets, [0; 3]);
    assert_eq!(item.attrs, [(0, 0); 7]);
}

#[test]
fn test_player_state_default() {
    let player = PlayerState::default();
    assert_eq!(player.vid, 0);
    assert_eq!(player.x, 0.0);
    assert_eq!(player.y, 0.0);
    assert_eq!(player.z, 0.0);
    assert_eq!(player.rot, 0.0);
    assert_eq!(player.points.len(), 256);
    assert!(player.inventory.is_empty());
    assert!(player.equipment.is_empty());
    assert!(player.dragon_soul.is_empty());
    assert_eq!(player.mount_vnum, 0);
    assert!(!player.is_mounted);
}

#[test]
fn test_actor_default() {
    let actor = Actor::default();
    assert_eq!(actor.vid, 0);
    assert_eq!(actor.race, 0);
    assert_eq!(actor.b_type, 0);
    assert_eq!(actor.hp, 0);
    assert_eq!(actor.max_hp, 0);
    assert!(actor.name.is_empty());
}

#[test]
fn test_game_state_player_pos() {
    let mut state = GameState::new();
    state.set_player_pos(123.5, 456.25, 78.0, 180.0);
    let (x, y, z, rot) = state.get_player_pos();
    assert_eq!(x, 123.5);
    assert_eq!(y, 456.25);
    assert_eq!(z, 78.0);
    assert_eq!(rot, 180.0);
}

#[test]
fn test_game_state_points() {
    let mut state = GameState::new();
    state.set_point(POINT_HP, 12500);
    state.set_point(POINT_MAX_HP, 15000);
    assert_eq!(state.get_point(POINT_HP), 12500);
    assert_eq!(state.get_point(POINT_MAX_HP), 15000);
    assert_eq!(state.get_point(100), 0);
}

#[test]
fn test_game_state_items() {
    let mut state = GameState::new();
    let item = ItemData {
        vnum: 309,
        count: 1,
        flags: 1,
        anti_flags: 2,
        sockets: [100, 200, 300],
        attrs: [(1, 50), (2, 100), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    };

    state.set_item(1, 0, item.clone());
    assert_eq!(state.get_item(1, 0), Some(&item));
    assert_eq!(state.get_item_vnum(1, 0), 309);
    assert_eq!(state.get_item_count(1, 0), 1);

    state.del_item(1, 0);
    assert_eq!(state.get_item(1, 0), None);
    assert_eq!(state.get_item_vnum(1, 0), 0);
    assert_eq!(state.get_item_count(1, 0), 0);
}

#[test]
fn test_game_state_actors_and_distances() {
    let mut state = GameState::new();
    state.set_player_pos(0.0, 0.0, 0.0, 0.0);

    let actor1 = Actor {
        vid: 101,
        x: 30.0,
        y: 40.0,
        z: 0.0,
        rot: 90.0,
        race: 101,
        b_type: 2, // potwor
        hp: 500,
        max_hp: 500,
        name: "Dziki Pies".to_string(),
        ..Default::default()
    };

    let actor2 = Actor {
        vid: 102,
        x: 6.0,
        y: 8.0,
        z: 0.0,
        rot: 0.0,
        race: 102,
        b_type: 2, // potwor
        hp: 300,
        max_hp: 300,
        name: "Wilk".to_string(),
        ..Default::default()
    };

    let actor3 = Actor {
        vid: 103,
        x: 3.0,
        y: 4.0,
        z: 0.0,
        rot: 45.0,
        race: 9001,
        b_type: 1, // npc
        hp: 1000,
        max_hp: 1000,
        name: "Handlarka".to_string(),
        ..Default::default()
    };

    state.add_or_update_actor(actor1);
    state.add_or_update_actor(actor2);
    state.add_or_update_actor(actor3);

    // Dystans do aktora 1: sqrt(30^2 + 40^2) = 50.0
    assert!((state.actor_distance(101) - 50.0).abs() < 1e-4);
    // Dystans do aktora 2: sqrt(6^2 + 8^2) = 10.0
    assert!((state.actor_distance(102) - 10.0).abs() < 1e-4);
    // Dystans do aktora 3: sqrt(3^2 + 4^2) = 5.0
    assert!((state.actor_distance(103) - 5.0).abs() < 1e-4);
    // Nieistniejacy aktor
    assert_eq!(state.actor_distance(999), 0.0);

    // Szukanie najblizszego typu 2 (potwor) w promieniu 100
    // Aktor 3 to b_type=1 (odpada), sposrod 101 (50.0) i 102 (10.0), wygrywa 102
    assert_eq!(state.find_nearest_actor(2, 100.0), 102);

    // Szukanie typu 2 w promieniu 8 (zaden potwor nie miesci sie w 8)
    assert_eq!(state.find_nearest_actor(2, 8.0), 0);

    // Szukanie typu 1 (npc) w promieniu 10
    assert_eq!(state.find_nearest_actor(1, 10.0), 103);

    // Szukanie dowolnego typu (actor_type = 0)
    assert_eq!(state.find_nearest_actor(0, 100.0), 103);

    // Usuniecie aktora
    state.remove_actor(103);
    assert_eq!(state.get_actor(103), None);
    assert_eq!(state.find_nearest_actor(1, 10.0), 0);
}

#[test]
fn test_ffi_integration_with_global_state() {
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.reset();

    // 1. Status i punkty
    state.set_point(POINT_HP, 8500);
    state.set_point(POINT_MAX_HP, 10000);
    state.set_point(15, 42);

    // 2. Pozycja gracza
    state.set_player_pos(111.0, 222.0, 333.0, 45.0);

    // 3. Przedmioty w ekwipunku
    state.set_item(1, 5, ItemData::new(50300, 3));

    // 4. Ekwipunek
    state.player.equipment.insert(4, 189); // bron: zatruty miecz

    // 5. Smocza alchemia
    state.player.dragon_soul.insert((0, 2), 60001);

    // 6. Aktor
    state.add_or_update_actor(Actor {
        vid: 2001,
        x: 114.0,
        y: 226.0,
        z: 333.0,
        rot: 0.0,
        race: 101,
        b_type: 2,
        hp: 50,
        max_hp: 50,
        name: "TestTarget".to_string(),
        ..Default::default()
    });
    drop(state);

    // Testy FFI
    // m2_player_get_status
    assert_eq!(m2_player_get_status(15), 42);
    assert_eq!(m2_player_get_status(POINT_HP as u32), 8500);

    // m2_player_get_position
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut rot: f32 = 0.0;
    let pos_ok = m2_player_get_position(&mut x, &mut y, &mut rot);
    assert!(pos_ok);
    assert_eq!(x, 111.0);
    assert_eq!(y, 222.0);
    assert_eq!(rot, 45.0);

    // m2_player_get_item_vnum / count
    assert_eq!(m2_player_get_item_vnum(1, 5), 50300);
    assert_eq!(m2_player_get_item_count(1, 5), 3);
    assert_eq!(m2_player_get_item_vnum(1, 99), 0);
    assert_eq!(m2_player_get_item_count(1, 99), 0);

    // m2_actor_get_distance: sqrt((114-111)^2 + (226-222)^2) = sqrt(9 + 16) = 5.0
    let vid_ptr = 2001 as *const c_void;
    let dist = m2_actor_get_distance(vid_ptr);
    assert!((dist - 5.0).abs() < 1e-4);

    // m2_find_nearest_actor
    assert_eq!(m2_find_nearest_actor(2, 10.0), 2001);
    assert_eq!(m2_find_nearest_actor(2, 3.0), 0);

    // m2_player_get_hp, get_max_hp, is_alive
    unsafe {
        assert_eq!(m2_player_get_hp(), 8500);
        assert_eq!(m2_player_get_max_hp(), 10000);
        assert!(m2_player_is_alive());
    }

    // m2_equipment_get_vnum
    unsafe {
        assert_eq!(m2_equipment_get_vnum(4), 189);
        assert_eq!(m2_equipment_get_vnum(99), 0);
    }

    // m2_ds_get_vnum
    unsafe {
        assert_eq!(m2_ds_get_vnum(0, 2), 60001);
        assert_eq!(m2_ds_get_vnum(1, 2), 0);
    }

    // Sprzatanie po tescie
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.reset();
}
