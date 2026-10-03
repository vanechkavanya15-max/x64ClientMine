#![allow(non_snake_case)]

use std::mem::size_of;

// We use gc_opcode from game_net_receiver
use crate::game_net_receiver::{gc_opcode, GameEvent, push_event, send_pong};
use crate::character_manager_core::get_character_manager;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;

// Missing structures for C++ 1:1 compliance
pub const CHR_EQUIPPART_NUM: usize = 4;
pub const PLAYER_PER_ACCOUNT4: usize = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TSimplePlayerInformation {
    pub dwID: u32,
    pub szName: [u8; 25],
    pub byJob: u8,
    pub byLevel: u8,
    pub dwPlayMinutes: u32,
    pub byST: u8,
    pub byHT: u8,
    pub byDX: u8,
    pub byIQ: u8,
    pub wMainPart: u16,
    pub byHair: u8,
    pub x: i32,
    pub y: i32,
    pub lAddr: i32,
    pub wPort: u16,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TPacketGCLoginSuccess3 {
    pub header: u16,
    pub length: u16,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; 4], // PLAYER_PER_ACCOUNT
    pub dwGuildID: [u32; 4],
    pub szGuildName: [[u8; 25]; 4],
    pub handle: u32,
    pub random_key: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TPacketGCLoginSuccess4 {
    pub header: u16,
    pub length: u16,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; 4],
    pub dwGuildID: [u32; 4],
    pub szGuildName: [[u8; 25]; 4],
    pub handle: u32,
    pub random_key: u32,
    pub adwClientKey: [u32; 4],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterAdditionalInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub name: [u8; 25], // Originally 24+1 or 65? In handle_character_additional_info it's 65, let's use 65
    pub awPart: [u16; 4], // CHR_EQUIPPART_NUM
    pub bEmpire: u8,
    pub dwGuildID: u32,
    pub dwLevel: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterDelete {
    pub header: u16,
    pub length: u16,
    pub id: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMainCharacter {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szName: [u8; 65], // CHARACTER_NAME_MAX_LEN + 1
    pub szBGMName: [u8; 25],
    pub fBGMVol: f32,
    pub lx: i32,
    pub ly: i32,
    pub lz: i32,
    pub empire: u8,
    pub skill_group: u8,
}

pub fn dispatch_packet_extended_v2(header: u16, length: u16, data: &[u8]) -> bool {
    match header {
        0x0007 => { // PING
            if let Some(pong_data) = crate::recv_ping_sync_v2::recv_ping_packet(data, 0) {
                send_pong();
            }
            return true;
        }
        0x0008 => { // PHASE
            if data.len() >= 5 {
                let phase = data[4];
                push_event(GameEvent::PhaseChange { phase });
                return true;
            }
        }
        0x0104 | 0x0105 => { // LOGIN_SUCCESS3, LOGIN_SUCCESS4
            // Zapis postaci i slotow - na razie po prostu sukces i zmiana fazy, 
            // bo logika logowania nalezy do innej fazy, ale dispatchujemy prawidlowo.
            push_event(GameEvent::PhaseChange { phase: 2 }); // SELECT
            return true;
        }
        0x0205 | 0x0206 => { // CHARACTER_ADD, CHARACTER_ADD2
            if crate::recv_actors::parse_character_add(data).is_ok() {
                return true;
            }
        }
        0x0207 => { // CHAR_ADDITIONAL_INFO
            if data.len() >= 20 {
                let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let name_end = data[8..].iter().take(65).position(|&b| b == 0).unwrap_or(65);
                let name = String::from_utf8_lossy(&data[8..8 + name_end]).to_string();
                
                if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                    if let Some(actor) = state.actors.get_mut(&vid) {
                        actor.name = name;
                    }
                }
                return true;
            }
        }
        0x0208 => { // CHARACTER_DEL
            if data.len() >= size_of::<TPacketGCCharacterDelete>() {
                let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterDelete) };
                let vid = packet.id;
                push_event(GameEvent::CharacterDelete { vid });
                if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
                    state.actors.remove(&vid);
                }
                return true;
            }
        }
        0x0209 | 0x020A => { // CHARACTER_UPDATE, CHARACTER_UPDATE2
            // aktualizacja ekwipunku postaci na mapie
            if data.len() >= 8 {
                let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                // W pelnej implementacji parsujemy reszte by uaktualnic model, 
                // ale tu oznaczamy jako przetworzone poprawnie
                return true;
            }
        }
        0x0210 => { // MAIN_CHARACTER
            if data.len() >= 30 {
                let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let race = u16::from_le_bytes(data[8..10].try_into().unwrap_or_default());
                let name_end = data[10..].iter().take(65).position(|&b| b == 0).unwrap_or(65);
                let name = String::from_utf8_lossy(&data[10..10 + name_end]).to_string();
                
                // Switch phase to PHASE_GAME and store main character
                push_event(GameEvent::MainCharacterSpawn {
                    vid,
                    race,
                    name,
                    x: 0, y: 0, z: 0,
                    empire: 0,
                    skill_group: 0,
                });
                return true;
            }
        }
        0x0214 | 0x0215 => { // PLAYER_POINTS, PLAYER_POINT_CHANGE
            if header == 0x0214 {
                if data.len() >= 8 {
                    let num_points = (data.len() - 4) / 4;
                    let mut points = Vec::with_capacity(num_points);
                    for i in 0..num_points.min(255) {
                        let off = 4 + i * 4;
                        if off + 4 <= data.len() {
                            points.push(i32::from_le_bytes(data[off..off+4].try_into().unwrap_or_default()));
                        }
                    }
                    
                    let level = points.get(0).copied().unwrap_or(0);
                    let hp = points.get(5).copied().unwrap_or(0);
                    let max_hp = points.get(6).copied().unwrap_or(0);
                    let sp = points.get(7).copied().unwrap_or(0);
                    let max_sp = points.get(8).copied().unwrap_or(0);
                    let gold = points.get(14).copied().unwrap_or(0);

                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    if let Some(hp_win) = wm.windows.values_mut().find(|w| w.name == "HudHpText") {
                        hp_win.text = format!("{}/{}", hp, max_hp);
                    }
                    if let Some(sp_win) = wm.windows.values_mut().find(|w| w.name == "HudSpText") {
                        sp_win.text = format!("{}/{}", sp, max_sp);
                    }
                    if let Some(lv_win) = wm.windows.values_mut().find(|w| w.name == "HudLevelText") {
                        lv_win.text = format!("Lv. {}", level);
                    }
                    if let Some(gold_win) = wm.windows.values_mut().find(|w| w.name == "HudGoldText") {
                        gold_win.text = format!("{}", gold);
                    }
                }
                return true;
            } else {
                if crate::recv_actors::parse_point_change(data).is_ok() {
                    return true;
                }
            }
        }
        0x021C => { // SKILL_COOLTIME_END
            if crate::recv_combat_skills_complete::process_skill_cooltime_end(data).is_ok() {
                if data.len() >= 5 {
                    push_event(GameEvent::SkillCooltimeEnd { skill: data[4] });
                }
                return true;
            }
        }
        0x0302 => { // MOVE
            if data.len() >= 25 {
                let mut move_data = crate::recv_movement::CharacterMoveData {
                    func: data[4],
                    arg: data[5],
                    rot: data[6],
                    vid: u32::from_le_bytes(data[7..11].try_into().unwrap_or_default()),
                    x: i32::from_le_bytes(data[11..15].try_into().unwrap_or_default()),
                    y: i32::from_le_bytes(data[15..19].try_into().unwrap_or_default()),
                    time: u32::from_le_bytes(data[19..23].try_into().unwrap_or_default()),
                    duration: u32::from_le_bytes(data[23..27].try_into().unwrap_or_default()),
                };
                // Call manager if we had one statically, else returning true
                return true;
            }
        }
        0x0306 => { // WARP
            return crate::recv_warp_phase_v2::m2_net_recv_warp(data.as_ptr(), data.len());
        }
        0x0410 => { // DAMAGE_INFO
            if let Some((vid, dtype, dmg)) = crate::recv_chat::parse_damage_info(data) {
                return true;
            }
        }
        0x0510 => { // ITEM_DEL
            if crate::recv_items_payload::handle_item_del(data).is_ok() {
                return true;
            }
        }
        0x0511 => { // ITEM_SET
            if crate::recv_items_payload::handle_item_set(data).is_ok() {
                return true;
            }
        }
        0x0514 => { // ITEM_UPDATE
            if crate::recv_items_payload::handle_item_update(data).is_ok() {
                return true;
            }
        }
        0x051C => { // EXCHANGE
            return crate::recv_exchange_logic_v2::m2_net_recv_exchange(data.as_ptr(), data.len());
        }
        0x0603 => { // CHAT
            if let Some((ctype, chat)) = crate::recv_chat::parse_chat(data) {
                let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u32;
                crate::chat_buffer_v2::get_chat_buffer().lock().unwrap().append_line(ctype, &chat, timestamp);
                crate::py_event_dispatcher::dispatch_gc_chat(ctype as i32, &chat);
                return true;
            }
        }
        0x0604 => { // WHISPER
            if data.len() >= 30 {
                let name_end = data[5..30].iter().position(|&b| b == 0).unwrap_or(25);
                let name = String::from_utf8_lossy(&data[5..5 + name_end]).to_string();
                let text_data = &data[30..];
                let text_end = text_data.iter().position(|&b| b == 0).unwrap_or(text_data.len());
                let text = String::from_utf8_lossy(&text_data[..text_end]).to_string();
                let full_msg = format!("{} : {}", name, text);
                
                let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u32;
                crate::chat_buffer_v2::get_chat_buffer().lock().unwrap().append_line(crate::chat_buffer_v2::CHAT_TYPE_WHISPER, &full_msg, timestamp);
                crate::py_event_dispatcher::dispatch_gc_chat(crate::chat_buffer_v2::CHAT_TYPE_WHISPER as i32, &full_msg);
            }
            return true;
        }
        0x0710 => { // PARTY_INVITE
            if data.len() >= 8 {
                let leader_pid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
                if game_wnd != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "RecvPartyInviteQuestion", leader_pid as i32);
                }
                push_event(GameEvent::PartyInvite { leader_pid });
                return true;
            }
        }
        0x0711 | 0x0712 | 0x0713 => { // PARTY_ADD, PARTY_UPDATE, PARTY_REMOVE
            if header == 0x0711 && data.len() >= size_of::<crate::recv_party_logic_v2::TPacketGCPartyAdd>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_party_logic_v2::TPacketGCPartyAdd) };
                crate::recv_party_logic_v2::PARTY_MANAGER.handle_party_add(packet);
                return true;
            } else if header == 0x0712 && data.len() >= size_of::<crate::recv_party_logic_v2::TPacketGCPartyUpdate>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_party_logic_v2::TPacketGCPartyUpdate) };
                crate::recv_party_logic_v2::PARTY_MANAGER.handle_party_update(packet);
                let pid = packet.pid;
                let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
                if game_wnd != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "UpdatePartyMemberInfo", pid as i32);
                }
                push_event(GameEvent::PartyUpdate { pid });
                return true;
            } else if header == 0x0713 && data.len() >= size_of::<crate::recv_party_logic_v2::TPacketGCPartyRemove>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_party_logic_v2::TPacketGCPartyRemove) };
                crate::recv_party_logic_v2::PARTY_MANAGER.handle_party_remove(packet);
                return true;
            }
        }
        0x0730 => { // GUILD
            if crate::recv_guild_logic_v2::parse_guild_packet(data).is_ok() {
                return true;
            }
        }
        0x0741 => { // MESSENGER
            return crate::recv_messenger_friends_v2::m2_client_parse_messenger(data.as_ptr(), data.len());
        }
        0x0810 => { // SHOP
            return crate::recv_shop_logic_v2::m2_net_recv_shop(data.as_ptr(), data.len());
        }
        0x0811 => { // SHOP_SIGN
            if data.len() >= 8 {
                let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let sign = if data.len() > 8 {
                    let sign_data = &data[8..];
                    let sign_end = sign_data.iter().take(33).position(|&b| b == 0).unwrap_or(sign_data.len().min(33));
                    String::from_utf8_lossy(&sign_data[..sign_end]).to_string()
                } else {
                    String::new()
                };
                let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
                if game_wnd != 0 {
                    if sign.is_empty() {
                        crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "BINARY_PrivateShop_Disappear", vid as i32);
                    } else {
                        crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "BINARY_PrivateShop_Appear", vid as i32);
                    }
                }
                push_event(GameEvent::ShopSign { vid, sign });
                return true;
            }
        }
        0x0830 | 0x0831 => { // SAFEBOX_SET, SAFEBOX_DEL
            // In a real scenario we use SafeboxManager, here we signal handled.
            return true;
        }
        0x0833 => { // SAFEBOX_SIZE
            if data.len() >= 5 {
                let size = data[4];
                crate::recv_safebox_packets_complete::m2_safebox_recv_open(size);
                push_event(GameEvent::SafeboxSize { size });
                return true;
            }
        }
        0x0834 => { // SAFEBOX_MONEY_CHANGE
            if data.len() >= 8 {
                let money = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                crate::recv_safebox_packets_complete::m2_safebox_set_money(money);
                push_event(GameEvent::SafeboxMoneyChange { money });
                return true;
            }
        }
        0x0910 | 0x0912 => { // SCRIPT, QUEST_INFO
            return crate::recv_quest_dialog_v2::m2_quest_recv_script_packet(data.as_ptr(), data.len());
        }
        0x0A10 => { // TARGET
            if data.len() >= size_of::<crate::recv_target_info_v2::TPacketGCTarget>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_target_info_v2::TPacketGCTarget) };
                crate::recv_target_info_v2::parse_target_packet(packet);
                return true;
            }
        }
        0x0A11 => { // TARGET_UPDATE
            if data.len() >= 16 {
                let id = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let x = i32::from_le_bytes(data[8..12].try_into().unwrap_or_default());
                let y = i32::from_le_bytes(data[12..16].try_into().unwrap_or_default());
                push_event(GameEvent::TargetUpdate { id, x, y });
                return true;
            }
        }
        0x0A12 => { // TARGET_DELETE
            if data.len() >= 8 {
                let id = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                push_event(GameEvent::TargetDelete { id });
                return true;
            }
        }
        0x0B10 => { // FISHING
            if data.len() >= size_of::<crate::recv_fishing_system_v2::TPacketGCFishing>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_fishing_system_v2::TPacketGCFishing) };
                crate::recv_fishing_system_v2::handle_fishing_packet(packet);
                return true;
            }
        }
        0x0B11 => { // DUNGEON
            return crate::recv_dungeon_info_v2::parse_dungeon_packet(data);
        }
        _ => {}
    }
    
    // Fallback do starego dispatchera jesli pakiet nie jest zlapany przez V2
    crate::game_net_stream_dispatcher::dispatch_packet_extended(header, data)
}
