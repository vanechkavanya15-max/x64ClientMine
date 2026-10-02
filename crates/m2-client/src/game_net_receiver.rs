// game_net_receiver.rs - Background Network Receive Thread for GAME Phase
// Odbiera i parsuje pakiety GC z serwera CH1 po wejsciu do swiata gry (ENTERGAME).
// Jest to KRYTYCZNY podsystem - bez niego klient jest calkowicie gluchy.

use std::io::Read;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::secure_cipher::SecureCipher;

// Flaga globalna kontrolujaca dzialanie watku odbioru
pub static RECV_THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

// Bufor zdarzen gry - kolejka komunikatow z serwera do watku renderowania/logiki
pub static GAME_EVENTS: std::sync::Mutex<Vec<GameEvent>> = std::sync::Mutex::new(Vec::new());

// ============================================================================
// Opkody GC (Game -> Client) z packet_headers.h
// ============================================================================
pub mod gc_opcode {
    pub const PING: u16            = 0x0007;
    pub const PHASE: u16           = 0x0008;
    pub const KEY_CHALLENGE: u16   = 0x000B;
    pub const KEY_COMPLETE: u16    = 0x000C;
    
    // Character
    pub const CHARACTER_ADD: u16   = 0x0205;
    pub const CHARACTER_ADD2: u16  = 0x0206;
    pub const CHAR_ADDITIONAL_INFO: u16 = 0x0207;
    pub const CHARACTER_DEL: u16   = 0x0208;
    pub const CHARACTER_UPDATE: u16 = 0x0209;
    pub const CHARACTER_UPDATE2: u16 = 0x020A;
    pub const MAIN_CHARACTER: u16  = 0x0210;
    pub const PLAYER_POINTS: u16   = 0x0214;
    pub const PLAYER_POINT_CHANGE: u16 = 0x0215;
    pub const STUN: u16            = 0x0216;
    pub const DEAD: u16            = 0x0217;
    pub const CHANGE_SPEED: u16    = 0x0218;
    pub const SKILL_LEVEL: u16     = 0x021A;
    pub const SKILL_LEVEL_NEW: u16 = 0x021B;
    
    // Movement
    pub const MOVE: u16            = 0x0302;
    pub const WARP: u16            = 0x0306;
    
    // Combat
    pub const DAMAGE_INFO: u16     = 0x0410;
    
    // Items
    pub const ITEM_DEL: u16        = 0x0510;
    pub const ITEM_SET: u16        = 0x0511;
    pub const ITEM_UPDATE: u16     = 0x0514;
    pub const ITEM_GROUND_ADD: u16 = 0x0515;
    pub const ITEM_GROUND_DEL: u16 = 0x0516;
    pub const QUICKSLOT_ADD: u16   = 0x0519;
    pub const QUICKSLOT_DEL: u16   = 0x051A;
    pub const EXCHANGE: u16        = 0x051C;
    
    // Chat
    pub const CHAT: u16            = 0x0603;
    pub const WHISPER: u16         = 0x0604;
    
    // Social
    pub const PARTY_ADD: u16       = 0x0711;
    pub const PARTY_REMOVE: u16    = 0x0713;
    pub const GUILD: u16           = 0x0730;
    pub const MESSENGER: u16       = 0x0741;
    
    // Shop
    pub const SHOP: u16            = 0x0810;
    
    // Quest
    pub const SCRIPT: u16          = 0x0910;
    pub const QUEST_INFO: u16      = 0x0912;
    
    // UI/Effects
    pub const TARGET: u16          = 0x0A10;
    pub const AFFECT_ADD: u16      = 0x0A20;
    pub const AFFECT_REMOVE: u16   = 0x0A21;
    pub const SEPCIAL_EFFECT: u16  = 0x0A30;
    pub const MOUNT: u16           = 0x0A40;
    pub const NPC_POSITION: u16    = 0x0A50;
    
    // World
    pub const FISHING: u16         = 0x0B10;
    pub const LAND_LIST: u16       = 0x0B12;
    pub const TIME: u16            = 0x0B13;
    pub const CHANNEL: u16         = 0x0B14;
}

