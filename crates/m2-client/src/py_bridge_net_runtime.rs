use std::ffi::CStr;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::os::raw::c_char;

use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use crate::secure_cipher::SecureCipher;

use crate::send_chat_packets_v2::{m2_send_chat, m2_send_whisper, TPacketCGChat, TPacketCGWhisper, CG_CHAT, CG_WHISPER};
use crate::send_combat_packets_v2::{m2_send_attack, m2_send_use_skill, TPacketCGAttack, TPacketCGUseSkill};
use crate::send_item_packets_v2::{
    m2_send_item_move, TPacketCGItemDrop, TPacketCGItemDrop2, TPacketCGItemMove, TPacketCGItemUse,
    HEADER_CG_ITEM_DROP, HEADER_CG_ITEM_DROP2, HEADER_CG_ITEM_MOVE, HEADER_CG_ITEM_USE, TItemPos,
};

#[no_mangle]
pub extern "C" fn m2_net_runtime_Connect(ip: *const c_char, port: u16) -> bool {
    if ip.is_null() {
        return false;
    }
    let ip_str = unsafe {
        match CStr::from_ptr(ip).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    let addr = format!("{}:{}", ip_str, port);

    match TcpStream::connect(&addr) {
        Ok(stream) => {
            let mut cipher = SecureCipher::new();
            // Metin2 auth happens separately, but here we just initialize the basic static keys
            // to allow packet encryption on the socket.
            cipher.initialize();
            
            if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
                *guard = Some((stream, cipher));
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_Disconnect() {
    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        *guard = None;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SetPhaseWindow(phase: u32, _window_handle: u32) {
    let mut phase_str = "LOGIN".to_string();
    if phase == 1 {
        phase_str = "SELECT".to_string();
    } else if phase == 2 {
        phase_str = "GAME".to_string();
    }
    
    if let Ok(mut lock) = crate::ui_bridge_ffi::CURRENT_PHASE.lock() {
        *lock = phase_str;
    }
}

fn send_packet(data: &[u8]) -> bool {
    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        if let Some(socket_data) = guard.as_mut() {
            let stream: &mut TcpStream = &mut socket_data.0;
            let cipher: &mut SecureCipher = &mut socket_data.1;
            let mut encrypted = data.to_vec();
            cipher.encrypt_in_place(&mut encrypted);
            return stream.write_all(&encrypted).is_ok();
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendChatPacket(msg: *const c_char, chat_type: u8) -> bool {
    if msg.is_null() {
        return false;
    }
    let msg_str = unsafe { CStr::from_ptr(msg).to_bytes() };
    
    if msg_str.is_empty() || msg_str.len() >= 512 {
        return false;
    }

    let text_len = msg_str.len() + 1;
    let packet_size = std::mem::size_of::<TPacketCGChat>();
    let mut buf = vec![0u8; packet_size + text_len];
    
    unsafe {
        m2_send_chat(chat_type, msg_str.as_ptr(), msg_str.len(), buf.as_mut_ptr());
    }
    
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendWhisperPacket(name: *const c_char, msg: *const c_char) -> bool {
    if name.is_null() || msg.is_null() {
        return false;
    }
    let name_str = unsafe { CStr::from_ptr(name).to_bytes() };
    let msg_str = unsafe { CStr::from_ptr(msg).to_bytes() };

    if msg_str.len() >= 255 {
        return false;
    }

    let text_len = msg_str.len() + 1;
    let packet_size = std::mem::size_of::<TPacketCGWhisper>();
    let mut buf = vec![0u8; packet_size + text_len];

    unsafe {
        m2_send_whisper(name_str.as_ptr(), name_str.len(), msg_str.as_ptr(), msg_str.len(), buf.as_mut_ptr());
    }
    
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendItemUsePacket(window_type: u8, cell: u16) -> bool {
    let packet = TPacketCGItemUse::new(window_type, cell);
    let packet_size = std::mem::size_of::<TPacketCGItemUse>();
    
    let mut buf = vec![0u8; packet_size];
    unsafe {
        std::ptr::copy_nonoverlapping(&packet as *const _ as *const u8, buf.as_mut_ptr(), packet_size);
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendItemMovePacket(src_w: u8, src_c: u16, dst_w: u8, dst_c: u16, count: u8) -> bool {
    let packet_size = std::mem::size_of::<TPacketCGItemMove>();
    
    let mut buf = vec![0u8; packet_size];
    unsafe {
        m2_send_item_move(src_w, src_c, dst_w, dst_c, count, buf.as_mut_ptr());
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendItemDropPacket(window_type: u8, cell: u16, count: u8) -> bool {
    let packet = TPacketCGItemDrop2::new(window_type, cell, 0, count);
    let packet_size = std::mem::size_of::<TPacketCGItemDrop2>();
    
    let mut buf = vec![0u8; packet_size];
    unsafe {
        std::ptr::copy_nonoverlapping(&packet as *const _ as *const u8, buf.as_mut_ptr(), packet_size);
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendItemDestroyPacket(window_type: u8, cell: u16) -> bool {
    let packet = TPacketCGItemDrop::new(window_type, cell, 0);
    let packet_size = std::mem::size_of::<TPacketCGItemDrop>();
    
    let mut buf = vec![0u8; packet_size];
    unsafe {
        std::ptr::copy_nonoverlapping(&packet as *const _ as *const u8, buf.as_mut_ptr(), packet_size);
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendAttackPacket(attack_type: u8, target_vid: u32) -> bool {
    let packet_size = std::mem::size_of::<TPacketCGAttack>();
    let mut buf = vec![0u8; packet_size];
    unsafe {
        m2_send_attack(attack_type, target_vid, buf.as_mut_ptr());
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendUseSkillPacket(skill_idx: u32, target_vid: u32) -> bool {
    let packet_size = std::mem::size_of::<TPacketCGUseSkill>();
    let mut buf = vec![0u8; packet_size];
    unsafe {
        m2_send_use_skill(skill_idx, target_vid, buf.as_mut_ptr());
    }
    send_packet(&buf)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendCharacterSelectPacket(char_index: u8) -> bool {
    let mut sel_pkt = [0u8; 2];
    sel_pkt[0] = 0x03; // HEADER_CG_CHARACTER_SELECT
    sel_pkt[1] = char_index; // slot_idx
    send_packet(&sel_pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_runtime_SendEnterGame() -> bool {
    let mut enter_pkt = [0u8; 1];
    enter_pkt[0] = 0x04; // HEADER_CG_ENTERGAME
    send_packet(&enter_pkt)
}
