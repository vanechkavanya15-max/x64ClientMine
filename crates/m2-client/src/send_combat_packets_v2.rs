use std::ptr;
use std::mem::size_of;

/// Struktura pakietu ataku (HEADER_CG_ATTACK = 0x0401)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TPacketCGAttack {
    pub header: u16,
    pub b_type: u8,
    pub dw_victim_vid: u32,
}

/// Struktura pakietu strzalu z luku (HEADER_CG_SHOOT = 0x0403)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TPacketCGShoot {
    pub header: u16,
    pub b_type: u8,
}

/// Struktura pakietu uzycia umiejetnosci (HEADER_CG_USE_SKILL = 0x0402)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

pub const HEADER_CG_ATTACK: u16 = 0x0401;
pub const HEADER_CG_USE_SKILL: u16 = 0x0402;
pub const HEADER_CG_SHOOT: u16 = 0x0403;

/// Generuje i kopiuje pakiet ataku do bufora wyjsciowego. Zwraca rozmiar zapisanych danych.
#[no_mangle]
pub extern "C" fn m2_send_attack(b_type: u8, vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGAttack {
        header: HEADER_CG_ATTACK,
        b_type,
        dw_victim_vid: vid,
    };
    
    let packet_size = size_of::<TPacketCGAttack>();
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGAttack as *const u8,
            out_buf,
            packet_size,
        );
    }
    packet_size
}

/// Generuje i kopiuje pakiet strzalu do bufora wyjsciowego. Zwraca rozmiar zapisanych danych.
#[no_mangle]
pub extern "C" fn m2_send_shoot(b_type: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGShoot {
        header: HEADER_CG_SHOOT,
        b_type,
    };
    
    let packet_size = size_of::<TPacketCGShoot>();
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGShoot as *const u8,
            out_buf,
            packet_size,
        );
    }
    packet_size
}

/// Generuje i kopiuje pakiet uzycia umiejetnosci do bufora wyjsciowego. Zwraca rozmiar zapisanych danych.
#[no_mangle]
pub extern "C" fn m2_send_use_skill(skill_index: u32, target_vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    
    let packet = TPacketCGUseSkill {
        header: HEADER_CG_USE_SKILL,
        dw_vnum: skill_index,
        dw_target_vid: target_vid,
    };
    
    let packet_size = size_of::<TPacketCGUseSkill>();
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGUseSkill as *const u8,
            out_buf,
            packet_size,
        );
    }
    packet_size
}