// ============================================================================
// Zdarzenia gry - komunikaty z serwera do logiki klienta
// ============================================================================
#[derive(Debug, Clone)]
pub enum GameEvent {
    // Wlasna postac zspawnowana na mapie
    MainCharacterSpawn {
        vid: u32,
        race: u16,
        name: String,
        x: i32,
        y: i32,
        z: i32,
        empire: u8,
        skill_group: u8,
    },
    // Punkty gracza (HP, SP, Level, Exp, Gold, itp.)
    PlayerPoints {
        points: Vec<i32>,
    },
    // Zmiana pojedynczego punktu
    PointChange {
        vid: u32,
        point_type: u8,
        amount: i32,
        value: i32,
    },
    // Nowy aktor (NPC/mob/gracz) pojawia sie w zasiegu
    CharacterAdd {
        vid: u32,
        x: i32,
        y: i32,
        z: i32,
        angle: f32,
        char_type: u8,
        race: u16,
        move_speed: u8,
        attack_speed: u8,
    },
    // Dodatkowe info o postaci (imie, gildia, level)
    CharacterAdditionalInfo {
        vid: u32,
        name: String,
        empire: u8,
        guild_id: u32,
        level: u32,
        pk_mode: u8,
        mount_vnum: u32,
    },
    // Aktor znika z zasiegu
    CharacterDelete {
        vid: u32,
    },
    // Ruch aktora
    CharacterMove {
        vid: u32,
        func: u8,
        arg: u8,
        rot: u8,
        x: i32,
        y: i32,
        time: u32,
        duration: u32,
    },
    // Wiadomosc czatu
    ChatMessage {
        chat_type: u8,
        vid: u32,
        empire: u8,
        message: String,
    },
    // Ping od serwera (wymaga odpowiedzi Pong)
    Ping {
        server_time: u32,
    },
    // Zmiana fazy
    PhaseChange {
        phase: u8,
    },
    // Czas serwera
    ServerTime {
        time: i64,
    },
    // Kanal
    Channel {
        channel: u8,
    },
    // Nieznany pakiet (logowany dla debugowania)
    Unknown {
        opcode: u16,
        length: u16,
    },
}

// ============================================================================
// Glowna funkcja uruchamiajaca watek odbioru pakietow
// ============================================================================
pub fn start_game_recv_thread() {
    // Sprawdz czy watek juz dziala
    if RECV_THREAD_RUNNING.load(Ordering::SeqCst) {
        println!("[NET] Watek odbioru juz dziala, pomijam.");
        return;
    }
    
    // Pobierz socket i cipher z ACTIVE_GAME_SOCKET
    let socket_data = {
        let mut guard = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
        if let Some((ref stream, ref cipher)) = *guard {
            let cloned_stream = stream.try_clone().expect("[NET] Nie udalo sie sklonowac TcpStream");
            let cloned_cipher = cipher.clone();
            Some((cloned_stream, cloned_cipher))
        } else {
            None
        }
    };
    
    let (stream, cipher) = match socket_data {
        Some(data) => data,
        None => {
            eprintln!("[-] Brak aktywnego gniazda gry - nie mozna uruchomic watku odbioru!");
            return;
        }
    };
    
    RECV_THREAD_RUNNING.store(true, Ordering::SeqCst);
    
    std::thread::Builder::new()
        .name("game-net-recv".to_string())
        .spawn(move || {
            println!("[NET] ========================================================");
            println!("[NET] Watek odbioru pakietow GAME uruchomiony!");
            println!("[NET] Oczekiwanie na pakiety GC z serwera CH1...");
            println!("[NET] ========================================================");
            
            game_recv_loop(stream, cipher);
            
            RECV_THREAD_RUNNING.store(false, Ordering::SeqCst);
            println!("[NET] Watek odbioru pakietow zakonczony.");
        })
        .expect("[NET] Nie udalo sie uruchomic watku odbioru");
}

/// Zatrzymuje watek odbioru pakietow
pub fn stop_game_recv_thread() {
    RECV_THREAD_RUNNING.store(false, Ordering::SeqCst);
}

