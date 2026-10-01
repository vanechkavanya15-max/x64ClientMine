use std::mem::size_of;
use std::ptr;
use crate::character_manager_core::get_character_manager;

pub const HEADER_CG_ATTACK: u16 = 0x0401;
pub const HEADER_CG_USE_SKILL: u16 = 0x0402;
pub const HEADER_CG_SHOOT: u16 = 0x0403;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGAttack {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub dw_victim_vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_skill_index: u32,
    pub dw_target_vid: u32,
    pub dw_mot_index: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGShoot {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
}

/// Zwraca true, jesli glowna postac nie jest martwa i nie jest ogluszona (STUN).
fn can_main_actor_act() -> bool {
    let mgr_lock = match get_character_manager().read() {
        Ok(guard) => guard,
        Err(_) => return false,
    };
    
    if let Some(main_actor) = mgr_lock.get_main_instance() {
        !main_actor.is_dead && !main_actor.is_stunned
    } else {
        false
    }
}

pub fn send_attack_packet(attack_type: u8, target_vid: u32) -> Result<Vec<u8>, &'static str> {
    if target_vid == 0 {
        return Err("Invalid target VID");
    }

    if !can_main_actor_act() {
        return Err("Main actor cannot act (stunned or dead)");
    }

    let packet = TPacketCGAttack {
        header: HEADER_CG_ATTACK,
        length: size_of::<TPacketCGAttack>() as u16,
        b_type: attack_type,
        dw_victim_vid: target_vid,
    };

    let packet_size = size_of::<TPacketCGAttack>();
    let mut buf = vec![0u8; packet_size];
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGAttack as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );
    }
    
    Ok(buf)
}

pub fn send_use_skill_packet(skill_vnum: u32, target_vid: u32) -> Result<Vec<u8>, &'static str> {
    if target_vid == 0 {
        return Err("Invalid target VID");
    }

    if !can_main_actor_act() {
        return Err("Main actor cannot act (stunned or dead)");
    }

    let packet = TPacketCGUseSkill {
        header: HEADER_CG_USE_SKILL,
        length: size_of::<TPacketCGUseSkill>() as u16,
        dw_skill_index: skill_vnum,
        dw_target_vid: target_vid,
        dw_mot_index: 0,
    };

    let packet_size = size_of::<TPacketCGUseSkill>();
    let mut buf = vec![0u8; packet_size];
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGUseSkill as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );
    }

    Ok(buf)
}

pub fn send_shoot_packet(b_type: u8) -> Result<Vec<u8>, &'static str> {
    if !can_main_actor_act() {
        return Err("Main actor cannot act (stunned or dead)");
    }

    let packet = TPacketCGShoot {
        header: HEADER_CG_SHOOT,
        length: size_of::<TPacketCGShoot>() as u16,
        b_type,
    };

    let packet_size = size_of::<TPacketCGShoot>();
    let mut buf = vec![0u8; packet_size];
    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const TPacketCGShoot as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );
    }

    Ok(buf)
}
