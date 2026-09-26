// Testy modulu sieciowego: constants, stream_reader i callbacks (dispatcher).
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use m2_client_bridge::net::constants::{cg, gc, PACKET_HEADER_SIZE};
use m2_client_bridge::net::stream_reader::PacketStreamReader;
use m2_client_bridge::net::callbacks::{
    m2_net_set_callbacks, m2_net_observe_recv, M2NetCallbacks,
    TPacketGCCharacterAdd, TPacketGCMove, TPacketGCChat, TPacketGCDamageInfo,
};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicU32, AtomicI32, AtomicU8, AtomicU16, Ordering};

#[test]
fn test_opcodes_constants() {
    assert_eq!(PACKET_HEADER_SIZE, 4);

    // Weryfikacja wybranych opcode CG
    assert_eq!(cg::PONG, 0x0006);
    assert_eq!(cg::LOGIN2, 0x0101);
    assert_eq!(cg::CHARACTER_CREATE, 0x0201);
    assert_eq!(cg::MOVE, 0x0301);
    assert_eq!(cg::ATTACK, 0x0401);
    assert_eq!(cg::ITEM_USE, 0x0501);
    assert_eq!(cg::CHAT, 0x0601);

    // Weryfikacja wybranych opcode GC
    assert_eq!(gc::PING, 0x0007);
    assert_eq!(gc::PHASE, 0x0008);
    assert_eq!(gc::AUTH_SUCCESS, 0x0108);
    assert_eq!(gc::CHARACTER_ADD, 0x0205);
    assert_eq!(gc::CHARACTER_ADD2, 0x0206);
    assert_eq!(gc::CHARACTER_DEL, 0x0208);
    assert_eq!(gc::MOVE, 0x0302);
    assert_eq!(gc::DAMAGE_INFO, 0x0410);
    assert_eq!(gc::ITEM_SET, 0x0511);
    assert_eq!(gc::CHAT, 0x0603);
}

#[test]
fn test_stream_reader_dynamic_length() {
    let mut reader = PacketStreamReader::new();

    // Tworzymy pakiet o dynamicznej dlugosci 20 bajtow
    let header_val: u16 = gc::CHAT;
    let length_val: u16 = 20;
    let mut pkt1 = Vec::new();
    pkt1.extend_from_slice(&header_val.to_le_bytes());
    pkt1.extend_from_slice(&length_val.to_le_bytes());
    pkt1.extend_from_slice(&[0xAA; 16]); // 16 bajtow payloadu

    // Drugi pakiet o dlugosci 10 bajtow
    let header2_val: u16 = gc::DAMAGE_INFO;
    let length2_val: u16 = 10;
    let mut pkt2 = Vec::new();
    pkt2.extend_from_slice(&header2_val.to_le_bytes());
    pkt2.extend_from_slice(&length2_val.to_le_bytes());
    pkt2.extend_from_slice(&[0xBB; 6]); // 6 bajtow payloadu

    // Podajemy czesc pierwszego pakietu (15 bajtow)
    reader.append_bytes(&pkt1[0..15]);
    assert!(reader.read_next_packet().is_none(), "Pakiet jeszcze niepelny");

    // Podajemy reszte pierwszego i caly drugi pakiet
    let mut remaining = pkt1[15..20].to_vec();
    remaining.extend_from_slice(&pkt2);
    reader.append_bytes(&remaining);

    // Odczyt pierwszego pakietu
    let res1 = reader.read_next_packet();
    assert!(res1.is_some());
    assert_eq!(res1.unwrap(), pkt1);

    // Odczyt drugiego pakietu
    let res2 = reader.read_next_packet();
    assert!(res2.is_some());
    assert_eq!(res2.unwrap(), pkt2);

    // Brak kolejnych pakietow
    assert!(reader.read_next_packet().is_none());
}

static SPAWN_VID: AtomicU32 = AtomicU32::new(0);
static SPAWN_X: AtomicI32 = AtomicI32::new(0);
static SPAWN_Y: AtomicI32 = AtomicI32::new(0);
static SPAWN_RACE: AtomicU16 = AtomicU16::new(0);

static MOVE_VID: AtomicU32 = AtomicU32::new(0);
static MOVE_X: AtomicI32 = AtomicI32::new(0);
static MOVE_Y: AtomicI32 = AtomicI32::new(0);
static MOVE_ROT: AtomicU8 = AtomicU8::new(0);

static DAMAGE_VID: AtomicU32 = AtomicU32::new(0);
static DAMAGE_VAL: AtomicI32 = AtomicI32::new(0);
static DAMAGE_FLAG: AtomicU8 = AtomicU8::new(0);

static CHAT_TYPE: AtomicU8 = AtomicU8::new(0);
static CHAT_VID: AtomicU32 = AtomicU32::new(0);
static CHAT_LEN: AtomicU32 = AtomicU32::new(0);

extern "C" fn test_on_char_spawn(vid: u32, x: i32, y: i32, race: u16) {
    SPAWN_VID.store(vid, Ordering::SeqCst);
    SPAWN_X.store(x, Ordering::SeqCst);
    SPAWN_Y.store(y, Ordering::SeqCst);
    SPAWN_RACE.store(race, Ordering::SeqCst);
}

extern "C" fn test_on_char_move(vid: u32, x: i32, y: i32, rot: u8) {
    MOVE_VID.store(vid, Ordering::SeqCst);
    MOVE_X.store(x, Ordering::SeqCst);
    MOVE_Y.store(y, Ordering::SeqCst);
    MOVE_ROT.store(rot, Ordering::SeqCst);
}

