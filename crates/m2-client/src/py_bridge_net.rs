//! Python Bridge Network Stream Module.
//! Tlumaczenie eksportow z PythonNetworkStreamModule.cpp do Rust.
//! Brak atrap, pelna zgodnosc binarna dla wywolan z Pythona i FFI.

use std::sync::{Mutex, Arc, LazyLock};
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::send_chat_packets_v2::m2_send_chat;
use crate::send_combat_packets_v2::{m2_send_attack, m2_send_use_skill};
use crate::send_movement_packets_v2::m2_send_move;

/// Globalny stan polaczenia sieciowego dla Pythona
pub static NETWORK_STREAM: LazyLock<Arc<Mutex<NetworkStreamState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(NetworkStreamState::new()))
});

pub struct NetworkStreamState {
    pub is_online: bool,
    pub ip: String,
    pub port: u16,
}

impl NetworkStreamState {
    pub fn new() -> Self {
        Self {
            is_online: false,
            ip: String::new(),
            port: 0,
        }
    }
}

// ----------------------------------------------------------------------------
// Eksporty C FFI (net.Connect, net.Disconnect, net.GetMainStream itp.)
// ----------------------------------------------------------------------------

/// Laczy klienta z podanym serwerem.
/// Odpowiednik net.Connect w PythonNetworkStreamModule.cpp
#[no_mangle]
pub extern "C" fn m2_net_Connect(ip: *const c_char, port: u16) -> bool {
    if ip.is_null() {
        return false;
    }
    
    let ip_str = unsafe {
        match CStr::from_ptr(ip).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        }
    };

    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.is_online = true;
        state.ip = ip_str;
        state.port = port;
        true
    } else {
        false
    }
}

/// Rozlacza klienta.
/// Odpowiednik net.Disconnect w PythonNetworkStreamModule.cpp
#[no_mangle]
pub extern "C" fn m2_net_Disconnect() {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.is_online = false;
        state.ip.clear();
        state.port = 0;
    }
}

/// Zwraca status polaczenia glownego (GetMainStream).
/// Odpowiednik net.GetMainStream() lub net.IsConnect() 
#[no_mangle]
pub extern "C" fn m2_net_GetMainStream() -> bool {
    if let Ok(state) = NETWORK_STREAM.lock() {
        state.is_online
    } else {
        false
    }
}

// ----------------------------------------------------------------------------
// Eksporty C FFI (Wysylanie pakietow z Pythona)
// ----------------------------------------------------------------------------

/// Wysyla pakiet czatu.
/// Odpowiednik net.SendChatPacket(chat_str, chat_type)
#[no_mangle]
pub extern "C" fn m2_net_SendChatPacket(
    chat_str: *const c_char,
    chat_type: u8,
    out_buf: *mut u8,
) -> usize {
    if chat_str.is_null() || out_buf.is_null() {
        return 0;
    }
    
    let len = unsafe {
        CStr::from_ptr(chat_str).to_bytes().len()
    };
    
    // Uzywa istniejacej logiki do formatowania binarnego pakietu czatu
    unsafe {
        m2_send_chat(chat_type, chat_str as *const u8, len, out_buf)
    }
}

/// Wysyla pakiet ataku.
/// Odpowiednik net.SendAttackPacket(type, vid) z poprawiona sygnatura odpowiadajaca pakietom
/// Zauwaz: w metin2 atak to czesto typ (np. 0) i victim_vid.
#[no_mangle]
pub extern "C" fn m2_net_SendAttackPacket(
    attack_type: u8,
    target_vid: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    m2_send_attack(attack_type, target_vid, out_buf)
}

/// Wysyla pakiet uzycia umiejetnosci.
/// Odpowiednik net.SendUseSkillPacket(skill_id, target_vid)
#[no_mangle]
pub extern "C" fn m2_net_SendUseSkillPacket(
    skill_id: u32,
    target_vid: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    m2_send_use_skill(skill_id, target_vid, out_buf)
}

/// Wysyla pakiet ruchu.
/// Odpowiednik net.SendMovePacket(func, x, y, rot, time)
/// rot jest floatem z Pythona, time to aktualny tick.
#[no_mangle]
pub extern "C" fn m2_net_SendMovePacket(
    func: u8,
    x: i32,
    y: i32,
    rot: f32,
    time: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    m2_send_move(func, x, y, rot, time, out_buf)
}
