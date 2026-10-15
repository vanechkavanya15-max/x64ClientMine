use m2_client_bridge::api::nearest_target::m2_find_nearest_actor;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, Actor};

#[test]
fn test_m2_find_nearest_actor_empty_state() {
    // Gdy brak pasujacych aktorow w stanie gry, funkcja zwraca 0
    let vid = m2_find_nearest_actor(99, 100.0);
    assert_eq!(vid, 0, "m2_find_nearest_actor powinno zwracac 0 gdy brak aktora");
}

#[test]
fn test_m2_find_nearest_actor_with_actor() {
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.set_player_pos(100.0, 100.0, 0.0, 0.0);
    state.add_or_update_actor(Actor {
        vid: 555,
        x: 103.0,
        y: 104.0,
        z: 0.0,
        rot: 0.0,
        race: 101,
        b_type: 2,
        hp: 100,
        max_hp: 100,
        name: "Wild Dog".to_string(),
        ..Default::default()
    });
    drop(state);

    let vid = m2_find_nearest_actor(2, 10.0);
    assert_eq!(vid, 555);

    // Poza zasiegiem max_distance
    let vid_far = m2_find_nearest_actor(2, 3.0);
    assert_eq!(vid_far, 0);

    // Czyszczenie
    let mut state = GLOBAL_GAME_STATE.write().unwrap();
    state.remove_actor(555);
}
