use m2_client_bridge::net::cg::use_skill::{CgUseSkill, HEADER_CG_USE_SKILL};

#[test]
fn test_cg_use_skill_constants() {
    // Sprawdzamy czy stala naglowka pakietu jest poprawna.
    assert_eq!(HEADER_CG_USE_SKILL, 52);
}

#[test]
fn test_cg_use_skill_creation() {
    let packet = CgUseSkill::new(12345, 67890);
    assert_eq!(packet.header, 52);
    assert_eq!(packet.vnum, 12345);
    assert_eq!(packet.target_vid, 67890);
}

#[test]
fn test_cg_use_skill_to_bytes() {
    let vnum: u32 = 0x12345678;
    let target_vid: u32 = 0x9ABCDEF0;
    
    let packet = CgUseSkill::new(vnum, target_vid);
    let bytes = packet.to_bytes();
    
    // Oczekiwana dlugosc pakietu: 1 (header) + 4 (vnum) + 4 (target_vid) = 9 bajtow.
    assert_eq!(bytes.len(), 9);
    
    // Naglowek
    assert_eq!(bytes[0], 52);
    
    // vnum (Little-Endian)
    assert_eq!(bytes[1], 0x78);
    assert_eq!(bytes[2], 0x56);
    assert_eq!(bytes[3], 0x34);
    assert_eq!(bytes[4], 0x12);
    
    // target_vid (Little-Endian)
    assert_eq!(bytes[5], 0xF0);
    assert_eq!(bytes[6], 0xDE);
    assert_eq!(bytes[7], 0xBC);
    assert_eq!(bytes[8], 0x9A);
}

#[test]
fn test_cg_use_skill_to_bytes_zero_values() {
    let packet = CgUseSkill::new(0, 0);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 9);
    assert_eq!(bytes[0], 52);
    
    // vnum = 0
    assert_eq!(bytes[1], 0);
    assert_eq!(bytes[2], 0);
    assert_eq!(bytes[3], 0);
    assert_eq!(bytes[4], 0);
    
    // target_vid = 0
    assert_eq!(bytes[5], 0);
    assert_eq!(bytes[6], 0);
    assert_eq!(bytes[7], 0);
    assert_eq!(bytes[8], 0);
}
