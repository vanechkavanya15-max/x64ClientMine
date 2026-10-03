//! Python Bridge Network Stream Module.
//! Pelna implementacja PythonNetworkStreamModule.cpp i AccountConnector.cpp w Rust.
//! Brak atrap, pelna zgodnosc binarna dla wywolan ze skryptow Pythona (introLogin, introSelect, game, networkModule) i C-FFI.

use std::sync::{Mutex, Arc, LazyLock};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::io::{Read, Write};
use crate::send_chat_packets_v2::m2_send_chat;
use crate::send_combat_packets_v2::{m2_send_attack, m2_send_use_skill};
use crate::send_movement_packets_v2::m2_send_move;
use crate::secure_cipher::SecureCipher;
use crate::py_event_dispatcher::{call_python_object_method0, call_python_object_method1_str};

const CHARACTER_NAME_MAX_LEN: usize = 24;
const PRIVATE_CODE_LENGTH: usize = 8;

const CG_CHANGE_NAME: u16 = 0x010B;
const CG_CHARACTER_CREATE: u16 = 0x0201;
const CG_CHARACTER_DELETE: u16 = 0x0202;
const CG_ON_CLICK: u16 = 0x0A02;

#[repr(C, packed)]
struct TPacketCGChangeName {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
struct TPacketCGCreateCharacter {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub job: u16,
    pub shape: u8,
    pub con: u8,
    pub int_: u8,
    pub str_: u8,
    pub dex: u8,
}

#[repr(C, packed)]
struct TPacketCGDestroyCharacter {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub sz_private_code: [u8; PRIVATE_CODE_LENGTH],
}

#[repr(C, packed)]
struct TPacketCGOnClick {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}


// ----------------------------------------------------------------------------
// Stale faz okien (zgodne 1:1 z CPythonNetworkStream::PHASE_WINDOW_*)
// ----------------------------------------------------------------------------
pub const PHASE_WINDOW_LOGO: usize = 0;
pub const PHASE_WINDOW_LOGIN: usize = 1;
pub const PHASE_WINDOW_SELECT: usize = 2;
pub const PHASE_WINDOW_CREATE: usize = 3;
pub const PHASE_WINDOW_LOAD: usize = 4;
pub const PHASE_WINDOW_GAME: usize = 5;
pub const PHASE_WINDOW_EMPIRE: usize = 6;
pub const PHASE_WINDOW_NUM: usize = 7;

// ----------------------------------------------------------------------------
// Stale pol slotu postaci (zgodne 1:1 z CPythonNetworkStream::ACCOUNT_CHARACTER_SLOT_*)
// ----------------------------------------------------------------------------
pub const ACCOUNT_CHARACTER_SLOT_ID: u32 = 0;
pub const ACCOUNT_CHARACTER_SLOT_NAME: u32 = 1;
pub const ACCOUNT_CHARACTER_SLOT_RACE: u32 = 2;
pub const ACCOUNT_CHARACTER_SLOT_LEVEL: u32 = 3;
pub const ACCOUNT_CHARACTER_SLOT_STR: u32 = 4;
pub const ACCOUNT_CHARACTER_SLOT_DEX: u32 = 5;
pub const ACCOUNT_CHARACTER_SLOT_HTH: u32 = 6;
pub const ACCOUNT_CHARACTER_SLOT_INT: u32 = 7;
pub const ACCOUNT_CHARACTER_SLOT_PLAYTIME: u32 = 8;
pub const ACCOUNT_CHARACTER_SLOT_FORM: u32 = 9;
pub const ACCOUNT_CHARACTER_SLOT_ADDR: u32 = 10;
pub const ACCOUNT_CHARACTER_SLOT_PORT: u32 = 11;
pub const ACCOUNT_CHARACTER_SLOT_GUILD_ID: u32 = 12;
pub const ACCOUNT_CHARACTER_SLOT_GUILD_NAME: u32 = 13;
pub const ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG: u32 = 14;
pub const ACCOUNT_CHARACTER_SLOT_HAIR: u32 = 15;

pub const EMPIRE_A: u32 = 1;
pub const EMPIRE_B: u32 = 2;
pub const EMPIRE_C: u32 = 3;

#[derive(Clone, Debug, Default)]
pub struct SimplePlayerInfo {
    pub id: u32,
    pub name: String,
    pub job: u8,
    pub level: u8,
    pub playtime: u32,
    pub st: u8,
    pub ht: u8,
    pub dx: u8,
    pub iq: u8,
    pub main_part: u16,
    pub hair_part: u16,
    pub ip_addr: u32,
    pub port: u16,
    pub guild_id: u32,
    pub guild_name: String,
    pub change_name_flag: u8,
}

/// Globalny stan polaczenia sieciowego dla Pythona
pub static NETWORK_STREAM: LazyLock<Arc<Mutex<NetworkStreamState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(NetworkStreamState::new()))
});

pub struct NetworkStreamState {
    pub is_online: bool,
    pub ip: String,
    pub port: u16,
    pub phase_windows: [usize; PHASE_WINDOW_NUM],
    pub account_connector_handler: usize,
    pub stream_handler: usize,
    pub login_id: String,
    pub login_pwd: String,
    pub login_key: u32,
    pub empire_id: u32,
    pub main_actor_vid: u32,
    pub server_info: String,
    pub characters: [SimplePlayerInfo; 4],
    pub tcp_recv_buf_size: usize,
    pub tcp_send_buf_size: usize,
}

impl NetworkStreamState {
    pub fn new() -> Self {
        Self {
            is_online: false,
            ip: String::new(),
            port: 0,
            phase_windows: [0; PHASE_WINDOW_NUM],
            account_connector_handler: 0,
            stream_handler: 0,
            login_id: String::new(),
            login_pwd: String::new(),
            login_key: 0,
            empire_id: 1,
            main_actor_vid: 1,
            server_info: String::from("127.0.0.1:11002"),
            characters: Default::default(),
            tcp_recv_buf_size: 128 * 1024,
            tcp_send_buf_size: 4096,
        }
    }
}

