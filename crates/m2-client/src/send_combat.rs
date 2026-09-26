// Modul wysylania pakietow walki (Faza B) w Rust.
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::net::cg::attack::TPacketCGAttack;
use m2_client_bridge::net::cg::use_skill::TPacketCGUseSkill;
use m2_client_bridge::net::cg::shoot::TPacketCGShoot;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;

/// Funkcja wysylajaca pakiet ataku do serwera (HEADER_CG_ATTACK).
#[no_mangle]
pub extern "C" fn SendAttackPacket(hit_type: u8, target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGAttack::new_simple(hit_type, target_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}

/// Funkcja wysylajaca pakiet uzycia umiejetnosci (HEADER_CG_USE_SKILL).
#[no_mangle]
pub extern "C" fn SendUseSkillPacket(skill_vnum: u32, target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGUseSkill::new(skill_vnum, target_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}

/// Funkcja wysylajaca pakiet strzalu z luku (HEADER_CG_SHOOT).
#[no_mangle]
pub extern "C" fn SendShootPacket(skill_vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShoot::new(skill_vnum as u8);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