extern "C" fn test_on_damage(victim_vid: u32, damage: i32, flag: u8) {
    DAMAGE_VID.store(victim_vid, Ordering::SeqCst);
    DAMAGE_VAL.store(damage, Ordering::SeqCst);
    DAMAGE_FLAG.store(flag, Ordering::SeqCst);
}

extern "C" fn test_on_chat(chat_type: u8, sender_vid: u32, msg: *const c_char) {
    CHAT_TYPE.store(chat_type, Ordering::SeqCst);
    CHAT_VID.store(sender_vid, Ordering::SeqCst);
    if !msg.is_null() {
        let c_str = unsafe { CStr::from_ptr(msg) };
        CHAT_LEN.store(c_str.to_bytes().len() as u32, Ordering::SeqCst);
    }
}

#[test]
fn test_m2_net_observe_recv_dispatcher() {
    let callbacks = M2NetCallbacks {
        on_char_spawn: Some(test_on_char_spawn),
        on_char_move: Some(test_on_char_move),
        on_chat: Some(test_on_chat),
        on_item_set: None,
        on_damage: Some(test_on_damage),
    };

    unsafe {
        m2_net_set_callbacks(&callbacks);
    }

    // 1. Pakiet GC::CHARACTER_ADD
    let char_add = TPacketGCCharacterAdd {
        header: gc::CHARACTER_ADD,
        length: std::mem::size_of::<TPacketGCCharacterAdd>() as u16,
        dw_vid: 5555,
        angle: 90.0,
        x: 123400,
        y: 567800,
        z: 100,
        b_type: 1,
        w_race_num: 101,
        b_moving_speed: 150,
        b_attack_speed: 120,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
    };

    let char_add_bytes = unsafe {
        std::slice::from_raw_parts(
            &char_add as *const _ as *const u8,
            std::mem::size_of::<TPacketGCCharacterAdd>(),
        )
    };
    unsafe {
        m2_net_observe_recv(char_add_bytes.as_ptr(), char_add_bytes.len());
    }

    assert_eq!(SPAWN_VID.load(Ordering::SeqCst), 5555);
    assert_eq!(SPAWN_X.load(Ordering::SeqCst), 123400);
    assert_eq!(SPAWN_Y.load(Ordering::SeqCst), 567800);
    assert_eq!(SPAWN_RACE.load(Ordering::SeqCst), 101);

    // 2. Pakiet GC::MOVE
    let char_move = TPacketGCMove {
        header: gc::MOVE,
        length: std::mem::size_of::<TPacketGCMove>() as u16,
        b_func: 2,
        b_arg: 0,
        b_rot: 180,
        dw_vid: 7777,
        l_x: 200000,
        l_y: 300000,
        dw_time: 123456,
        dw_duration: 500,
    };

    let char_move_bytes = unsafe {
        std::slice::from_raw_parts(
            &char_move as *const _ as *const u8,
            std::mem::size_of::<TPacketGCMove>(),
        )
    };
    unsafe {
        m2_net_observe_recv(char_move_bytes.as_ptr(), char_move_bytes.len());
    }

    assert_eq!(MOVE_VID.load(Ordering::SeqCst), 7777);
    assert_eq!(MOVE_X.load(Ordering::SeqCst), 200000);
    assert_eq!(MOVE_Y.load(Ordering::SeqCst), 300000);
    assert_eq!(MOVE_ROT.load(Ordering::SeqCst), 180);

    // 3. Pakiet GC::DAMAGE_INFO
    let damage_info = TPacketGCDamageInfo {
        header: gc::DAMAGE_INFO,
        length: std::mem::size_of::<TPacketGCDamageInfo>() as u16,
        dw_vid: 8888,
        flag: 3,
        damage: 1542,
    };

    let damage_bytes = unsafe {
        std::slice::from_raw_parts(
            &damage_info as *const _ as *const u8,
            std::mem::size_of::<TPacketGCDamageInfo>(),
        )
    };
    unsafe {
        m2_net_observe_recv(damage_bytes.as_ptr(), damage_bytes.len());
    }

    assert_eq!(DAMAGE_VID.load(Ordering::SeqCst), 8888);
    assert_eq!(DAMAGE_VAL.load(Ordering::SeqCst), 1542);
    assert_eq!(DAMAGE_FLAG.load(Ordering::SeqCst), 3);

    // 4. Pakiet GC::CHAT
    let chat_text = b"Witaj w grze!";
    let chat_hdr_len = std::mem::size_of::<TPacketGCChat>();
    let total_chat_len = (chat_hdr_len + chat_text.len()) as u16;
    let chat_hdr = TPacketGCChat {
        header: gc::CHAT,
        length: total_chat_len,
        b_type: 2,
        dw_vid: 9999,
        b_empire: 1,
    };

    let mut chat_buf = Vec::new();
    let chat_hdr_slice = unsafe {
        std::slice::from_raw_parts(
            &chat_hdr as *const _ as *const u8,
            chat_hdr_len,
        )
    };
    chat_buf.extend_from_slice(chat_hdr_slice);
    chat_buf.extend_from_slice(chat_text);

    unsafe {
        m2_net_observe_recv(chat_buf.as_ptr(), chat_buf.len());
    }

    assert_eq!(CHAT_TYPE.load(Ordering::SeqCst), 2);
    assert_eq!(CHAT_VID.load(Ordering::SeqCst), 9999);
    assert_eq!(CHAT_LEN.load(Ordering::SeqCst), chat_text.len() as u32);
}