// ============================================================================
// Petla odbioru pakietow - rdzen systemu sieciowego
// ============================================================================
fn game_recv_loop(mut stream: TcpStream, mut cipher: SecureCipher) {
    // Ustawiamy timeout na odczyt, zeby watek mogl sprawdzac flage RUNNING
    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
    
    // Bufor ciaglego odczytu - ring buffer 64KB
    let mut recv_buf = vec![0u8; 65536];
    let mut buf_len: usize = 0;
    
    while RECV_THREAD_RUNNING.load(Ordering::SeqCst) {
        // Sprawdz czy faza gry jest nadal aktywna (LOADING lub GAME)
        {
            let phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
            if phase.as_str() != "GAME" && phase.as_str() != "LOADING" {
                println!("[NET] Faza gry zmieniona na '{}', konczenie watku odbioru.", *phase);
                break;
            }
        }
        
        // Odczyt danych z socketa
        match stream.read(&mut recv_buf[buf_len..]) {
            Ok(0) => {
                println!("[NET] Serwer zamknal polaczenie (recv 0 bajtow).");
                break;
            }
            Ok(n) => {
                // Deszyfrowanie odebranych danych
                cipher.decrypt_in_place(&mut recv_buf[buf_len..buf_len + n]);
                buf_len += n;
                
                // Parsowanie pakietow z bufora
                let consumed = parse_packets(&recv_buf[..buf_len]);
                if consumed > 0 {
                    // Przesuwanie nieprzeparsowanych danych na poczatek bufora
                    recv_buf.copy_within(consumed..buf_len, 0);
                    buf_len -= consumed;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut 
                       || e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout - normalne, kontynuujemy petle
                continue;
            }
            Err(e) => {
                eprintln!("[NET] Blad odczytu z socketa: {} (kind: {:?})", e, e.kind());
                break;
            }
        }
    }
}

// ============================================================================
// Parser pakietow - wyciaga kompletne pakiety z bufora
// ============================================================================
pub fn parse_packets(data: &[u8]) -> usize {
    let mut offset = 0;
    
    while offset + 4 <= data.len() {
        let header = u16::from_le_bytes([data[offset], data[offset + 1]]);
        let length = u16::from_le_bytes([data[offset + 2], data[offset + 3]]) as usize;
        
        // Walidacja dlugosci pakietu
        if length < 4 {
            eprintln!("[NET] Nieprawidlowa dlugosc pakietu: header=0x{:04X}, length={}", header, length);
            // Pomijalem 4 bajty (minimalny naglowek) i probuje dalej
            offset += 4;
            continue;
        }
        
        // Sprawdz czy mamy kompletny pakiet w buforze
        if offset + length > data.len() {
            // Niekompletny pakiet - czekamy na wiecej danych
            break;
        }
        
        // Kompletny pakiet - dispatchuj
        let packet_data = &data[offset..offset + length];
        dispatch_packet(header, length as u16, packet_data);
        
        offset += length;
    }
    
    offset
}

// ============================================================================
// Dispatcher pakietow - kieruje pakiety do odpowiednich handlerow
// ============================================================================
fn dispatch_packet(header: u16, length: u16, data: &[u8]) {
    match header {
        gc_opcode::PING => handle_ping(data),
        gc_opcode::PHASE => handle_phase(data),
        gc_opcode::TIME => handle_time(data),
        gc_opcode::CHANNEL => handle_channel(data),
        gc_opcode::MAIN_CHARACTER => handle_main_character(data),
        gc_opcode::PLAYER_POINTS => handle_player_points(data),
        gc_opcode::PLAYER_POINT_CHANGE => handle_point_change(data),
        gc_opcode::CHARACTER_ADD => handle_character_add(data),
        gc_opcode::CHARACTER_ADD2 => handle_character_add(data), // ten sam format
        gc_opcode::CHAR_ADDITIONAL_INFO => handle_character_additional_info(data),
        gc_opcode::CHARACTER_DEL => handle_character_delete(data),
        gc_opcode::CHARACTER_UPDATE | gc_opcode::CHARACTER_UPDATE2 => handle_character_update(data),
        gc_opcode::MOVE => handle_move(data),
        gc_opcode::CHAT => handle_chat(data),
        gc_opcode::STUN => handle_stun_dead(data, "STUN"),
        gc_opcode::DEAD => handle_stun_dead(data, "DEAD"),
        gc_opcode::CHANGE_SPEED => handle_change_speed(data),
        gc_opcode::ITEM_SET => handle_item_set(data),
        gc_opcode::ITEM_DEL => handle_item_del(data),
        gc_opcode::ITEM_UPDATE => handle_item_update(data),
        gc_opcode::ITEM_GROUND_ADD => handle_item_ground(data, true),
        gc_opcode::ITEM_GROUND_DEL => handle_item_ground(data, false),
        gc_opcode::AFFECT_ADD => handle_affect(data, true),
        gc_opcode::AFFECT_REMOVE => handle_affect(data, false),
        gc_opcode::TARGET => handle_target(data),
        gc_opcode::WARP => handle_warp(data),
        gc_opcode::QUEST_INFO | gc_opcode::SCRIPT
        | gc_opcode::SKILL_LEVEL | gc_opcode::SKILL_LEVEL_NEW
        | gc_opcode::SHOP | gc_opcode::GUILD | gc_opcode::MESSENGER 
        | gc_opcode::PARTY_ADD | gc_opcode::PARTY_REMOVE 
        | gc_opcode::FISHING | gc_opcode::LAND_LIST 
        | gc_opcode::NPC_POSITION | gc_opcode::MOUNT
        | gc_opcode::SEPCIAL_EFFECT | gc_opcode::WHISPER
        | gc_opcode::QUICKSLOT_ADD | gc_opcode::QUICKSLOT_DEL
        | gc_opcode::EXCHANGE | gc_opcode::KEY_CHALLENGE | gc_opcode::KEY_COMPLETE => {
            if !crate::game_net_stream_dispatcher_v2::dispatch_packet_extended_v2(header, length, data) {
                println!("[NET] Pakiet GC: opcode=0x{:04X}, len={} (przekazano do v2)", header, length);
            }
        }
        _ => {
            if !crate::game_net_stream_dispatcher_v2::dispatch_packet_extended_v2(header, length, data) {
                println!("[NET] Nieznany pakiet GC: opcode=0x{:04X}, len={}", header, length);
                push_event(GameEvent::Unknown { opcode: header, length });
            }
        }
    }
}

// ============================================================================
// Handlery poszczegolnych pakietow
// ============================================================================

fn handle_ping(data: &[u8]) {
    // TPacketGCPing: header(2) + length(2) + server_time(4) = 8 bajtow
    if data.len() < 8 { return; }
    let server_time = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    println!("[NET] PING od serwera (time={}), wysylam PONG...", server_time);
    
    push_event(GameEvent::Ping { server_time });
    
    // Wyslij CG::PONG (0x0006) - 4 bajty: header + length
    send_pong();
}

fn handle_phase(data: &[u8]) {
    // TPacketGCPhase: header(2) + length(2) + phase(1) = 5 bajtow
    if data.len() < 5 { return; }
    let phase = data[4];
    println!("[NET] Zmiana fazy serwera: phase={}", phase);
    push_event(GameEvent::PhaseChange { phase });
}

fn handle_time(data: &[u8]) {
    // TPacketGCTime: header(2) + length(2) + time(8 na 64-bit, 4 na 32-bit)
    // Serwer 64-bit: time_t = 8 bajtow
    if data.len() < 8 { return; }
    let time = if data.len() >= 12 {
        i64::from_le_bytes(data[4..12].try_into().unwrap_or_default())
    } else {
        i32::from_le_bytes(data[4..8].try_into().unwrap_or_default()) as i64
    };
    println!("[NET] Czas serwera: {}", time);
    push_event(GameEvent::ServerTime { time });
}

fn handle_channel(data: &[u8]) {
    // TPacketGCChannel: header(2) + length(2) + channel(1) = 5 bajtow
    if data.len() < 5 { return; }
    let channel = data[4];
    println!("[NET] Kanal serwera: CH{}", channel);
    push_event(GameEvent::Channel { channel });
}

fn handle_main_character(data: &[u8]) {
    // TPacketGCMainCharacter: header(2) + length(2) + dwVID(4) + wRaceNum(2)
    // + szName[65] + szBGMName[25] + fBGMVol(4) + lx(4) + ly(4) + lz(4) + empire(1) + skill_group(1)
    // Calkowity rozmiar: 4 + 4 + 2 + 65 + 25 + 4 + 4 + 4 + 4 + 1 + 1 = 118 bajtow
    if data.len() < 30 { return; }
    
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    let race = u16::from_le_bytes(data[8..10].try_into().unwrap_or_default());
    
    // Parsuj imie (65 bajtow od offsetu 10)
    let name_end = data[10..].iter().take(65).position(|&b| b == 0).unwrap_or(65);
    let name = String::from_utf8_lossy(&data[10..10 + name_end]).to_string();
    
    // BGM name (25 bajtow od offsetu 75)
    // BGM volume (4 bajty od offsetu 100)
    let pos_offset = 104; // 10 + 65 + 25 + 4
    let lx = i32::from_le_bytes(data[pos_offset..pos_offset+4].try_into().unwrap_or_default());
    let ly = i32::from_le_bytes(data[pos_offset+4..pos_offset+8].try_into().unwrap_or_default());
    let lz = i32::from_le_bytes(data[pos_offset+8..pos_offset+12].try_into().unwrap_or_default());
    let empire = if data.len() > pos_offset + 12 { data[pos_offset + 12] } else { 0 };
    let skill_group = if data.len() > pos_offset + 13 { data[pos_offset + 13] } else { 0 };
    
    println!("[NET] ========================================================");
    println!("[NET] [GRA] GLOWNA POSTAC GRACZA ZSPAWNOWANA NA MAPIE!");
    println!("[NET] VID: {}, Rasa: {}, Imie: '{}'", vid, race, name);
    println!("[NET] Pozycja: ({}, {}, {}), Imperium: {}", lx, ly, lz, empire);
    println!("[NET] ========================================================");
    
    push_event(GameEvent::MainCharacterSpawn {
        vid, race, name, x: lx, y: ly, z: lz, empire, skill_group,
    });

    // Faza LOADING zakonczona - postac zaladowana z bazy serwera!
    println!("[NET] [+] Postac zaladowana pomyslnie! Odsylam CG_ENTERGAME do CH1...");
    send_entergame();

    // Przelacz faze na GAME
    {
        let mut phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap();
        *phase = "GAME".to_string();
    }
    println!("[+] ========================================================");
    println!("[+] [GRA] WEJSCIE DO SWIATA GRY (PHASE_GAME) ROZPOCZETE!");
    println!("[+] Ladowanie HUDu gracza, paska zadan, czatu i minimapy...");
    println!("[+] ========================================================");

    // Przelaczenie interfejsu na in-game HUD
    {
        let mut wm = crate::ui_window_mgr::get_window_manager();
        crate::ui_bridge_ffi::m2_ui_setup_game_hud(&mut wm, 1024, 768);
    }
}

fn handle_player_points(data: &[u8]) {
    // TPacketGCPoints: header(2) + length(2) + points[255] (255 * 4 = 1020 bajtow)
    // Calkowity rozmiar: 4 + 1020 = 1024 bajty
    if data.len() < 8 { return; }
    
    let num_points = (data.len() - 4) / 4;
    let mut points = Vec::with_capacity(num_points);
    for i in 0..num_points.min(255) {
        let off = 4 + i * 4;
        if off + 4 <= data.len() {
            points.push(i32::from_le_bytes(data[off..off+4].try_into().unwrap_or_default()));
        }
    }
    
    // Wyswietl kluczowe statystyki
    let level = points.get(0).copied().unwrap_or(0);
    let hp = points.get(5).copied().unwrap_or(0);
    let max_hp = points.get(6).copied().unwrap_or(0);
    let sp = points.get(7).copied().unwrap_or(0);
    let max_sp = points.get(8).copied().unwrap_or(0);
    let gold = points.get(14).copied().unwrap_or(0);
    let exp = points.get(12).copied().unwrap_or(0);
    
    println!("[NET] ========================================================");
    println!("[NET] [PUNKTY] Statystyki gracza zaladowane!");
    println!("[NET] Level: {}, HP: {}/{}, SP: {}/{}", level, hp, max_hp, sp, max_sp);
    println!("[NET] Gold: {}, EXP: {}", gold, exp);
    println!("[NET] ========================================================");
    
    // Aktualizuj tekst HUD HP/SP
    update_hud_stats(level, hp, max_hp, sp, max_sp, gold);
    
    push_event(GameEvent::PlayerPoints { points });
}

fn handle_point_change(data: &[u8]) {
    // TPacketGCPointChange: header(2) + length(2) + dwVID(4) + type(1) + amount(4) + value(4) = 17 bajtow
    if data.len() < 15 { return; }
    
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    let point_type = data[8];
    let amount = i32::from_le_bytes(data[9..13].try_into().unwrap_or_default());
    let value = i32::from_le_bytes(data[13..17].try_into().unwrap_or_default());
    
    // Pomin nadmiarowe logowanie dla czestych zmian
    match point_type {
        5 | 6 => println!("[NET] PointChange: HP vid={} amount={} value={}", vid, amount, value),
        7 | 8 => println!("[NET] PointChange: SP vid={} amount={} value={}", vid, amount, value),
        12 => println!("[NET] PointChange: EXP vid={} +{}", vid, amount),
        14 => println!("[NET] PointChange: Gold vid={} amount={} value={}", vid, amount, value),
        _ => {} // ciche dla nieistotnych punktow
    }
    
    push_event(GameEvent::PointChange { vid, point_type, amount, value });
}

fn handle_character_add(data: &[u8]) {
    // TPacketGCCharacterAdd: header(2) + length(2) + dwVID(4) + angle(4f) + x(4) + y(4) + z(4)
    // + bType(1) + wRaceNum(2) + bMovingSpeed(1) + bAttackSpeed(1) + bStateFlag(1) + dwAffectFlag[2](8)
    // = 4 + 4 + 4 + 4 + 4 + 4 + 1 + 2 + 1 + 1 + 1 + 8 = 38 bajtow
    if data.len() < 28 { return; }
    
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    let angle = f32::from_le_bytes(data[8..12].try_into().unwrap_or_default());
    let x = i32::from_le_bytes(data[12..16].try_into().unwrap_or_default());
    let y = i32::from_le_bytes(data[16..20].try_into().unwrap_or_default());
    let z = i32::from_le_bytes(data[20..24].try_into().unwrap_or_default());
    let char_type = data[24];
    let race = u16::from_le_bytes(data[25..27].try_into().unwrap_or_default());
    let move_speed = if data.len() > 27 { data[27] } else { 100 };
    let attack_speed = if data.len() > 28 { data[28] } else { 100 };
    
    let type_name = match char_type {
        0 => "Gracz",
        1 => "NPC",
        2 => "Potwor",
        3 => "Kamien Metinowy",
        6 => "NPC-Drzwi",
        _ => "Inny",
    };
    
    println!("[NET] CharAdd: VID={} {} rasa={} pos=({},{},{}) angle={:.0}", 
             vid, type_name, race, x, y, z, angle);
    
    push_event(GameEvent::CharacterAdd {
        vid, x, y, z, angle, char_type, race, move_speed, attack_speed,
    });
}

fn handle_character_additional_info(data: &[u8]) {
    // TPacketGCCharacterAdditionalInfo: header(2) + length(2) + dwVID(4) + name[65] 
    // + awPart[4](8) + bEmpire(1) + dwGuildID(4) + dwLevel(4) + sAlignment(2) + bPKMode(1) + dwMountVnum(4)
    if data.len() < 20 { return; }
    
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    
    let name_end = data[8..].iter().take(65).position(|&b| b == 0).unwrap_or(65);
    let name = String::from_utf8_lossy(&data[8..8 + name_end]).to_string();
    
    // Offsety po nazwie (8 + 65 = 73)
    let parts_off = 73;
    // awPart[4] = 8 bajtow (73..81)
    let info_off = parts_off + 8; // 81
    
    let empire = if data.len() > info_off { data[info_off] } else { 0 };
    let guild_id = if data.len() >= info_off + 5 {
        u32::from_le_bytes(data[info_off+1..info_off+5].try_into().unwrap_or_default())
    } else { 0 };
    let level = if data.len() >= info_off + 9 {
        u32::from_le_bytes(data[info_off+5..info_off+9].try_into().unwrap_or_default())
    } else { 0 };
    // sAlignment(2) na offset info_off+9
    let pk_mode = if data.len() > info_off + 11 { data[info_off + 11] } else { 0 };
    let mount_vnum = if data.len() >= info_off + 16 {
        u32::from_le_bytes(data[info_off+12..info_off+16].try_into().unwrap_or_default())
    } else { 0 };
    
    println!("[NET] CharInfo: VID={} '{}' Lv.{} Imp.{} Gildia:{}", vid, name, level, empire, guild_id);
    
    push_event(GameEvent::CharacterAdditionalInfo {
        vid, name, empire, guild_id, level, pk_mode, mount_vnum,
    });
}

fn handle_character_delete(data: &[u8]) {
    // TPacketGCCharacterDelete: header(2) + length(2) + id(4) = 8 bajtow
    if data.len() < 8 { return; }
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    println!("[NET] CharDel: VID={}", vid);
    push_event(GameEvent::CharacterDelete { vid });
}

fn handle_character_update(data: &[u8]) {
    // TPacketGCCharacterUpdate: header(2) + length(2) + dwVID(4) + awPart[4](8) + ...
    if data.len() < 8 { return; }
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    // Logujemy tylko - pelna aktualizacja wygladu nie jest krytyczna na teraz
    println!("[NET] CharUpdate: VID={}, len={}", vid, data.len());
}

fn handle_move(data: &[u8]) {
    // TPacketGCMove: header(2) + length(2) + bFunc(1) + bArg(1) + bRot(1) + dwVID(4)
    // + lX(4) + lY(4) + dwTime(4) + dwDuration(4) = 27 bajtow
    if data.len() < 25 { return; }
    
    let func = data[4];
    let arg = data[5];
    let rot = data[6];
    let vid = u32::from_le_bytes(data[7..11].try_into().unwrap_or_default());
    let x = i32::from_le_bytes(data[11..15].try_into().unwrap_or_default());
    let y = i32::from_le_bytes(data[15..19].try_into().unwrap_or_default());
    let time = u32::from_le_bytes(data[19..23].try_into().unwrap_or_default());
    let duration = if data.len() >= 27 {
        u32::from_le_bytes(data[23..27].try_into().unwrap_or_default())
    } else { 0 };
    
    // Nie logujemy kazdego ruchu - za duzo spamu
    push_event(GameEvent::CharacterMove {
        vid, func, arg, rot, x, y, time, duration,
    });
}

fn handle_chat(data: &[u8]) {
    // TPacketGCChat: header(2) + length(2) + type(1) + id(4) + bEmpire(1) + tekst...
    if data.len() < 12 { return; }
    
    let chat_type = data[4];
    let vid = u32::from_le_bytes(data[5..9].try_into().unwrap_or_default());
    let empire = data[9];
    
    // Tekst czatu: od offsetu 10 do konca pakietu, terminowany \0
    let msg_data = &data[10..];
    let msg_end = msg_data.iter().position(|&b| b == 0).unwrap_or(msg_data.len());
    let message = String::from_utf8_lossy(&msg_data[..msg_end]).to_string();
    
    let type_name = match chat_type {
        0 => "Normal",
        1 => "Shout",
        2 => "Info",
        3 => "Whisper",
        4 => "Party",
        5 => "Guild",
        6 => "System",
        7 => "Notice",
        _ => "Unknown",
    };
    
    println!("[CHAT] [{}] VID={}: {}", type_name, vid, message);
    
    push_event(GameEvent::ChatMessage { chat_type, vid, empire, message });
}

fn handle_stun_dead(data: &[u8], kind: &str) {
    if data.len() < 8 { return; }
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    println!("[NET] {}: VID={}", kind, vid);
}

fn handle_change_speed(data: &[u8]) {
    // TPacketGCChangeSpeed: header(2) + length(2) + vid(4) + moving_speed(2) = 10 bajtow
    if data.len() < 10 { return; }
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    let speed = u16::from_le_bytes(data[8..10].try_into().unwrap_or_default());
    println!("[NET] ChangeSpeed: VID={} speed={}", vid, speed);
}

fn handle_item_set(data: &[u8]) {
    if data.len() < 10 { return; }
    // Logujemy tylko ogolem - szczegolowe parsowanie w nastepnym kroku
    println!("[NET] ItemSet: len={}", data.len());
}

fn handle_item_del(data: &[u8]) {
    if data.len() < 7 { return; }
    println!("[NET] ItemDel: len={}", data.len());
}

fn handle_item_update(data: &[u8]) {
    if data.len() < 8 { return; }
    println!("[NET] ItemUpdate: len={}", data.len());
}

fn handle_item_ground(data: &[u8], add: bool) {
    if data.len() < 8 { return; }
    let action = if add { "GroundAdd" } else { "GroundDel" };
    println!("[NET] Item{}: len={}", action, data.len());
}

fn handle_affect(data: &[u8], add: bool) {
    if data.len() < 8 { return; }
    let action = if add { "AffectAdd" } else { "AffectRemove" };
    println!("[NET] {}: len={}", action, data.len());
}

fn handle_target(data: &[u8]) {
    if data.len() < 8 { return; }
    let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
    println!("[NET] Target: VID={}", vid);
}

fn handle_warp(data: &[u8]) {
    if data.len() < 8 { return; }
    println!("[NET] Warp: len={}", data.len());
}

// ============================================================================
// Wysylanie pakietow CG (Client -> Game)
// ============================================================================

pub(crate) fn send_pong() {
    let mut pong = [0u8; 4];
    pong[0..2].copy_from_slice(&0x0006u16.to_le_bytes()); // CG::PONG
    pong[2..4].copy_from_slice(&4u16.to_le_bytes());       // length = 4
    
    let mut sock_guard = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
    if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
        cipher.encrypt_in_place(&mut pong);
        if let Err(e) = std::io::Write::write_all(stream, &pong) {
            eprintln!("[NET] Blad wysylania PONG: {}", e);
        }
    }
}

