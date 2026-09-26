// Test integracyjny: weryfikacja parsera m2_net_observe_recv oraz synchronizacji z GLOBAL_GAME_STATE.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU8, Ordering};
use std::os::raw::c_char;

use m2_client_bridge::net::callbacks::*;
use m2_client_bridge::net::constants::gc;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_HP, POINT_MAX_HP};

static LAST_SPAWN_VID: AtomicU32 = AtomicU32::new(0);
static LAST_MOVE_VID: AtomicU32 = AtomicU32::new(0);
static LAST_ITEM_VNUM: AtomicU32 = AtomicU32::new(0);
static LAST_DAMAGE_VID: AtomicU32 = AtomicU32::new(0);
static LAST_DAMAGE_VAL: AtomicI32 = AtomicI32::new(0);
static LAST_CHAT_TYPE: AtomicU8 = AtomicU8::new(0);

extern "C" fn mock_on_char_spawn(vid: u32, _x: i32, _y: i32, _race: u16) {
    LAST_SPAWN_VID.store(vid, Ordering::SeqCst);
}

extern "C" fn mock_on_char_move(vid: u32, _x: i32, _y: i32, _rot: u8) {
    LAST_MOVE_VID.store(vid, Ordering::SeqCst);
}

extern "C" fn mock_on_item_set(_window: u8, _cell: u16, vnum: u32, _count: u8) {
    LAST_ITEM_VNUM.store(vnum, Ordering::SeqCst);
}

extern "C" fn mock_on_damage(victim_vid: u32, damage: i32, _flag: u8) {
    LAST_DAMAGE_VID.store(victim_vid, Ordering::SeqCst);
    LAST_DAMAGE_VAL.store(damage, Ordering::SeqCst);
}

extern "C" fn mock_on_chat(chat_type: u8, _sender_vid: u32, _msg: *const c_char) {
    LAST_CHAT_TYPE.store(chat_type, Ordering::SeqCst);
}

static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn setup_callbacks() {
    let mut cbs = M2NetCallbacks::default();
    cbs.on_char_spawn = Some(mock_on_char_spawn);
    cbs.on_char_move = Some(mock_on_char_move);
    cbs.on_item_set = Some(mock_on_item_set);
    cbs.on_damage = Some(mock_on_damage);
    cbs.on_chat = Some(mock_on_chat);
    unsafe {
        assert!(m2_net_set_callbacks(&cbs));
    }
}

#[test]
fn test_observe_player_points() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let mut points = [0i32; 255];
    points[POINT_HP as usize] = 12500;
    points[POINT_MAX_HP as usize] = 15000;
    points[11] = 999999; // GOLD

    let header: u16 = gc::PLAYER_POINTS;
    let length: u16 = (4 + 255 * 4) as u16;

    let mut buf = Vec::new();
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&length.to_le_bytes());
    for p in points {
        buf.extend_from_slice(&p.to_le_bytes());
    }

    unsafe {
        m2_net_observe_recv(buf.as_ptr(), buf.len());
    }

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert_eq!(state.get_point(POINT_HP), 12500);
    assert_eq!(state.get_point(POINT_MAX_HP), 15000);
    assert_eq!(state.get_gold(), 999999);
}

#[test]
fn test_observe_player_point_change() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let header: u16 = gc::PLAYER_POINT_CHANGE;
    let length: u16 = 17;
    let dw_vid: u32 = 12345;
    let point_type: u8 = POINT_HP;
    let amount: i32 = 500;
    let value: i32 = 8000;

    let mut buf = Vec::new();
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&length.to_le_bytes());
    buf.extend_from_slice(&dw_vid.to_le_bytes());
    buf.push(point_type);
    buf.extend_from_slice(&amount.to_le_bytes());
    buf.extend_from_slice(&value.to_le_bytes());

    unsafe {
        m2_net_observe_recv(buf.as_ptr(), buf.len());
    }

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert_eq!(state.get_point(POINT_HP), 8000);
}

