// Modul callbacks - rejestracja callbackow sieciowych i obsluga m2_net_observe_recv.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use std::os::raw::c_char;
use std::sync::RwLock;
use std::panic::{catch_unwind, AssertUnwindSafe};
use once_cell::sync::Lazy;

use crate::net::constants::{gc, PACKET_HEADER_SIZE};

pub const M2_NET_CALLBACKS: u32 = 0;

// Struktury pakietow GC zgodne z Packet.h z uzyciem #[repr(C, packed)]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCCharacterAdd {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub b_type: u8,
    pub w_race_num: u16,
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterAdd2 {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub name: [c_char; 65],
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub b_type: u8,
    pub w_race_num: u16,
    pub aw_part: [u16; 4],
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
    pub b_empire: u8,
    pub dw_guild: u32,
    pub s_alignment: i16,
    pub b_pk_mode: u8,
    pub dw_mount_vnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
    pub dw_duration: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCChat {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub dw_vid: u32,
    pub b_empire: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCDamageInfo {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub flag: u8,
    pub damage: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPoints {
    pub header: u16,
    pub length: u16,
    pub points: [i32; 255],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPointChange {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub point_type: u8,
    pub amount: i32,
    pub value: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCCharacterDelete {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
}

// Re-eksporty istniejacych struktur pakietow z podmodulow net/gc oraz ze stanu gry
pub use crate::net::gc::item_set::TPacketGCItemSet;
pub use crate::net::gc::item_del::TPacketGCItemDel;
pub use crate::state::game_state::{GLOBAL_GAME_STATE, GameState, GameStateActor, GameStateItem, PlayerState, Actor, ItemData};

#[repr(C)]
#[derive(Clone, Default)]
pub struct M2NetCallbacks {
    pub on_char_spawn: Option<extern "C" fn(vid: u32, x: i32, y: i32, race: u16)>,
    pub on_char_move: Option<extern "C" fn(vid: u32, x: i32, y: i32, rot: u8)>,
    pub on_chat: Option<extern "C" fn(chat_type: u8, sender_vid: u32, msg: *const c_char)>,
    pub on_item_set: Option<extern "C" fn(window: u8, cell: u16, vnum: u32, count: u8)>,
    pub on_damage: Option<extern "C" fn(victim_vid: u32, damage: i32, flag: u8)>,
}

static CALLBACKS: Lazy<RwLock<M2NetCallbacks>> = Lazy::new(|| RwLock::new(M2NetCallbacks::default()));

pub fn register_callbacks(cbs: M2NetCallbacks) {
    if let Ok(mut lock) = CALLBACKS.write() {
        *lock = cbs;
    }
}

pub fn dispatch_char_spawn(vid: u32, x: i32, y: i32, race: u16) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_char_spawn {
            cb(vid, x, y, race);
        }
    }
}

pub fn dispatch_char_move(vid: u32, x: i32, y: i32, rot: u8) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_char_move {
            cb(vid, x, y, rot);
        }
    }
}

pub fn dispatch_chat(chat_type: u8, sender_vid: u32, msg: *const c_char) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_chat {
            cb(chat_type, sender_vid, msg);
        }
    }
}

pub fn dispatch_item_set(window: u8, cell: u16, vnum: u32, count: u8) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_item_set {
            cb(window, cell, vnum, count);
        }
    }
}

pub fn dispatch_damage(victim_vid: u32, damage: i32, flag: u8) {
    if let Ok(lock) = CALLBACKS.read() {
        if let Some(cb) = lock.on_damage {
            cb(victim_vid, damage, flag);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_set_callbacks(cbs: *const M2NetCallbacks) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        if cbs.is_null() {
            return false;
        }
        let callbacks = (*cbs).clone();
        register_callbacks(callbacks);
        true
    })).unwrap_or(false)
}

