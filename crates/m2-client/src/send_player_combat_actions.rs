#![allow(non_snake_case)]
// Metin2 Send Player Combat Actions
// 1:1 C++ Client Parity for Phase Game Combat Actions

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use std::mem::size_of;

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
    pub b_crc_magic_cube_proc_piece: u8,
    pub b_crc_magic_cube_file_piece: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGShoot {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
}

/// # Safety
/// FFI wrapper to send attack packet
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_player_attack(dw_vid_victim: u32, u_mot_attack: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGAttack {
            header: HEADER_CG_ATTACK,
            length: size_of::<TPacketCGAttack>() as u16,
            b_type: u_mot_attack,
            dw_victim_vid: dw_vid_victim,
            b_crc_magic_cube_proc_piece: 0,
            b_crc_magic_cube_file_piece: 0,
        };

        let packet_size = size_of::<TPacketCGAttack>();
        let mut buf = vec![0u8; packet_size];
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGAttack as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );

        enqueue_outgoing_packet(buf);
        true
    })
}

/// # Safety
/// FFI wrapper to send use skill packet
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_player_use_skill(dw_skill_index: u32, dw_target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGUseSkill {
            header: HEADER_CG_USE_SKILL,
            length: size_of::<TPacketCGUseSkill>() as u16,
            dw_vnum: dw_skill_index,
            dw_target_vid: dw_target_vid,
        };

        let packet_size = size_of::<TPacketCGUseSkill>();
        let mut buf = vec![0u8; packet_size];
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGUseSkill as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );

        enqueue_outgoing_packet(buf);
        true
    })
}

/// # Safety
/// FFI wrapper to send shoot packet
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_player_shoot(u_skill: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShoot {
            header: HEADER_CG_SHOOT,
            length: size_of::<TPacketCGShoot>() as u16,
            b_type: u_skill,
        };

        let packet_size = size_of::<TPacketCGShoot>();
        let mut buf = vec![0u8; packet_size];
        std::ptr::copy_nonoverlapping(
            &packet as *const TPacketCGShoot as *const u8,
            buf.as_mut_ptr(),
            packet_size,
        );

        enqueue_outgoing_packet(buf);
        true
    })
}
