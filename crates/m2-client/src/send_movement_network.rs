use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;

pub const MOVE_STOP: u8 = 0; // func dla zatrzymania ruchu, np. FUNC_WAIT

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGMove {
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

impl TPacketCGMove {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; 27] {
        let mut buf = [0u8; 27];
        buf[0..2].copy_from_slice(&self.header.to_le_bytes());
        buf[2..4].copy_from_slice(&self.length.to_le_bytes());
        buf[4] = self.b_func;
        buf[5] = self.b_arg;
        buf[6] = self.b_rot;
        buf[7..11].copy_from_slice(&self.dw_vid.to_le_bytes());
        buf[11..15].copy_from_slice(&self.l_x.to_le_bytes());
        buf[15..19].copy_from_slice(&self.l_y.to_le_bytes());
        buf[19..23].copy_from_slice(&self.dw_time.to_le_bytes());
        buf[23..27].copy_from_slice(&self.dw_duration.to_le_bytes());
        buf
    }
}

// Globalny stan dławienia (throttlingu) wysyłania pakietów
lazy_static::lazy_static! {
    static ref LAST_MOVE_PACKET_TIME: Mutex<Option<Instant>> = Mutex::new(None);
}

// Globalny stan (mock dla vid)
pub fn get_my_vid() -> u32 {
    // W prawdziwym środowisku pobrałoby to VID gracza.
    // Używamy zera dla bezpieczeństwa, jeśli nie mamy konkretnego.
    0
}

/// Resetuje throttling (np. używane w testach).
pub fn reset_movement_throttling() {
    if let Ok(mut lock) = LAST_MOVE_PACKET_TIME.lock() {
        *lock = None;
    }
}

/// Funkcja wysyłająca pakiet ruchu z uwzględnieniem throttlingu.
pub fn send_move_packet(
    func: u8,
    arg: u8,
    rot: f32,
    x: i32,
    y: i32,
    duration: u32,
) -> Result<(), String> {
    let now = Instant::now();

    // Sprawdzanie throttlingu
    if func != MOVE_STOP {
        if let Ok(mut last_time) = LAST_MOVE_PACKET_TIME.lock() {
            if let Some(time) = *last_time {
                if now.duration_since(time) < Duration::from_millis(100) {
                    return Ok(()); // Ignorujemy ten pakiet, jesteśmy throttlowani
                }
            }
            *last_time = Some(now);
        }
    } else {
        // Natychmiastowe wysłanie (np. przy zatrzymaniu), omijamy throttling,
        // ale też resetujemy/aktualizujemy zegar, aby zablokować spam STOPów.
        if let Ok(mut last_time) = LAST_MOVE_PACKET_TIME.lock() {
            *last_time = Some(now);
        }
    }

    // Normalizacja kąta do przedziału <0.0, 360.0) przed konwersją
    let mut rot_norm = rot % 360.0;
    if rot_norm < 0.0 {
        rot_norm += 360.0;
    }

    // Konwersja kąta f32 na u8
    let rot_u8 = ((rot_norm / 360.0) * 255.0).round() as u8;

    let time = crate::app_timing_clock::m2_get_time() as u32;
    let vid = get_my_vid(); // Powinno w normalnych warunkach pobierać dwVID gracza, uzywamy 0 (lub pobierz wlasny vid).

    let packet = TPacketCGMove {
        header: 0x0301,
        length: 27,
        b_func: func,
        b_arg: arg,
        b_rot: rot_u8,
        dw_vid: vid,
        l_x: x,
        l_y: y,
        dw_time: time,
        dw_duration: duration,
    };

    let bytes = packet.to_bytes();

    let mut guard = ACTIVE_GAME_SOCKET
        .lock()
        .map_err(|_| "Failed to lock socket".to_string())?;

    if let Some((stream, cipher)) = guard.as_mut() {
        let mut encrypted_bytes = bytes.to_vec();
        cipher.encrypt_in_place(&mut encrypted_bytes);
        stream
            .write_all(&encrypted_bytes)
            .map_err(|e| format!("Failed to send move packet: {}", e))?;
        Ok(())
    } else {
        Err("Active game socket not found".to_string())
    }
}
