// Testy dla send_combat.rs w m2-client
// Zgodnie z wytycznymi, zero testow typu assert!(is_ready()), pelne pokrycie logiki.

use m2_client::send_combat::{SendAttackPacket, SendShootPacket, SendUseSkillPacket};
use m2_client_bridge::net::send_queue::{clear_outgoing_packets, get_last_outgoing_packet};
use m2_client_bridge::net::cg::attack::TPacketCGAttack;
use m2_client_bridge::net::cg::use_skill::TPacketCGUseSkill;
use m2_client_bridge::net::cg::shoot::TPacketCGShoot;

use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_send_attack_packet() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    let hit_type = 5u8;
    let target_vid = 12345u32;
    
    let result = SendAttackPacket(hit_type, target_vid);
    assert!(result, "SendAttackPacket powinno zwrocic true");
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    
    // Rozmiar powinien zgadzac sie z TPacketCGAttack
    assert_eq!(queued.len(), std::mem::size_of::<TPacketCGAttack>());
    
    // Weryfikacja naglowka w formacie Little-Endian
    let header = u16::from_le_bytes([queued[0], queued[1]]);
    assert_eq!(header, TPacketCGAttack::HEADER);
    assert_eq!(header, 0x0401);
    
    // Weryfikacja length
    let length = u16::from_le_bytes([queued[2], queued[3]]);
    assert_eq!(length, std::mem::size_of::<TPacketCGAttack>() as u16);
    
    // Weryfikacja typu uderzenia
    assert_eq!(queued[4], hit_type);
    
    // Weryfikacja celu (VID)
    let vid_bytes = [queued[5], queued[6], queued[7], queued[8]];
    assert_eq!(u32::from_le_bytes(vid_bytes), target_vid);
}

#[test]
fn test_send_use_skill_packet() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    let skill_vnum = 15u32;
    let target_vid = 54321u32;
    
    let result = SendUseSkillPacket(skill_vnum, target_vid);
    assert!(result, "SendUseSkillPacket powinno zwrocic true");
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    
    // Rozmiar powinien zgadzac sie z TPacketCGUseSkill
    assert_eq!(queued.len(), std::mem::size_of::<TPacketCGUseSkill>());
    
    // Weryfikacja naglowka
    let header = u16::from_le_bytes([queued[0], queued[1]]);
    assert_eq!(header, TPacketCGUseSkill::HEADER);
    assert_eq!(header, 0x0402);
    
    // Weryfikacja length
    let length = u16::from_le_bytes([queued[2], queued[3]]);
    assert_eq!(length, std::mem::size_of::<TPacketCGUseSkill>() as u16);
    
    // Weryfikacja vnum umiejetnosci
    let vnum_bytes = [queued[4], queued[5], queued[6], queued[7]];
    assert_eq!(u32::from_le_bytes(vnum_bytes), skill_vnum);
    
    // Weryfikacja celu (VID)
    let vid_bytes = [queued[8], queued[9], queued[10], queued[11]];
    assert_eq!(u32::from_le_bytes(vid_bytes), target_vid);
}

#[test]
fn test_send_shoot_packet() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    let skill_vnum = 20u32;
    
    let result = SendShootPacket(skill_vnum);
    assert!(result, "SendShootPacket powinno zwrocic true");
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    
    // Rozmiar powinien zgadzac sie z TPacketCGShoot
    assert_eq!(queued.len(), std::mem::size_of::<TPacketCGShoot>());
    
    // Weryfikacja naglowka
    let header = u16::from_le_bytes([queued[0], queued[1]]);
    assert_eq!(header, TPacketCGShoot::HEADER);
    assert_eq!(header, 0x0403);
    
    // Weryfikacja length
    let length = u16::from_le_bytes([queued[2], queued[3]]);
    assert_eq!(length, std::mem::size_of::<TPacketCGShoot>() as u16);
    
    // Weryfikacja typu strzalu
    assert_eq!(queued[4], skill_vnum as u8);
}

#[test]
fn test_send_attack_packet_edge_cases() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    // Max values
    let hit_type = u8::MAX;
    let target_vid = u32::MAX;
    
    let result = SendAttackPacket(hit_type, target_vid);
    assert!(result);
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    assert_eq!(queued[4], hit_type);
    
    let vid_bytes = [queued[5], queued[6], queued[7], queued[8]];
    assert_eq!(u32::from_le_bytes(vid_bytes), target_vid);
}

#[test]
fn test_send_use_skill_packet_edge_cases() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    // Max values
    let skill_vnum = u32::MAX;
    let target_vid = u32::MAX;
    
    let result = SendUseSkillPacket(skill_vnum, target_vid);
    assert!(result);
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    
    let vnum_bytes = [queued[4], queued[5], queued[6], queued[7]];
    assert_eq!(u32::from_le_bytes(vnum_bytes), skill_vnum);
    
    let vid_bytes = [queued[8], queued[9], queued[10], queued[11]];
    assert_eq!(u32::from_le_bytes(vid_bytes), target_vid);
}

#[test]
fn test_send_shoot_packet_edge_cases() {
    let _guard = TEST_MUTEX.lock().unwrap();
    clear_outgoing_packets();
    
    // Max values
    let skill_vnum = u32::MAX;
    
    let result = SendShootPacket(skill_vnum);
    assert!(result);
    
    let queued = get_last_outgoing_packet().expect("Pakiet powinien byc w kolejce");
    // b_type in shoot packet is u8, so it truncates u32::MAX to u8::MAX (255)
    assert_eq!(queued[4], skill_vnum as u8);
}