pub fn send_entergame() {
    let mut enter_pkt = [0u8; 4];
    enter_pkt[0..2].copy_from_slice(&0x0204u16.to_le_bytes()); // CG::ENTERGAME
    enter_pkt[2..4].copy_from_slice(&4u16.to_le_bytes());       // length = 4

    let mut sock_guard = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
    if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
        cipher.encrypt_in_place(&mut enter_pkt);
        if let Err(e) = std::io::Write::write_all(stream, &enter_pkt) {
            eprintln!("[-] Blad wysylania CG_ENTERGAME: {}", e);
        } else {
            println!("[NET] [+] Wyslano CG_ENTERGAME (opcode=0x0204) do CH1 po odebraniu postaci!");
        }
    }
}

// ============================================================================
// Aktualizacja HUD z danymi statystyk
// ============================================================================
fn update_hud_stats(level: i32, hp: i32, max_hp: i32, sp: i32, max_sp: i32, gold: i32) {
    let mut wm = crate::ui_window_mgr::get_window_manager();
    
    // Aktualizuj tekst HP
    if let Some(hp_win) = wm.windows.values_mut().find(|w| w.name == "HudHpText") {
        hp_win.text = format!("{}/{}", hp, max_hp);
    }
    // Aktualizuj tekst SP
    if let Some(sp_win) = wm.windows.values_mut().find(|w| w.name == "HudSpText") {
        sp_win.text = format!("{}/{}", sp, max_sp);
    }
    // Aktualizuj tekst Level
    if let Some(lv_win) = wm.windows.values_mut().find(|w| w.name == "HudLevelText") {
        lv_win.text = format!("Lv. {}", level);
    }
    // Aktualizuj tekst Gold
    if let Some(gold_win) = wm.windows.values_mut().find(|w| w.name == "HudGoldText") {
        gold_win.text = format!("{}", gold);
    }
}

