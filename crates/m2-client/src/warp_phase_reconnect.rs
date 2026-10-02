use std::mem::size_of;
use m2_client_bridge::state::phase::{ClientPhase, GLOBAL_PHASE_MANAGER};
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use crate::recv_warp_phase_v2::GLOBAL_WARP_TRANSITION;
use crate::game_net_receiver::stop_game_recv_thread;
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use crate::secure_cipher::SecureCipher;
use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCWarp {
    pub header: u16,
    pub length: u16,
    pub lX: i32,
    pub lY: i32,
    pub lAddr: u32,
    pub wPort: u16,
}

pub fn process_warp_packet(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCWarp>() {
        return Err("Buffer too short");
    }

    let lX = i32::from_le_bytes(data[4..8].try_into().unwrap());
    let lY = i32::from_le_bytes(data[8..12].try_into().unwrap());
    let lAddr = u32::from_le_bytes(data[12..16].try_into().unwrap());
    let wPort = u16::from_le_bytes(data[16..18].try_into().unwrap());

    // 2. Przelaczenie fazy gry na PHASE_LOADING
    if let Ok(mut pm) = GLOBAL_PHASE_MANAGER.lock() {
        let _ = pm.set_phase(ClientPhase::Loading);
    }

    // Zatrzymanie watku `game_net_receiver`.
    stop_game_recv_thread();

    // Wyczyszczenie wszystkich zspawnowanych aktorow i obiektow z pamieci.
    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.reset();
    }

    // Zablokuj fizyke i zaktualizuj pozycje docelowa
    if let Ok(mut wt) = GLOBAL_WARP_TRANSITION.lock() {
        wt.physics_locked = true;
        // In original client, map base might change, here we just update for the warp logic
        // The original code actually does CPythonNetworkStream::Connect
    }
    
    // Ustanowienie nowego polaczenia TCP
    let ip_str = std::net::Ipv4Addr::from(lAddr).to_string();
    let addr = format!("{}:{}", ip_str, wPort);
    
    println!("[WARP] Ladowanie mapy... laczenie z: {}", addr);
    
    let mut stream = match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(5)) {
        Ok(s) => s,
        Err(_) => return Err("Failed to connect to new server"),
    };
    
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
    
    // Handshake
    let mut handshake_buf = [0u8; 2048];
    let mut ch_read = 0;
    while ch_read < 77 {
        match stream.read(&mut handshake_buf[ch_read..]) {
            Ok(bytes) if bytes > 0 => ch_read += bytes,
            _ => break,
        }
    }
    
    if ch_read < 77 {
        return Err("Failed to complete handshake");
    }
    
    let mut server_pk = [0u8; 32];
    server_pk.copy_from_slice(&handshake_buf[9..41]);
    
    let mut cipher = SecureCipher::new();
    cipher.initialize();
    cipher.compute_client_keys(&server_pk);
    
    // Send client pub key
    let mut client_response = [0u8; 64];
    client_response[0..32].copy_from_slice(&cipher.pk);
    client_response[32..64].copy_from_slice(&cipher.pk);
    
    if stream.write_all(&client_response).is_err() {
        return Err("Failed to send handshake response");
    }
    
    cipher.set_activated(true);
    
    // Update active game socket
    if let Ok(mut sock) = ACTIVE_GAME_SOCKET.lock() {
        *sock = Some((stream.try_clone().unwrap(), cipher.clone()));
    }
    
    // Send CG_ENTERGAME (0x0204)
    let mut enter_pkt = [0u8; 4];
    enter_pkt[0..2].copy_from_slice(&0x0204u16.to_le_bytes()); 
    enter_pkt[2..4].copy_from_slice(&4u16.to_le_bytes());       
    
    cipher.encrypt_in_place(&mut enter_pkt);
    if stream.write_all(&enter_pkt).is_err() {
        return Err("Failed to send enter game packet");
    }
    
    // The loading phase will wait for GC_MAIN_CHARACTER in its own loop or the re-started network thread
    // The C++ original implementation simply does: Connect(lAddr, wPort)
    
    Ok(())
}