#[test]
fn test_observe_item_set_and_del() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    let header: u16 = gc::ITEM_SET;
    let length: u16 = 54;
    let window: u8 = 1;
    let cell: u16 = 5;
    let vnum: u32 = 309; // Miecz Pelni Ksiezyca (FMS)
    let count: u8 = 1;
    let flags: u32 = 0;
    let anti_flags: u32 = 0;
    let highlight: u8 = 0;
    let sockets = [101i32, 102i32, 0i32];

    let mut buf = Vec::new();
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&length.to_le_bytes());
    buf.push(window);
    buf.extend_from_slice(&cell.to_le_bytes());
    buf.extend_from_slice(&vnum.to_le_bytes());
    buf.push(count);
    buf.extend_from_slice(&flags.to_le_bytes());
    buf.extend_from_slice(&anti_flags.to_le_bytes());
    buf.push(highlight);
    for s in sockets {
        buf.extend_from_slice(&s.to_le_bytes());
    }
    // 7 bonusow (1B type, 2B value = 3B kazdy, lacznie 21B)
    for i in 0..7u8 {
        buf.push(i);
        buf.extend_from_slice(&(10i16 * (i as i16)).to_le_bytes());
    }

    assert_eq!(buf.len(), 54);

    unsafe {
        m2_net_observe_recv(buf.as_ptr(), buf.len());
    }

    assert_eq!(LAST_ITEM_VNUM.load(Ordering::SeqCst), 309);
    {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        assert_eq!(state.get_item_vnum(window, cell), 309);
        assert_eq!(state.get_item_count(window, cell), 1);
        let item = state.get_item(window, cell).unwrap();
        assert_eq!(item.sockets[0], 101);
        assert_eq!(item.sockets[1], 102);
    }

    // Usuniecie przedmiotu pakietem ITEM_DEL
    let del_header: u16 = gc::ITEM_DEL;
    let del_length: u16 = 7;
    let mut del_buf = Vec::new();
    del_buf.extend_from_slice(&del_header.to_le_bytes());
    del_buf.extend_from_slice(&del_length.to_le_bytes());
    del_buf.push(window);
    del_buf.extend_from_slice(&cell.to_le_bytes());

    assert_eq!(del_buf.len(), 7);

    unsafe {
        m2_net_observe_recv(del_buf.as_ptr(), del_buf.len());
    }

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert_eq!(state.get_item_vnum(window, cell), 0);
    assert_eq!(state.get_item(window, cell), None);
}

