use m2_client_bridge::net::cg::use_skill::{HEADER_CG_USE_SKILL, TPacketCGUseSkill};
use std::mem::size_of;

#[test]
fn test_cg_use_skill_creation_and_size() {
    let packet = TPacketCGUseSkill::new(1, 2);
    
    // Zgodnie ze specyfikacja: 1 bajt (header) + 4 bajty (vnum) + 4 bajty (vid) = 9 bajtow
    assert_eq!(size_of::<TPacketCGUseSkill>(), 9);
    
    // Kopiujemy wartosci poniewaz struktura jest packed
    let header = packet.header;
    let vnum = packet.dw_vnum;
    let vid = packet.dw_vid;

    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(vnum, 1);
    assert_eq!(vid, 2);
}

#[test]
fn test_cg_use_skill_to_bytes() {
    // 0x01020304 = 16909060
    // 0x0A0B0C0D = 168496141
    let packet = TPacketCGUseSkill::new(0x01020304, 0x0A0B0C0D);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 9);
    assert_eq!(bytes[0], HEADER_CG_USE_SKILL);
    
    // Test wartosci w formacie Little-Endian dla vnum
    assert_eq!(bytes[1], 0x04);
    assert_eq!(bytes[2], 0x03);
    assert_eq!(bytes[3], 0x02);
    assert_eq!(bytes[4], 0x01);
    
    // Test wartosci w formacie Little-Endian dla vid
    assert_eq!(bytes[5], 0x0D);
    assert_eq!(bytes[6], 0x0C);
    assert_eq!(bytes[7], 0x0B);
    assert_eq!(bytes[8], 0x0A);
}