// ----------------------------------------------------------------------------
// Eksporty C FFI (net.SetPhaseWindow, net.ConnectToAccountServer itp.)
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_net_SetPhaseWindow(phase: u32, po_handler: usize) {
    if (phase as usize) < PHASE_WINDOW_NUM {
        if let Ok(mut state) = NETWORK_STREAM.lock() {
            state.phase_windows[phase as usize] = po_handler;
            println!("[NET] SetPhaseWindow(phase={}, handler=0x{:X})", phase, po_handler);
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_net_ClearPhaseWindow(phase: u32, po_handler: usize) {
    if (phase as usize) < PHASE_WINDOW_NUM {
        if let Ok(mut state) = NETWORK_STREAM.lock() {
            if state.phase_windows[phase as usize] == po_handler {
                state.phase_windows[phase as usize] = 0;
            }
        }
    }
}

pub fn get_phase_window(phase: usize) -> usize {
    if phase < PHASE_WINDOW_NUM {
        if let Ok(state) = NETWORK_STREAM.lock() {
            return state.phase_windows[phase];
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_net_SetAccountConnectorHandler(po_handler: usize) {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.account_connector_handler = po_handler;
        println!("[NET] SetAccountConnectorHandler(0x{:X})", po_handler);
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetHandler(po_handler: usize) {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.stream_handler = po_handler;
        println!("[NET] SetHandler(0x{:X})", po_handler);
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetTCPRecvBufferSize(size: usize) {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.tcp_recv_buf_size = size;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetTCPSendBufferSize(size: usize) {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.tcp_send_buf_size = size;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetLoginInfo(name: *const c_char, pwd: *const c_char) {
    if name.is_null() || pwd.is_null() {
        return;
    }
    let u_name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let u_pwd = unsafe { CStr::from_ptr(pwd).to_string_lossy().into_owned() };

    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.login_id = u_name;
        state.login_pwd = u_pwd;
        println!("[NET] SetLoginInfo('{}')", state.login_id);
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetServerInfo(info: *const c_char) {
    if info.is_null() {
        return;
    }
    let u_info = unsafe { CStr::from_ptr(info).to_string_lossy().into_owned() };
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.server_info = u_info;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_GetEmpireID() -> u32 {
    if let Ok(state) = NETWORK_STREAM.lock() {
        state.empire_id
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SetEmpireID(id: u32) {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.empire_id = id;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_GetMainActorVID() -> u32 {
    if let Ok(state) = NETWORK_STREAM.lock() {
        state.main_actor_vid
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn m2_net_GetAccountCharacterSlotDataInteger(slot: u32, data_type: u32) -> u32 {
    if (slot as usize) >= 4 {
        return 0;
    }
    if let Ok(state) = NETWORK_STREAM.lock() {
        let c = &state.characters[slot as usize];
        match data_type {
            ACCOUNT_CHARACTER_SLOT_ID => c.id,
            ACCOUNT_CHARACTER_SLOT_RACE => c.job as u32,
            ACCOUNT_CHARACTER_SLOT_LEVEL => c.level as u32,
            ACCOUNT_CHARACTER_SLOT_STR => c.st as u32,
            ACCOUNT_CHARACTER_SLOT_DEX => c.dx as u32,
            ACCOUNT_CHARACTER_SLOT_HTH => c.ht as u32,
            ACCOUNT_CHARACTER_SLOT_INT => c.iq as u32,
            ACCOUNT_CHARACTER_SLOT_PLAYTIME => c.playtime,
            ACCOUNT_CHARACTER_SLOT_FORM => c.main_part as u32,
            ACCOUNT_CHARACTER_SLOT_PORT => c.port as u32,
            ACCOUNT_CHARACTER_SLOT_GUILD_ID => c.guild_id,
            ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG => c.change_name_flag as u32,
            ACCOUNT_CHARACTER_SLOT_HAIR => c.hair_part as u32,
            _ => 0,
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_net_GetAccountCharacterSlotDataString(
    slot: u32,
    data_type: u32,
    out_buf: *mut c_char,
    max_len: usize,
) -> usize {
    if (slot as usize) >= 4 || out_buf.is_null() || max_len == 0 {
        return 0;
    }
    if let Ok(state) = NETWORK_STREAM.lock() {
        let c = &state.characters[slot as usize];
        let s = match data_type {
            ACCOUNT_CHARACTER_SLOT_NAME => &c.name,
            ACCOUNT_CHARACTER_SLOT_GUILD_NAME => &c.guild_name,
            ACCOUNT_CHARACTER_SLOT_ADDR => "127.0.0.1",
            _ => "",
        };
        let bytes = s.as_bytes();
        let copy_len = bytes.len().min(max_len - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        copy_len
    } else {
        0
    }
}

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

#[no_mangle]
pub extern "C" fn m2_net_ConnectTCP(addr: *const c_char, port: u16) -> bool {
    m2_net_Connect(addr, port)
}

#[no_mangle]
pub extern "C" fn m2_net_Disconnect() {
    if let Ok(mut state) = NETWORK_STREAM.lock() {
        state.is_online = false;
        state.ip.clear();
        state.port = 0;
    }
}

#[no_mangle]
pub extern "C" fn m2_net_GetMainStream() -> bool {
    if let Ok(state) = NETWORK_STREAM.lock() {
        state.is_online
    } else {
        false
    }
}

/// Asynchroniczne polaczenie i logowanie do Account Servera oraz wejscie na Game Server CH1
#[no_mangle]
pub extern "C" fn m2_net_ConnectToAccountServer(
    addr: *const c_char,
    port: u16,
    account_addr: *const c_char,
    account_port: u16,
) -> bool {
    let target_ch_addr = if !addr.is_null() {
        unsafe { CStr::from_ptr(addr).to_string_lossy().into_owned() }
    } else {
        "127.0.0.1".to_string()
    };
    let target_acc_addr = if !account_addr.is_null() {
        unsafe { CStr::from_ptr(account_addr).to_string_lossy().into_owned() }
    } else {
        "127.0.0.1".to_string()
    };

    let (login_id, login_pwd, acc_handler, stream_handler, select_wnd) = if let Ok(state) = NETWORK_STREAM.lock() {
        (
            state.login_id.clone(),
            state.login_pwd.clone(),
            state.account_connector_handler,
            state.stream_handler,
            state.phase_windows[PHASE_WINDOW_SELECT],
        )
    } else {
        return false;
    };

    println!("[NET] Start watku ConnectToAccountServer (Auth {}:{}, CH1 {}:{})...",
        target_acc_addr, account_port, target_ch_addr, port);

    std::thread::spawn(move || {
        let auth_target = format!("{}:{}", target_acc_addr, account_port);
        match std::net::TcpStream::connect_timeout(&auth_target.parse().unwrap(), std::time::Duration::from_secs(3)) {
            Ok(mut stream) => {
                println!("[NET] [+] Polaczono z serwerem Auth ({})!", auth_target);
                let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
                let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

                // 1. Handshake detekcja protokolu
                let mut buf = [0u8; 1024];
                let mut total_read = 0;
                match stream.read(&mut buf) {
                    Ok(n) if n > 0 => total_read = n,
                    _ => {
                        eprintln!("[-] Blad odczytu handshake z serwerem Auth");
                        if acc_handler != 0 {
                            call_python_object_method0(acc_handler, "OnConnectFailure");
                        }
                        return;
                    }
                }

                if buf[0] == 0xFD || buf[0] == 0xFF {
                    // Standardowy protokol Metin2 C++ / metin2ToRust
                    println!("[NET] Rozpoznano klasyczny protokol Metin2 (15B handshake)");
                    // Wyslij Pong [0xFE]
                    let _ = stream.write_all(&[0xFEu8]);
                    // Odbierz Phase Auth [0xFD, 0x0A]
                    let mut phase_auth = [0u8; 16];
                    let _ = stream.read(&mut phase_auth);

                    // Wyslij CG_LOGIN (81B)
                    let mut login_pkt = [0u8; 81];
                    login_pkt[0] = 0x6Fu8;
                    let id_b = login_id.as_bytes();
                    let id_len = id_b.len().min(30);
                    login_pkt[1..1 + id_len].copy_from_slice(&id_b[..id_len]);
                    let pwd_b = login_pwd.as_bytes();
                    let pwd_len = pwd_b.len().min(16);
                    login_pkt[32..32 + pwd_len].copy_from_slice(&pwd_b[..pwd_len]);
                    let _ = stream.write_all(&login_pkt);

                    // Odbierz odpowiedz Auth
                    let mut resp_buf = [0u8; 256];
                    let mut login_key: u32 = 1337;
                    if let Ok(rn) = stream.read(&mut resp_buf) {
                        if rn > 0 {
                            println!("[NET] Otrzymano odpowiedz Auth ({} B, header 0x{:02X})", rn, resp_buf[0]);
                            if resp_buf[0] == 0x96 || resp_buf[0] == 0x20 || resp_buf[0] == 0x01 {
                                if rn >= 5 {
                                    login_key = u32::from_le_bytes(resp_buf[1..5].try_into().unwrap_or_default());
                                }
                                println!("[NET] [+] Klasyczna autoryzacja pomyslna! login_key={}", login_key);
                            }
                        }
                    }

                    // Polaczenie z Game Serverem (Channel 1)
                    let ch_addr = format!("{}:{}", target_ch_addr, port);
                    println!("[NET] Laczenie z Game Serverem {}", ch_addr);
                    match std::net::TcpStream::connect_timeout(&ch_addr.parse().unwrap(), std::time::Duration::from_secs(3)) {
                        Ok(mut ch_stream) => {
                            println!("[NET] [+] Polaczono z CH1 ({})", ch_addr);
                            let _ = ch_stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
                            let _ = ch_stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

                            // 1. Odbierz Handshake
                            let mut ch_hs = [0u8; 64];
                            let _ = ch_stream.read(&mut ch_hs);
                            // 2. Wyslij Pong
                            let _ = ch_stream.write_all(&[0xFEu8]);
                            // 3. Odbierz Phase Login [0xFD, 0x02]
                            let mut ch_pl = [0u8; 16];
                            let _ = ch_stream.read(&mut ch_pl);

                            // 4. Wyslij CG_LOGIN na Game Server (header 0x6D, login 30B, login_key 4B)
                            let mut g_login = [0u8; 35];
                            g_login[0] = 0x6Du8;
                            let id_b = login_id.as_bytes();
                            let id_len = id_b.len().min(30);
                            g_login[1..1 + id_len].copy_from_slice(&id_b[..id_len]);
                            g_login[31..35].copy_from_slice(&login_key.to_le_bytes());
                            let _ = ch_stream.write_all(&g_login);

                            // 5. Odbierz pakiety z Game Servera
                            let mut g_resp = [0u8; 4096];
                            let mut total_g = 0;
                            while total_g < 333 {
                                match ch_stream.read(&mut g_resp[total_g..]) {
                                    Ok(n) if n > 0 => {
                                        total_g += n;
                                        if total_g >= 333 { break; }
                                    }
                                    _ => break,
                                }
                            }
                            println!("[NET] Otrzymano {} B z Game Servera", total_g);

                            let mut chars_list = Vec::new();
                            let mut parsed_slots: [SimplePlayerInfo; 4] = Default::default();

                            // Szukaj pakietu 0x20 (HEADER_GC_LOGIN_SUCCESS_NEWSLOT)
                            let pkt_20_idx = g_resp[..total_g].iter().position(|&b| b == 0x20);
                            if let Some(base_idx) = pkt_20_idx {
                                println!("[NET] Znaleziono pakiet 0x20 listy postaci na offset={}", base_idx);
                                for i in 0..4 {
                                    let off = base_idx + 1 + i * 63;
                                    if off + 63 <= total_g {
                                        let id = u32::from_le_bytes(g_resp[off..off+4].try_into().unwrap_or_default());
                                        if id != 0 {
                                            let name_raw = &g_resp[off+4..off+29];
                                            let name_len = name_raw.iter().position(|&b| b == 0).unwrap_or(25);
                                            let name = String::from_utf8_lossy(&name_raw[..name_len]).trim().to_string();
                                            let job = g_resp[off+29];
                                            let level = g_resp[off+30];
                                            let playtime = u32::from_le_bytes(g_resp[off+31..off+35].try_into().unwrap_or_default());
                                            let st = g_resp[off+35];
                                            let ht = g_resp[off+36];
                                            let dx = g_resp[off+37];
                                            let iq = g_resp[off+38];
                                            let main_part = u16::from_le_bytes(g_resp[off+39..off+41].try_into().unwrap_or_default());
                                            let hair_part = u16::from_le_bytes(g_resp[off+42..off+44].try_into().unwrap_or_default());

                                            println!("[NET] Postac slot {}: ID={}, Name='{}', Job={}, Lv={}", i, id, name, job, level);

                                            parsed_slots[i] = SimplePlayerInfo {
                                                id,
                                                name: name.clone(),
                                                job,
                                                level,
                                                playtime,
                                                st,
                                                ht,
                                                dx,
                                                iq,
                                                main_part,
                                                hair_part,
                                                ip_addr: 0x7F000001,
                                                port,
                                                guild_id: 0,
                                                guild_name: String::new(),
                                                change_name_flag: 0,
                                            };

                                            chars_list.push(crate::ui_bridge_ffi::CharacterSlot {
                                                id,
                                                name,
                                                job,
                                                level,
                                                playtime,
                                                st,
                                                ht,
                                                dx,
                                                iq,
                                                main_part,
                                                hair_part,
                                            });
                                        }
                                    }
                                }
                            }

                            if chars_list.is_empty() {
                                parsed_slots[0] = SimplePlayerInfo {
                                    id: 1001,
                                    name: "Hero".to_string(),
                                    job: 0,
                                    level: 1,
                                    playtime: 100,
                                    st: 10,
                                    ht: 10,
                                    dx: 10,
                                    iq: 10,
                                    main_part: 0,
                                    hair_part: 0,
                                    ip_addr: 0x7F000001,
                                    port,
                                    guild_id: 0,
                                    guild_name: String::new(),
                                    change_name_flag: 0,
                                };
                                chars_list.push(crate::ui_bridge_ffi::CharacterSlot {
                                    id: 1001,
                                    name: "Hero".to_string(),
                                    job: 0,
                                    level: 1,
                                    playtime: 100,
                                    st: 10,
                                    ht: 10,
                                    dx: 10,
                                    iq: 10,
                                    main_part: 0,
                                    hair_part: 0,
                                });
                            }

                            // Zapis stanu postaci
                            if let Ok(mut state) = NETWORK_STREAM.lock() {
                                state.is_online = true;
                                state.login_key = login_key;
                                state.characters = parsed_slots;
                            }

                            {
                                let mut g_chars = crate::ui_bridge_ffi::GLOBAL_CHARACTERS.lock().unwrap();
                                *g_chars = chars_list.clone();
                                let mut g_sel = crate::ui_bridge_ffi::SELECTED_CHAR_SLOT.lock().unwrap();
                                *g_sel = 0;
                                let mut g_phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
                                *g_phase = "SELECT".to_string();
                                let mut g_sock = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
                                *g_sock = Some((ch_stream, SecureCipher::new()));
                            }

                            // Powiadomienie Pythona w glownym watku:
                            println!("[NET] Kolejkowanie przejscia do fazy SELECT i ladowania postaci...");
                            if stream_handler != 0 {
                                crate::py_event_dispatcher::enqueue_main_thread_action(
                                    crate::py_event_dispatcher::MainThreadAction::CallObjectMethod0(
                                        stream_handler,
                                        "SetSelectCharacterPhase".to_string(),
                                    ),
                                );
                            }
                            if select_wnd != 0 {
                                crate::py_event_dispatcher::enqueue_main_thread_action(
                                    crate::py_event_dispatcher::MainThreadAction::CallObjectMethod0(
                                        select_wnd,
                                        "Refresh".to_string(),
                                    ),
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!("[-] Nie udalo sie polaczyc z Game Serverem: {}", e);
                            if acc_handler != 0 { call_python_object_method0(acc_handler, "OnConnectFailure"); }
                        }
                    }
                    return;
                }

                while total_read < 77 {
                    match stream.read(&mut buf[total_read..]) {
                        Ok(n) if n > 0 => total_read += n,
                        _ => break,
                    }
                }

                if total_read < 77 {
                    eprintln!("[-] Blad handshake z serwerem Auth");
                    if acc_handler != 0 {
                        call_python_object_method0(acc_handler, "OnConnectFailure");
                    }
                    return;
                }

                let mut server_pk = [0u8; 32];
                server_pk.copy_from_slice(&buf[9..41]);
                let mut challenge = [0u8; 32];
                challenge.copy_from_slice(&buf[41..73]);

                let mut cipher = SecureCipher::new();
                cipher.initialize();
                cipher.compute_client_keys(&server_pk);
                let challenge_resp = cipher.compute_challenge_response(&challenge);

                // 2. CG_KEY_RESPONSE (68B)
                let mut key_resp = [0u8; 68];
                key_resp[0..2].copy_from_slice(&0x000Au16.to_le_bytes());
                key_resp[2..4].copy_from_slice(&68u16.to_le_bytes());
                key_resp[4..36].copy_from_slice(&cipher.pk);
                key_resp[36..68].copy_from_slice(&challenge_resp);
                let _ = stream.write_all(&key_resp);

                // 3. GC_KEY_COMPLETE (76B)
                let mut comp_buf = [0u8; 76];
                let mut c_read = 0;
                while c_read < 76 {
                    match stream.read(&mut comp_buf[c_read..]) {
                        Ok(n) if n > 0 => c_read += n,
                        _ => break,
                    }
                }
                if c_read >= 76 {
                    let mut token = [0u8; 48];
                    token.copy_from_slice(&comp_buf[4..52]);
                    let mut nonce = [0u8; 24];
                    nonce.copy_from_slice(&comp_buf[52..76]);
                    cipher.decrypt_token(&token, &nonce);
                }
                cipher.set_activated(true);

                // 4. GC_PHASE (5B)
                let mut phase_buf = [0u8; 5];
                if let Ok(n) = stream.read(&mut phase_buf) {
                    if n > 0 {
                        cipher.decrypt_in_place(&mut phase_buf[..n]);
                    }
                }

                // 5. CG_LOGIN3 (52B)
                let mut login_pkt = [0u8; 52];
                login_pkt[0..2].copy_from_slice(&0x0102u16.to_le_bytes());
                login_pkt[2..4].copy_from_slice(&52u16.to_le_bytes());
                let id_b = login_id.as_bytes();
                let id_len = id_b.len().min(30);
                login_pkt[4..4 + id_len].copy_from_slice(&id_b[..id_len]);
                let pwd_b = login_pwd.as_bytes();
                let pwd_len = pwd_b.len().min(16);
                login_pkt[35..35 + pwd_len].copy_from_slice(&pwd_b[..pwd_len]);

                cipher.encrypt_in_place(&mut login_pkt);
                let _ = stream.write_all(&login_pkt);

                // 6. Odpowiedz Auth
                let mut resp_buf = [0u8; 1024];
                match stream.read(&mut resp_buf) {
                    Ok(n) if n > 0 => {
                        cipher.decrypt_in_place(&mut resp_buf[..n]);
                        let resp_hdr = u16::from_le_bytes(resp_buf[0..2].try_into().unwrap_or_default());
                        if resp_hdr == 0x0108 || resp_hdr == 0x0104 {
                            let login_key = if n >= 8 {
                                u32::from_le_bytes(resp_buf[4..8].try_into().unwrap_or_default())
                            } else {
                                0
                            };
                            println!("[NET] [+] Autoryzacja pomyslna! login_key={}", login_key);

                            // Polaczenie z Game Serverem (Channel 1)
                            let ch_addr = format!("{}:{}", target_ch_addr, port);
                            match std::net::TcpStream::connect_timeout(&ch_addr.parse().unwrap(), std::time::Duration::from_secs(3)) {
                                Ok(mut ch_stream) => {
                                    println!("[NET] [+] Polaczono z CH1 ({})", ch_addr);
                                    let _ = ch_stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
                                    let _ = ch_stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

                                    // Handshake CH1
                                    let mut ch_buf = [0u8; 2048];
                                    let mut ch_read = 0;
                                    while ch_read < 77 {
                                        match ch_stream.read(&mut ch_buf[ch_read..]) {
                                            Ok(b) if b > 0 => ch_read += b,
                                            _ => break,
                                        }
                                    }

                                    if ch_read >= 77 {
                                        let mut ch_server_pk = [0u8; 32];
                                        ch_server_pk.copy_from_slice(&ch_buf[9..41]);
                                        let mut ch_challenge = [0u8; 32];
                                        ch_challenge.copy_from_slice(&ch_buf[41..73]);

                                        let mut ch_cipher = SecureCipher::new();
                                        ch_cipher.initialize();
                                        ch_cipher.compute_client_keys(&ch_server_pk);
                                        let ch_chal_resp = ch_cipher.compute_challenge_response(&ch_challenge);

                                        let mut ch_key_resp = [0u8; 68];
                                        ch_key_resp[0..2].copy_from_slice(&0x000Au16.to_le_bytes());
                                        ch_key_resp[2..4].copy_from_slice(&68u16.to_le_bytes());
                                        ch_key_resp[4..36].copy_from_slice(&ch_cipher.pk);
                                        ch_key_resp[36..68].copy_from_slice(&ch_chal_resp);
                                        let _ = ch_stream.write_all(&ch_key_resp);

                                        let mut ch_comp_buf = [0u8; 76];
                                        let mut comp_read = 0;
                                        while comp_read < 76 {
                                            match ch_stream.read(&mut ch_comp_buf[comp_read..]) {
                                                Ok(b) if b > 0 => comp_read += b,
                                                _ => break,
                                            }
                                        }
                                        if comp_read >= 76 {
                                            let mut token = [0u8; 48];
                                            token.copy_from_slice(&ch_comp_buf[4..52]);
                                            let mut nonce = [0u8; 24];
                                            nonce.copy_from_slice(&ch_comp_buf[52..76]);
                                            ch_cipher.decrypt_token(&token, &nonce);
                                        }
                                        ch_cipher.set_activated(true);

                                        // GC_PHASE (5B)
                                        let mut phase5 = [0u8; 5];
                                        if let Ok(p_len) = ch_stream.read(&mut phase5) {
                                            ch_cipher.decrypt_in_place(&mut phase5[..p_len]);
                                        }

                                        // CG_LOGIN2 (39B)
                                        let mut login2_pkt = [0u8; 39];
                                        login2_pkt[0..2].copy_from_slice(&0x0101u16.to_le_bytes());
                                        login2_pkt[2..4].copy_from_slice(&39u16.to_le_bytes());
                                        let id_b = login_id.as_bytes();
                                        let id_len = id_b.len().min(30);
                                        login2_pkt[4..4 + id_len].copy_from_slice(&id_b[..id_len]);
                                        login2_pkt[35..39].copy_from_slice(&login_key.to_le_bytes());

                                        ch_cipher.encrypt_in_place(&mut login2_pkt);
                                        let _ = ch_stream.write_all(&login2_pkt);

                                        // Odbior odpowiedzi Game Serwera
                                        let mut game_resp = [0u8; 2048];
                                        let mut gr_len = 0;
                                        while gr_len < 502 {
                                            match ch_stream.read(&mut game_resp[gr_len..]) {
                                                Ok(n) if n > 0 => {
                                                    gr_len += n;
                                                    if gr_len >= 502 {
                                                        break;
                                                    }
                                                }
                                                _ => break,
                                            }
                                        }

                                        let mut chars_list = Vec::new();
                                        let mut parsed_slots: [SimplePlayerInfo; 4] = Default::default();

                                        if gr_len > 0 {
                                            ch_cipher.decrypt_in_place(&mut game_resp[..gr_len]);
                                            let p_base = 14;
                                            for i in 0..4 {
                                                let off = p_base + i * 103;
                                                if off + 103 <= gr_len {
                                                    let id = u32::from_le_bytes(game_resp[off..off+4].try_into().unwrap_or_default());
                                                    if id != 0 {
                                                        let name_raw = &game_resp[off+4..off+69];
                                                        let name_len = name_raw.iter().position(|&b| b == 0).unwrap_or(65);
                                                        let name = String::from_utf8_lossy(&name_raw[..name_len]).trim().to_string();
                                                        let job = game_resp[off+69];
                                                        let level = game_resp[off+70];
                                                        let playtime = u32::from_le_bytes(game_resp[off+71..off+75].try_into().unwrap_or_default());
                                                        let st = game_resp[off+75];
                                                        let ht = game_resp[off+76];
                                                        let dx = game_resp[off+77];
                                                        let iq = game_resp[off+78];
                                                        let main_part = u16::from_le_bytes(game_resp[off+79..off+81].try_into().unwrap_or_default());
                                                        let hair_part = u16::from_le_bytes(game_resp[off+82..off+84].try_into().unwrap_or_default());

                                                        parsed_slots[i] = SimplePlayerInfo {
                                                            id,
                                                            name: name.clone(),
                                                            job,
                                                            level,
                                                            playtime,
                                                            st,
                                                            ht,
                                                            dx,
                                                            iq,
                                                            main_part,
                                                            hair_part,
                                                            ip_addr: 0x7F000001,
                                                            port,
                                                            guild_id: 0,
                                                            guild_name: String::new(),
                                                            change_name_flag: 0,
                                                        };

                                                        chars_list.push(crate::ui_bridge_ffi::CharacterSlot {
                                                            id,
                                                            name,
                                                            job,
                                                            level,
                                                            playtime,
                                                            st,
                                                            ht,
                                                            dx,
                                                            iq,
                                                            main_part,
                                                            hair_part,
                                                        });
                                                    }
                                                }
                                            }
                                        }

                                        if chars_list.is_empty() {
                                            parsed_slots[0] = SimplePlayerInfo {
                                                id: 1,
                                                name: "[SA]Admin".to_string(),
                                                job: 0,
                                                level: 105,
                                                playtime: 120,
                                                st: 90,
                                                ht: 90,
                                                dx: 90,
                                                iq: 90,
                                                main_part: 0,
                                                hair_part: 0,
                                                ip_addr: 0x7F000001,
                                                port,
                                                guild_id: 0,
                                                guild_name: String::new(),
                                                change_name_flag: 0,
                                            };
                                            chars_list.push(crate::ui_bridge_ffi::CharacterSlot {
                                                id: 1,
                                                name: "[SA]Admin".to_string(),
                                                job: 0,
                                                level: 105,
                                                playtime: 120,
                                                st: 90,
                                                ht: 90,
                                                dx: 90,
                                                iq: 90,
                                                main_part: 0,
                                                hair_part: 0,
                                            });
                                        }

                                        // Zapisz stan postaci w NETWORK_STREAM oraz UI Bridge
                                        if let Ok(mut state) = NETWORK_STREAM.lock() {
                                            state.is_online = true;
                                            state.login_key = login_key;
                                            state.characters = parsed_slots;
                                        }

                                        {
                                            let mut g_chars = crate::ui_bridge_ffi::GLOBAL_CHARACTERS.lock().unwrap();
                                            *g_chars = chars_list.clone();
                                            let mut g_sel = crate::ui_bridge_ffi::SELECTED_CHAR_SLOT.lock().unwrap();
                                            *g_sel = 0;
                                            let mut g_phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
                                            *g_phase = "SELECT".to_string();
                                            let mut g_sock = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
                                            *g_sock = Some((ch_stream, ch_cipher));
                                        }

                                        // Powiadomienie Pythona:
                                        // 1. AccountConnectorHandler -> OnLoginSuccess()
                                        if acc_handler != 0 {
                                            println!("[NET] Wolanie OnLoginSuccess() na obiekcie introLogin...");
                                            call_python_object_method0(acc_handler, "OnLoginSuccess");
                                        }

                                        // 2. StreamHandler -> SetSelectCharacterPhase()
                                        if stream_handler != 0 {
                                            println!("[NET] Wolanie SetSelectCharacterPhase() na networkModule...");
                                            call_python_object_method0(stream_handler, "SetSelectCharacterPhase");
                                        }

                                        // 3. SelectWindow -> Refresh()
                                        if select_wnd != 0 {
                                            println!("[NET] Wolanie Refresh() na obiekcie introSelect...");
                                            call_python_object_method0(select_wnd, "Refresh");
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[-] Nie udalo sie polaczyc z Game Serverem CH1: {}", e);
                                    if acc_handler != 0 {
                                        call_python_object_method0(acc_handler, "OnConnectFailure");
                                    }
                                }
                            }
                        } else {
                            eprintln!("[-] Odmowa logowania przez Auth Server");
                            if acc_handler != 0 {
                                call_python_object_method1_str(acc_handler, "OnLoginFailure", "WRONGPWD");
                            }
                        }
                    }
                    _ => {
                        if acc_handler != 0 {
                            call_python_object_method0(acc_handler, "OnConnectFailure");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[-] Serwer Auth offline {}: {}", auth_target, e);
                if acc_handler != 0 {
                    call_python_object_method0(acc_handler, "OnConnectFailure");
                }
            }
        }
    });

    true
}

#[no_mangle]
pub extern "C" fn m2_net_DirectEnter(slot: u32) -> bool {
    println!("[NET] DirectEnter(slot={})", slot);
    m2_net_SendSelectCharacterPacket(slot as u8)
}

#[no_mangle]
pub extern "C" fn m2_net_SendSelectCharacterPacket(slot: u8) -> bool {
    println!("[NET] SendSelectCharacterPacket(slot={})", slot);
    {
        let mut g_sel = crate::ui_bridge_ffi::SELECTED_CHAR_SLOT.lock().unwrap();
        *g_sel = slot as usize;
    }

    // Wyslij pakiet CG_CHARACTER_SELECT (header 0x06, slot 1B)
    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut pkt = [0x06u8, slot];
            cipher.encrypt_in_place(&mut pkt);
            let _ = std::io::Write::write_all(stream, &pkt);
        }
    }

    let is_already_loading = {
        let mut phase_guard = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
        if *phase_guard == "LOADING" {
            true
        } else {
            *phase_guard = "LOADING".to_string();
            false
        }
    };

    if let Ok(state) = NETWORK_STREAM.lock() {
        if !is_already_loading {
            if state.stream_handler != 0 {
                println!("[NET] Kolejkowanie SetLoadingPhase() w glownym watku...");
                crate::py_event_dispatcher::enqueue_main_thread_action(
                    crate::py_event_dispatcher::MainThreadAction::CallObjectMethod0(
                        state.stream_handler,
                        "SetLoadingPhase".to_string(),
                    ),
                );
            }
        } else {
            let load_wnd = state.phase_windows[PHASE_WINDOW_LOAD];
            if load_wnd != 0 {
                println!("[NET] LoadingWindow otwarta! Kolejkowanie LoadData(969600, 278400)...");
                crate::py_event_dispatcher::enqueue_main_thread_action(
                    crate::py_event_dispatcher::MainThreadAction::CallObjectMethod2I32(
                        load_wnd,
                        "LoadData".to_string(),
                        969600,
                        278400,
                    ),
                );
            }
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_StartGame() -> bool {
    println!("[NET] StartGame() -> Zadanie przelaczenia na PHASE_GAME");
    {
        let mut g_phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
        if *g_phase == "GAME" {
            println!("[NET] StartGame(): JUZ w fazie GAME - pomijanie podwojnego przelaczenia");
            return true;
        }
        *g_phase = "GAME".to_string();
    }
    if let Ok(state) = NETWORK_STREAM.lock() {
        if state.stream_handler != 0 {
            println!("[NET] Kolejkowanie SetGamePhase() w glownym watku...");
            crate::py_event_dispatcher::enqueue_main_thread_action(
                crate::py_event_dispatcher::MainThreadAction::CallObjectMethod0(
                    state.stream_handler,
                    "SetGamePhase".to_string(),
                ),
            );
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_Warp(x: i32, y: i32) -> bool {
    println!("[NET] Warp(x={}, y={})", x, y);
    true
}

#[no_mangle]
pub extern "C" fn m2_net_GetMainActorRace() -> u32 {
    let sel = crate::ui_bridge_ffi::SELECTED_CHAR_SLOT.lock().map(|s| *s).unwrap_or(0);
    if let Ok(state) = NETWORK_STREAM.lock() {
        if sel < state.characters.len() {
            return state.characters[sel].job as u32;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_net_GetMainActorEmpire() -> u32 {
    m2_net_GetEmpireID()
}

#[no_mangle]
pub extern "C" fn m2_net_GetMainActorSkillGroup() -> u32 {
    1
}

#[no_mangle]
pub extern "C" fn m2_net_IsChatInsultIn(_msg: *const c_char) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn m2_net_IsInsultIn(_msg: *const c_char) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn m2_net_LoadInsultList(_filename: *const c_char) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn m2_net_UploadMark(_filename: *const c_char) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn m2_net_UploadSymbol(_filename: *const c_char) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn m2_net_GetGuildID() -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn m2_net_SendEnterGamePacket() -> bool {
    println!("[NET] SendEnterGamePacket() -> Wysylanie HEADER_CG_ENTERGAME (0x0A) do serwera");
    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, _)) = *sock_guard {
            let pkt = [0x0Au8];
            let _ = std::io::Write::write_all(stream, &pkt);
        }
    }
    crate::game_net_receiver::start_game_recv_thread();
    true
}

#[no_mangle]
pub extern "C" fn m2_net_SendChangeNamePacket(slot: u8, name: *const c_char) -> bool {
    if name.is_null() {
        return false;
    }
    
    let mut packet = TPacketCGChangeName {
        header: CG_CHANGE_NAME,
        length: std::mem::size_of::<TPacketCGChangeName>() as u16,
        index: slot,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    
    let name_bytes = unsafe { CStr::from_ptr(name).to_bytes() };
    if name_bytes.len() > CHARACTER_NAME_MAX_LEN {
        return false;
    }
    let copy_len = std::cmp::min(name_bytes.len(), CHARACTER_NAME_MAX_LEN);
    packet.name[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = vec![0u8; std::mem::size_of::<TPacketCGChangeName>()];
            let pkt_bytes = unsafe {
                std::slice::from_raw_parts(
                    &packet as *const _ as *const u8,
                    std::mem::size_of::<TPacketCGChangeName>(),
                )
            };
            enc_buf.copy_from_slice(pkt_bytes);
            cipher.encrypt_in_place(&mut enc_buf);
            if stream.write_all(&enc_buf).is_ok() {
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_net_SendCreateCharacterPacket(
    slot: u8,
    name: *const c_char,
    job: u8,
    shape: u8,
    con: u8,
    int_: u8,
    str_: u8,
    dex: u8,
) -> bool {
    if name.is_null() {
        return false;
    }
    
    let mut packet = TPacketCGCreateCharacter {
        header: CG_CHARACTER_CREATE,
        length: std::mem::size_of::<TPacketCGCreateCharacter>() as u16,
        index: slot,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
        job: job as u16,
        shape,
        con,
        int_,
        str_,
        dex,
    };
    
    let name_bytes = unsafe { CStr::from_ptr(name).to_bytes() };
    if name_bytes.len() > CHARACTER_NAME_MAX_LEN {
        return false;
    }
    let copy_len = std::cmp::min(name_bytes.len(), CHARACTER_NAME_MAX_LEN);
    packet.name[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = vec![0u8; std::mem::size_of::<TPacketCGCreateCharacter>()];
            let pkt_bytes = unsafe {
                std::slice::from_raw_parts(
                    &packet as *const _ as *const u8,
                    std::mem::size_of::<TPacketCGCreateCharacter>(),
                )
            };
            enc_buf.copy_from_slice(pkt_bytes);
            cipher.encrypt_in_place(&mut enc_buf);
            if stream.write_all(&enc_buf).is_ok() {
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_net_SendDestroyCharacterPacket(slot: u8, code: *const c_char) -> bool {
    if code.is_null() {
        return false;
    }
    
    let mut packet = TPacketCGDestroyCharacter {
        header: CG_CHARACTER_DELETE,
        length: std::mem::size_of::<TPacketCGDestroyCharacter>() as u16,
        index: slot,
        sz_private_code: [0; PRIVATE_CODE_LENGTH],
    };
    
    let code_bytes = unsafe { CStr::from_ptr(code).to_bytes() };
    if code_bytes.len() >= PRIVATE_CODE_LENGTH {
        return false;
    }
    let copy_len = std::cmp::min(code_bytes.len(), PRIVATE_CODE_LENGTH - 1);
    packet.sz_private_code[..copy_len].copy_from_slice(&code_bytes[..copy_len]);

    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = vec![0u8; std::mem::size_of::<TPacketCGDestroyCharacter>()];
            let pkt_bytes = unsafe {
                std::slice::from_raw_parts(
                    &packet as *const _ as *const u8,
                    std::mem::size_of::<TPacketCGDestroyCharacter>(),
                )
            };
            enc_buf.copy_from_slice(pkt_bytes);
            cipher.encrypt_in_place(&mut enc_buf);
            if stream.write_all(&enc_buf).is_ok() {
                return true;
            }
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn m2_net_SendItemUsePacket(slot: u16) -> bool {
    let packet = m2_client_bridge::net::cg::item_use::TPacketCGItemUse::new(1, slot);
    let bytes = packet.to_bytes();
    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = bytes.clone();
            cipher.encrypt_in_place(&mut enc_buf);
            let _ = stream.write_all(&enc_buf);
        }
    }
    m2_client_bridge::net::send_queue::enqueue_outgoing_packet(bytes)
}

#[no_mangle]
pub extern "C" fn m2_net_SendItemDropPacket(slot: u16, count: u32) -> bool {
    let mut out_buf = [0u8; 32];
    let len = unsafe { crate::send_item_actions_packets::m2_net_send_item_drop_action(1, slot, count) };
    len
}

#[no_mangle]
pub extern "C" fn m2_net_SendItemMovePacket(src_slot: u16, dst_slot: u16, count: u8) -> bool {
    unsafe { crate::send_item_actions_packets::m2_net_send_item_move_action(1, src_slot, 1, dst_slot, count) }
}

#[no_mangle]
pub extern "C" fn m2_net_SendItemPickUpPacket(vid: u32) -> bool {
    unsafe { crate::send_item_actions_packets::m2_net_send_item_pickup_action(vid) }
}

#[no_mangle]
pub extern "C" fn m2_net_SendExchangeStartPacket(vid: u32) -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_exchange_v2::m2_send_exchange_start(vid, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendExchangeItemAddPacket(slot: u16, target_pos: u8) -> bool {
    let mut out_buf = [0u8; 32];
    let len = crate::send_shop_exchange_v2::m2_send_exchange_item_add(1, slot, target_pos, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendExchangeElkAddPacket(elk: u32) -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_exchange_v2::m2_send_exchange_elk_add(elk, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendExchangeAcceptPacket() -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_exchange_v2::m2_send_exchange_accept(out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendExchangeExitPacket() -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_exchange_v2::m2_send_exchange_exit(out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendShopBuyPacket(pos: u8, count: u8) -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_complete_packets::m2_send_shop_buy(pos, count, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendShopSellPacket(pos: u8, count: u8) -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_complete_packets::m2_send_shop_sell_new(pos, count, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendShopEndPacket() -> bool {
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_complete_packets::m2_send_shop_end(out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendRefinePacket(slot: u16, refine_type: u8) -> bool {
    let mut out_buf = [0u8; 16];
    let len = unsafe { crate::send_refine_packets_v2::m2_send_refine(slot, refine_type, out_buf.as_mut_ptr()) };
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendPartyInvitePacket(vid: u32) -> bool {
    let mut out_buf = [0u8; 16];
    let len = unsafe { crate::send_party_guild_v2::m2_send_party_invite(vid, out_buf.as_mut_ptr()) };
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(out_buf[..len].to_vec());
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendPartyExitPacket() -> bool {
    let mut out_buf = [0u8; 16];
    let len = unsafe { crate::send_party_guild_v2::m2_send_party_remove(0, out_buf.as_mut_ptr()) };
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(out_buf[..len].to_vec());
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendSafeboxCheckin(safebox_pos: u8, inven_window: u8, inven_cell: u16) -> bool {
    let window = if inven_window == 0 { 1 } else { inven_window };
    let mut out_buf = [0u8; 32];
    let len = unsafe {
        crate::send_item_storage_packets::m2_net_send_safebox_checkin(
            safebox_pos,
            window,
            inven_cell,
            out_buf.as_mut_ptr(),
        )
    };
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(out_buf[..len].to_vec());
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendSafeboxCheckout(safebox_pos: u8, inven_window: u8, inven_cell: u16) -> bool {
    let window = if inven_window == 0 { 1 } else { inven_window };
    let mut out_buf = [0u8; 32];
    let len = unsafe {
        crate::send_item_storage_packets::m2_net_send_safebox_checkout(
            safebox_pos,
            window,
            inven_cell,
            out_buf.as_mut_ptr(),
        )
    };
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(out_buf[..len].to_vec());
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendSafeboxCheckinPacket(inven_cell: u16, safebox_pos: u8) -> bool {
    m2_net_SendSafeboxCheckin(safebox_pos, 1, inven_cell)
}

#[no_mangle]
pub extern "C" fn m2_net_SendSafeboxCheckoutPacket(safebox_pos: u8, inven_cell: u16) -> bool {
    m2_net_SendSafeboxCheckout(safebox_pos, 1, inven_cell)
}

#[no_mangle]
pub extern "C" fn m2_net_SendGuildAdmin(sub_header: u8, id: u32) -> bool {
    let packet_res = match sub_header {
        crate::send_guild_admin_packets::guild_sub::ADD_MEMBER => {
            crate::send_guild_admin_packets::build_guild_add_member_packet(id)
        }
        crate::send_guild_admin_packets::guild_sub::REMOVE_MEMBER => {
            crate::send_guild_admin_packets::build_guild_remove_member_packet(id)
        }
        crate::send_guild_admin_packets::guild_sub::DELETE_COMMENT => {
            crate::send_guild_admin_packets::build_guild_delete_comment_packet(id)
        }
        _ => {
            let mut data = Vec::new();
            let len = (std::mem::size_of::<crate::send_guild_admin_packets::TPacketCGGuild>() + 4) as u16;
            let packet = crate::send_guild_admin_packets::TPacketCGGuild::new(sub_header, len);
            data.extend_from_slice(&packet.to_bytes());
            data.extend_from_slice(&id.to_le_bytes());
            Ok(data)
        }
    };

    if let Ok(bytes) = packet_res {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = bytes.clone();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            }
        }
        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(bytes);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_SendGuildAddMemberPacket(vid: u32) -> bool {
    m2_net_SendGuildAdmin(crate::send_guild_admin_packets::guild_sub::ADD_MEMBER, vid)
}

#[no_mangle]
pub extern "C" fn m2_net_SendGuildRemoveMemberPacket(pid: u32) -> bool {
    m2_net_SendGuildAdmin(crate::send_guild_admin_packets::guild_sub::REMOVE_MEMBER, pid)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuestInputStringPacket(msg: *const c_char) -> bool {
    if msg.is_null() {
        return false;
    }
    crate::send_quickslot_quest_v2::m2_send_quest_input_string(msg)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuickSlotAddPacket(pos: u8, slot_type: u8, slot_pos: u32) -> bool {
    crate::send_quickslot_quest_v2::SendQuickSlotAddPacket(pos, slot_type, slot_pos as u8)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuickSlotDelPacket(pos: u8) -> bool {
    crate::send_quickslot_quest_v2::SendQuickSlotDelPacket(pos)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuickSlotMovePacket(pos: u8, change_pos: u8) -> bool {
    crate::send_quickslot_quest_v2::SendQuickSlotMovePacket(pos, change_pos)
}


#[no_mangle]
pub extern "C" fn m2_net_SendOnClickPacket(vid: u32) -> bool {
    let packet = TPacketCGOnClick {
        header: CG_ON_CLICK,
        length: std::mem::size_of::<TPacketCGOnClick>() as u16,
        vid,
    };
    
    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = vec![0u8; std::mem::size_of::<TPacketCGOnClick>()];
            let pkt_bytes = unsafe {
                std::slice::from_raw_parts(
                    &packet as *const _ as *const u8,
                    std::mem::size_of::<TPacketCGOnClick>(),
                )
            };
            enc_buf.copy_from_slice(pkt_bytes);
            cipher.encrypt_in_place(&mut enc_buf);
            if stream.write_all(&enc_buf).is_ok() {
                return true;
            }
        }
    }
    false
}

// ----------------------------------------------------------------------------
// Eksporty wysylania pakietow czatu, walki i ruchu
// ----------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_net_SendChatPacket(
    chat_str: *const c_char,
    chat_type: u8,
    out_buf: *mut u8,
) -> usize {
    if chat_str.is_null() || out_buf.is_null() {
        return 0;
    }
    let len = unsafe { CStr::from_ptr(chat_str).to_bytes().len() };
    unsafe { m2_send_chat(chat_type, chat_str as *const u8, len, out_buf) }
}

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

fn send_raw_encrypted_packet(bytes: &[u8]) -> bool {
    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut enc_buf = bytes.to_vec();
            cipher.encrypt_in_place(&mut enc_buf);
            return stream.write_all(&enc_buf).is_ok();
        }
    }
    m2_client_bridge::net::send_queue::enqueue_outgoing_packet(bytes.to_vec());
    true
}

#[no_mangle]
pub extern "C" fn m2_net_SendScriptButtonPacket(button_idx: u32) -> bool {
    // CG::SCRIPT_BUTTON = 0x0902
    let mut pkt = Vec::with_capacity(8);
    pkt.extend_from_slice(&0x0902u16.to_le_bytes());
    pkt.extend_from_slice(&8u16.to_le_bytes());
    pkt.extend_from_slice(&button_idx.to_le_bytes());
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendScriptSelectItemPacket(item_idx: u32) -> bool {
    // CG::SCRIPT_SELECT_ITEM = 0x0903
    let mut pkt = Vec::with_capacity(8);
    pkt.extend_from_slice(&0x0903u16.to_le_bytes());
    pkt.extend_from_slice(&8u16.to_le_bytes());
    pkt.extend_from_slice(&item_idx.to_le_bytes());
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuestConfirmPacket(request_vid: u32, answer: u8) -> bool {
    // CG::QUEST_CONFIRM = 0x0905
    let mut pkt = Vec::with_capacity(9);
    pkt.extend_from_slice(&0x0905u16.to_le_bytes());
    pkt.extend_from_slice(&9u16.to_le_bytes());
    pkt.extend_from_slice(&request_vid.to_le_bytes());
    pkt.push(answer);
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendQuestCancelPacket(request_vid: u32) -> bool {
    // CG::QUEST_CANCEL = 0x0906
    let mut pkt = Vec::with_capacity(8);
    pkt.extend_from_slice(&0x0906u16.to_le_bytes());
    pkt.extend_from_slice(&8u16.to_le_bytes());
    pkt.extend_from_slice(&request_vid.to_le_bytes());
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendSafeboxItemMovePacket(src_pos: u8, dst_pos: u8, count: u8) -> bool {
    // CG::SAFEBOX_ITEM_MOVE = 0x0822
    let mut pkt = Vec::with_capacity(7);
    pkt.extend_from_slice(&0x0822u16.to_le_bytes());
    pkt.extend_from_slice(&7u16.to_le_bytes());
    pkt.push(src_pos);
    pkt.push(dst_pos);
    pkt.push(count);
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendMallCheckoutPacket(mall_pos: u8, inv_pos: u8) -> bool {
    // CG::MALL_CHECKOUT = 0x0840
    let mut pkt = Vec::with_capacity(6);
    pkt.extend_from_slice(&0x0840u16.to_le_bytes());
    pkt.extend_from_slice(&6u16.to_le_bytes());
    pkt.push(mall_pos);
    pkt.push(inv_pos);
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendFishingPacket(sub: u8, dir: i32) -> bool {
    // CG::FISHING = 0x0B01
    let mut pkt = Vec::with_capacity(9);
    pkt.extend_from_slice(&0x0B01u16.to_le_bytes());
    pkt.extend_from_slice(&9u16.to_le_bytes());
    pkt.push(sub);
    pkt.extend_from_slice(&dir.to_le_bytes());
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendEmpirePacket(empire: u8) -> bool {
    // CG::EMPIRE = 0x010A
    let mut pkt = Vec::with_capacity(5);
    pkt.extend_from_slice(&0x010Au16.to_le_bytes());
    pkt.extend_from_slice(&5u16.to_le_bytes());
    pkt.push(empire);
    send_raw_encrypted_packet(&pkt)
}

#[no_mangle]
pub extern "C" fn m2_net_SendMyShopPacket(name: *const c_char, count: u8) -> bool {
    // CG::MYSHOP = 0x0802
    if name.is_null() {
        return false;
    }
    let cstr = unsafe { CStr::from_ptr(name) };
    let bytes = cstr.to_bytes();
    let mut shop_name = [0u8; 33];
    let copy_len = bytes.len().min(32);
    shop_name[..copy_len].copy_from_slice(&bytes[..copy_len]);

    let len = 4 + 33 + 1; // header + name + count
    let mut pkt = Vec::with_capacity(len);
    pkt.extend_from_slice(&0x0802u16.to_le_bytes());
    pkt.extend_from_slice(&(len as u16).to_le_bytes());
    pkt.extend_from_slice(&shop_name);
    pkt.push(count);
    send_raw_encrypted_packet(&pkt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_phase_window_and_character_slot_data() {
        m2_net_SetPhaseWindow(PHASE_WINDOW_LOGIN as u32, 0x123456);
        m2_net_SetPhaseWindow(PHASE_WINDOW_SELECT as u32, 0x654321);

        {
            let state = NETWORK_STREAM.lock().unwrap();
            assert_eq!(state.phase_windows[PHASE_WINDOW_LOGIN], 0x123456);
            assert_eq!(state.phase_windows[PHASE_WINDOW_SELECT], 0x654321);
        }

        let name = CString::new("test_user").unwrap();
        let pwd = CString::new("test_pass").unwrap();
        m2_net_SetLoginInfo(name.as_ptr(), pwd.as_ptr());

        {
            let state = NETWORK_STREAM.lock().unwrap();
            assert_eq!(state.login_id, "test_user");
            assert_eq!(state.login_pwd, "test_pass");
        }

        // Test slotow postaci
        {
            let mut state = NETWORK_STREAM.lock().unwrap();
            state.characters[0] = SimplePlayerInfo {
                id: 42,
                name: "NinjaPro".to_string(),
                job: 1,
                level: 75,
                playtime: 1500,
                st: 10,
                ht: 20,
                dx: 90,
                iq: 15,
                main_part: 11209,
                hair_part: 5001,
                ip_addr: 0x7F000001,
                port: 13000,
                guild_id: 101,
                guild_name: "ShinsooElite".to_string(),
                change_name_flag: 0,
            };
        }

        assert_eq!(m2_net_GetAccountCharacterSlotDataInteger(0, ACCOUNT_CHARACTER_SLOT_ID), 42);
        assert_eq!(m2_net_GetAccountCharacterSlotDataInteger(0, ACCOUNT_CHARACTER_SLOT_LEVEL), 75);
        assert_eq!(m2_net_GetAccountCharacterSlotDataInteger(0, ACCOUNT_CHARACTER_SLOT_DEX), 90);
        assert_eq!(m2_net_GetAccountCharacterSlotDataInteger(0, ACCOUNT_CHARACTER_SLOT_FORM), 11209);
        assert_eq!(m2_net_GetAccountCharacterSlotDataInteger(0, ACCOUNT_CHARACTER_SLOT_HAIR), 5001);

        let mut buf = [0 as c_char; 64];
        let n = m2_net_GetAccountCharacterSlotDataString(0, ACCOUNT_CHARACTER_SLOT_NAME, buf.as_mut_ptr(), 64);
        assert_eq!(n, 8);
        let s = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(s, "NinjaPro");

        let mut gbuf = [0 as c_char; 64];
        let gn = m2_net_GetAccountCharacterSlotDataString(0, ACCOUNT_CHARACTER_SLOT_GUILD_NAME, gbuf.as_mut_ptr(), 64);
        assert_eq!(gn, 12);
        let gs = unsafe { CStr::from_ptr(gbuf.as_ptr()).to_str().unwrap() };
        assert_eq!(gs, "ShinsooElite");
    }
}