#[test]
fn test_observe_character_lifecycle_and_movement() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    // 1. CHARACTER_ADD (0x0205)
    let add_header: u16 = gc::CHARACTER_ADD;
    let add_length: u16 = 38;
    let vid: u32 = 777;
    let angle: f32 = 90.0;
    let x: i32 = 1000;
    let y: i32 = 2000;
    let z: i32 = 0;
    let b_type: u8 = 1;
    let race_num: u16 = 101;
    let mov_spd: u8 = 100;
    let atk_spd: u8 = 100;
    let state_flag: u8 = 0;
    let affect_flag = [0u32, 0u32];

    let mut add_buf = Vec::new();
    add_buf.extend_from_slice(&add_header.to_le_bytes());
    add_buf.extend_from_slice(&add_length.to_le_bytes());
    add_buf.extend_from_slice(&vid.to_le_bytes());
    add_buf.extend_from_slice(&angle.to_le_bytes());
    add_buf.extend_from_slice(&x.to_le_bytes());
    add_buf.extend_from_slice(&y.to_le_bytes());
    add_buf.extend_from_slice(&z.to_le_bytes());
    add_buf.push(b_type);
    add_buf.extend_from_slice(&race_num.to_le_bytes());
    add_buf.push(mov_spd);
    add_buf.push(atk_spd);
    add_buf.push(state_flag);
    add_buf.extend_from_slice(&affect_flag[0].to_le_bytes());
    add_buf.extend_from_slice(&affect_flag[1].to_le_bytes());

    assert_eq!(add_buf.len(), 38);

    unsafe {
        m2_net_observe_recv(add_buf.as_ptr(), add_buf.len());
    }

    assert_eq!(LAST_SPAWN_VID.load(Ordering::SeqCst), 777);
    {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let actor = state.get_actor(777).unwrap();
        assert_eq!(actor.vid, 777);
        assert_eq!(actor.x, 1000.0);
        assert_eq!(actor.y, 2000.0);
        assert_eq!(actor.race, 101);
    }

    // 2. MOVE (0x0302)
    let move_header: u16 = gc::MOVE;
    let move_length: u16 = 27;
    let b_func: u8 = 1;
    let b_arg: u8 = 0;
    let b_rot: u8 = 18; // 18 * 5 = 90 deg
    let new_x: i32 = 1500;
    let new_y: i32 = 2500;
    let dw_time: u32 = 1000;
    let dw_duration: u32 = 500;

    let mut move_buf = Vec::new();
    move_buf.extend_from_slice(&move_header.to_le_bytes());
    move_buf.extend_from_slice(&move_length.to_le_bytes());
    move_buf.push(b_func);
    move_buf.push(b_arg);
    move_buf.push(b_rot);
    move_buf.extend_from_slice(&vid.to_le_bytes());
    move_buf.extend_from_slice(&new_x.to_le_bytes());
    move_buf.extend_from_slice(&new_y.to_le_bytes());
    move_buf.extend_from_slice(&dw_time.to_le_bytes());
    move_buf.extend_from_slice(&dw_duration.to_le_bytes());

    assert_eq!(move_buf.len(), 27);

    unsafe {
        m2_net_observe_recv(move_buf.as_ptr(), move_buf.len());
    }

    assert_eq!(LAST_MOVE_VID.load(Ordering::SeqCst), 777);
    {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let actor = state.get_actor(777).unwrap();
        assert_eq!(actor.x, 1500.0);
        assert_eq!(actor.y, 2500.0);
        assert_eq!(actor.rot, 90.0);
    }

    // 3. CHARACTER_DELETE (0x020A)
    let del_header: u16 = gc::CHARACTER_DELETE;
    let del_length: u16 = 8;
    let mut del_buf = Vec::new();
    del_buf.extend_from_slice(&del_header.to_le_bytes());
    del_buf.extend_from_slice(&del_length.to_le_bytes());
    del_buf.extend_from_slice(&vid.to_le_bytes());

    assert_eq!(del_buf.len(), 8);

    unsafe {
        m2_net_observe_recv(del_buf.as_ptr(), del_buf.len());
    }

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert!(state.get_actor(777).is_none());
}

#[test]
fn test_observe_damage_info() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();
    GLOBAL_GAME_STATE.write().unwrap().reset();

    // Ustawienie bazowego HP na 5000
    GLOBAL_GAME_STATE.write().unwrap().set_point(POINT_HP, 5000);

    let header: u16 = gc::DAMAGE_INFO;
    let length: u16 = 13;
    let dw_vid: u32 = 0; // Wlasny gracz (player_vid == 0)
    let flag: u8 = 1;
    let damage: i32 = 1200;

    let mut buf = Vec::new();
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&length.to_le_bytes());
    buf.extend_from_slice(&dw_vid.to_le_bytes());
    buf.push(flag);
    buf.extend_from_slice(&damage.to_le_bytes());

    assert_eq!(buf.len(), 13);

    unsafe {
        m2_net_observe_recv(buf.as_ptr(), buf.len());
    }

    assert_eq!(LAST_DAMAGE_VID.load(Ordering::SeqCst), 0);
    assert_eq!(LAST_DAMAGE_VAL.load(Ordering::SeqCst), 1200);

    let state = GLOBAL_GAME_STATE.read().unwrap();
    assert_eq!(state.get_hp(), 3800);
}

#[test]
fn test_observe_chat() {
    let _lock = TEST_MUTEX.lock().unwrap();
    setup_callbacks();

    let header: u16 = gc::CHAT;
    let chat_text = "Witaj w swiecie gry!\0";
    let chat_bytes = chat_text.as_bytes();
    let length: u16 = (8 + chat_bytes.len()) as u16;
    let b_type: u8 = 2; // CHAT_TYPE_INFO
    let dw_vid: u32 = 999;
    let b_empire: u8 = 1;

    let mut buf = Vec::new();
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(&length.to_le_bytes());
    buf.push(b_type);
    buf.extend_from_slice(&dw_vid.to_le_bytes());
    buf.push(b_empire);
    buf.extend_from_slice(chat_bytes);

    unsafe {
        m2_net_observe_recv(buf.as_ptr(), buf.len());
    }

    assert_eq!(LAST_CHAT_TYPE.load(Ordering::SeqCst), 2);
}
