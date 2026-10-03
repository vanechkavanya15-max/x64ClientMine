use std::collections::HashMap;
use std::mem::size_of;
use std::ptr;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

// Pakiet ataku (według specyfikacji: header 0x0401)
pub const HEADER_CG_ATTACK: u16 = 0x0401;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGAttack {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub dw_victim_vid: u32,
    pub b_crc_magic_cube_proc_piece: u8,
    pub b_crc_magic_cube_file_piece: u8,
}

// Pakiet użycia skilla (według specyfikacji: header 0x0402)
pub const HEADER_CG_USE_SKILL: u16 = 0x0402;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

// Pakiet strzału z luku (według specyfikacji: header 0x0403)
pub const HEADER_CG_SHOOT: u16 = 0x0403;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGShoot {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub u_skill: u8,
}

// Rejestr cooldownów skilli
static SKILL_COOLDOWNS: LazyLock<Mutex<HashMap<u32, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Resetuje rejestr cooldownów (przydatne do testów)
pub fn reset_skill_cooldowns() {
    if let Ok(mut cooldowns) = SKILL_COOLDOWNS.lock() {
        cooldowns.clear();
    }
}

/// Generuje i kopiuje pakiet ataku do bufora wyjsciowego. Zwraca rozmiar zapisanych danych.
#[no_mangle]
pub extern "C" fn m2_combat_skill_send_attack(
    b_type: u8,
    victim_vid: u32,
    crc_hash: u32,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet = TPacketCGAttack {
        header: HEADER_CG_ATTACK,
        length: size_of::<TPacketCGAttack>() as u16, // = 11 bytes according to spec layout (2+2+1+4+1+1)
        b_type,
        dw_victim_vid: victim_vid,
        b_crc_magic_cube_proc_piece: (crc_hash & 0xFF) as u8,
        b_crc_magic_cube_file_piece: ((crc_hash >> 8) & 0xFF) as u8,
    };

    let packet_size = size_of::<TPacketCGAttack>();
    unsafe {
        ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    }
    packet_size
}

/// Generuje i kopiuje pakiet uzycia umiejetnosci do bufora. Zwraca rozmiar jesli wyslano, 0 jesli cooldown blokuje.
#[no_mangle]
pub extern "C" fn m2_combat_skill_send_use_skill(
    skill_vnum: u32,
    target_vid: u32,
    cooldown_ms: u64,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let now = Instant::now();

    // Sprawdzanie i aktualizacja cooldownu
    if let Ok(mut cooldowns) = SKILL_COOLDOWNS.lock() {
        if let Some(&last_used) = cooldowns.get(&skill_vnum) {
            if now.duration_since(last_used) < Duration::from_millis(cooldown_ms) {
                // Skill na cooldownie - blokujemy wyslanie
                return 0;
            }
        }
        // Ustawiamy nowy czas uzycia
        cooldowns.insert(skill_vnum, now);
    } else {
        // W przypadku zatrucia mutexa bezpiecznie nie pozwalamy wyslac (fail-safe)
        return 0;
    }

    let packet = TPacketCGUseSkill {
        header: HEADER_CG_USE_SKILL,
        length: size_of::<TPacketCGUseSkill>() as u16, // = 12 bytes
        dw_vnum: skill_vnum,
        dw_target_vid: target_vid,
    };

    let packet_size = size_of::<TPacketCGUseSkill>();
    unsafe {
        ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    }
    packet_size
}

/// Generuje i kopiuje pakiet strzalu do bufora wyjsciowego. Zwraca rozmiar zapisanych danych.
#[no_mangle]
pub extern "C" fn m2_combat_skill_send_shoot(b_type: u8, u_skill: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let packet = TPacketCGShoot {
        header: HEADER_CG_SHOOT,
        length: size_of::<TPacketCGShoot>() as u16, // = 6 bytes
        b_type,
        u_skill,
    };

    let packet_size = size_of::<TPacketCGShoot>();
    unsafe {
        ptr::copy_nonoverlapping(&packet as *const _ as *const u8, out_buf, packet_size);
    }
    packet_size
}