static RECV_BYTES_TOTAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static RECV_PACKETS_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[no_mangle]
pub unsafe extern "C" fn m2_net_observe_recv(data: *const u8, len: usize) {
    catch_unwind(AssertUnwindSafe(|| {
        if data.is_null() || len < PACKET_HEADER_SIZE {
            return;
        }
        let slice = std::slice::from_raw_parts(data, len);
        let total = RECV_BYTES_TOTAL.fetch_add(len, std::sync::atomic::Ordering::Relaxed) + len;
        let pkt_idx = RECV_PACKETS_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

        let mut offset = 0;
        while offset + PACKET_HEADER_SIZE <= len {
            let header = u16::from_le_bytes([slice[offset], slice[offset + 1]]);
            let length = u16::from_le_bytes([slice[offset + 2], slice[offset + 3]]) as usize;

            if length < PACKET_HEADER_SIZE || offset + length > len {
                break;
            }

            let pkt_bytes = &slice[offset..offset + length];

            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("rust_bridge_live.log") {
                let _ = writeln!(f, "[RUST NET RECV] Packet Batch #{}: offset={} len={} B, opcode=0x{:04X}, total_rx={} B",
                    pkt_idx, offset, length, header, total);
            }

            match header {
                gc::CHARACTER_ADD => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCCharacterAdd>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCCharacterAdd);
                        let actor = Actor {
                            vid: pkt.dw_vid,
                            x: pkt.x as f32,
                            y: pkt.y as f32,
                            z: pkt.z as f32,
                            rot: pkt.angle,
                            race: pkt.w_race_num,
                            b_type: pkt.b_type,
                            hp: 0,
                            max_hp: 0,
                            name: String::new(),
                        };
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.add_or_update_actor(actor);
                        }
                        dispatch_char_spawn(pkt.dw_vid, pkt.x, pkt.y, pkt.w_race_num);
                    }
                }
                gc::CHARACTER_ADD2 => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCCharacterAdd2>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCCharacterAdd2);
                        let mut name_bytes = Vec::new();
                        for &c in &pkt.name {
                            if c == 0 {
                                break;
                            }
                            name_bytes.push(c as u8);
                        }
                        let name = String::from_utf8_lossy(&name_bytes).to_string();
                        let actor = Actor {
                            vid: pkt.dw_vid,
                            x: pkt.x as f32,
                            y: pkt.y as f32,
                            z: pkt.z as f32,
                            rot: pkt.angle,
                            race: pkt.w_race_num,
                            b_type: pkt.b_type,
                            hp: 0,
                            max_hp: 0,
                            name,
                        };
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.add_or_update_actor(actor);
                        }
                        dispatch_char_spawn(pkt.dw_vid, pkt.x, pkt.y, pkt.w_race_num);
                    }
                }
                gc::MOVE => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCMove>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCMove);
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.update_actor_move(pkt.dw_vid, pkt.l_x, pkt.l_y, pkt.b_rot);
                        }
                        dispatch_char_move(pkt.dw_vid, pkt.l_x, pkt.l_y, pkt.b_rot);
                    }
                }
                gc::CHAT => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCChat>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCChat);
                        let chat_header_size = std::mem::size_of::<TPacketGCChat>();
                        let msg_bytes = &pkt_bytes[chat_header_size..];
                        let mut c_str_buf = Vec::with_capacity(msg_bytes.len() + 1);
                        for &b in msg_bytes {
                            if b == 0 {
                                break;
                            }
                            c_str_buf.push(b);
                        }
                        c_str_buf.push(0);
                        dispatch_chat(pkt.b_type, pkt.dw_vid, c_str_buf.as_ptr() as *const c_char);
                    }
                }
                gc::DAMAGE_INFO => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCDamageInfo>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCDamageInfo);
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.apply_damage(pkt.dw_vid, pkt.damage);
                        }
                        dispatch_damage(pkt.dw_vid, pkt.damage, pkt.flag);
                    }
                }
                gc::PLAYER_POINTS => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCPoints>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCPoints);
                        let points = pkt.points;
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.set_points(&points);
                        }
                    }
                }
                gc::PLAYER_POINT_CHANGE => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCPointChange>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCPointChange);
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.set_point(pkt.point_type, pkt.value as i64);
                        }
                    }
                }
                gc::ITEM_SET => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCItemSet>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCItemSet);
                        let window = pkt.pos.window_type;
                        let cell = pkt.pos.cell;
                        let vnum = pkt.vnum;
                        let count = pkt.count;
                        let flags = pkt.flags;
                        let anti_flags = pkt.anti_flags;
                        let sockets = pkt.alSockets;
                        let mut attrs = [(0u8, 0i16); 7];
                        for i in 0..7 {
                            attrs[i] = (pkt.aAttr[i].bType, pkt.aAttr[i].sValue);
                        }
                        let item = ItemData {
                            vnum,
                            count,
                            flags,
                            anti_flags,
                            sockets,
                            attrs,
                        };
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.set_item(window, cell, item);
                        }
                        dispatch_item_set(window, cell, vnum, count);
                    }
                }
                gc::ITEM_DEL => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCItemDel>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCItemDel);
                        let window = pkt.pos.window_type;
                        let cell = pkt.pos.cell;
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.del_item(window, cell);
                        }
                    }
                }
                gc::CHARACTER_DELETE | gc::CHARACTER_DEL => {
                    if pkt_bytes.len() >= std::mem::size_of::<TPacketGCCharacterDelete>() {
                        let pkt = std::ptr::read_unaligned(pkt_bytes.as_ptr() as *const TPacketGCCharacterDelete);
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.remove_actor(pkt.dw_vid);
                        }
                    }
                }
                gc::MAIN_CHARACTER => {
                    if pkt_bytes.len() >= 8 {
                        let vid = u32::from_le_bytes([pkt_bytes[4], pkt_bytes[5], pkt_bytes[6], pkt_bytes[7]]);
                        if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                            state.set_player_vid(vid);
                        }
                    }
                }
                _ => {}
            }

            offset += length;
        }
    })).unwrap_or(());
}
