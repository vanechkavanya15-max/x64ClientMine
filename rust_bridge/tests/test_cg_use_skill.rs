use m2_client_bridge::net::cg::use_skill::{HEADER_CG_USE_SKILL, TPacketCGUseSkill};
use std::mem::size_of;

#[test]
fn test_cg_use_skill_creation_and_size() {
    let packet = TPacketCGUseSkill::new(1, 2);
    
    // header(2) + length(2) + vnum(4) + vid(4) = 12 bajtow
    assert_eq!(size_of::<TPacketCGUseSkill>(), 12);
    
    let header = { packet.header };
    let length = { packet.length };
    let vnum = { packet.dw_vnum };
    let vid = { packet.dw_target_vid };

    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(length, 12);
    assert_eq!(vnum, 1);
    assert_eq!(vid, 2);
}

#[test]
fn test_cg_use_skill_to_bytes() {
    let packet = TPacketCGUseSkill::new(0x01020304, 0x0A0B0C0D);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 12);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_USE_SKILL);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 12);
    
    // Test wartosci w formacie Little-Endian dla vnum
    assert_eq!(bytes[4], 0x04);
    assert_eq!(bytes[5], 0x03);
    assert_eq!(bytes[6], 0x02);
    assert_eq!(bytes[7], 0x01);
    
    // Test wartosci w formacie Little-Endian dla vid
    assert_eq!(bytes[8], 0x0D);
    assert_eq!(bytes[9], 0x0C);
    assert_eq!(bytes[10], 0x0B);
    assert_eq!(bytes[11], 0x0A);
}