// ============================================================================
// Pomocnicza funkcja do wrzucania zdarzen do kolejki
// ============================================================================
pub(crate) fn push_event(event: GameEvent) {
    if let Ok(mut events) = GAME_EVENTS.lock() {
        // Ograniczamy rozmiar kolejki do 1000 zdarzen
        if events.len() > 1000 {
            events.drain(..500);
        }
        events.push(event);
    }
}

/// Pobiera i oczyszcza wszystkie zdarzenia z kolejki
pub fn drain_events() -> Vec<GameEvent> {
    if let Ok(mut events) = GAME_EVENTS.lock() {
        events.drain(..).collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ping_packet() {
        // Symulacja pakietu PING: header=0x0007, length=8, server_time=12345
        let mut pkt = vec![0u8; 8];
        pkt[0..2].copy_from_slice(&0x0007u16.to_le_bytes());
        pkt[2..4].copy_from_slice(&8u16.to_le_bytes());
        pkt[4..8].copy_from_slice(&12345u32.to_le_bytes());
        
        let consumed = parse_packets(&pkt);
        assert_eq!(consumed, 8);
    }
    
    #[test]
    fn test_parse_incomplete_packet() {
        // Niekompletny pakiet - mamy naglowek ale brak danych
        let pkt = vec![0x07, 0x00, 0x08, 0x00]; // header=0x0007, length=8, ale mamy tylko 4 bajty
        let consumed = parse_packets(&pkt);
        assert_eq!(consumed, 0); // Nie powinno nic skonsumowac
    }
    
    #[test]
    fn test_parse_multiple_packets() {
        // Dwa pakiety PHASE (5 bajtow kazdy)
        let mut pkt = vec![0u8; 10];
        pkt[0..2].copy_from_slice(&0x0008u16.to_le_bytes());
        pkt[2..4].copy_from_slice(&5u16.to_le_bytes());
        pkt[4] = 5; // PHASE_GAME
        pkt[5..7].copy_from_slice(&0x0008u16.to_le_bytes());
        pkt[7..9].copy_from_slice(&5u16.to_le_bytes());
        pkt[9] = 3; // PHASE_SELECT
        
        let consumed = parse_packets(&pkt);
        assert_eq!(consumed, 10);
    }
    
    #[test]
    fn test_drain_events() {
        push_event(GameEvent::Channel { channel: 1 });
        push_event(GameEvent::ServerTime { time: 100 });
        let events = drain_events();
        assert!(events.len() >= 2);
    }
    
    #[test]
    fn test_gc_opcodes() {
        assert_eq!(gc_opcode::PING, 0x0007);
        assert_eq!(gc_opcode::PLAYER_POINTS, 0x0214);
        assert_eq!(gc_opcode::CHARACTER_ADD, 0x0205);
        assert_eq!(gc_opcode::MOVE, 0x0302);
        assert_eq!(gc_opcode::CHAT, 0x0603);
        assert_eq!(gc_opcode::TIME, 0x0B13);
        assert_eq!(gc_opcode::CHANNEL, 0x0B14);
    }
}
