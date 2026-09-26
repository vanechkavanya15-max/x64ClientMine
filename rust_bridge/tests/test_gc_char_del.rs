use m2_client_bridge::net::gc::char_del::{
    PacketGcCharacterDelete, TPacketGCCharacterDelete, HEADER_GC_CHARACTER_DEL,
};

#[test]
fn test_char_del_valid_buffer() {
    let mut buf = [0u8; 8];
    buf[0..2].copy_from_slice(&HEADER_GC_CHARACTER_DEL.to_le_bytes());
    buf[2..4].copy_from_slice(&8u16.to_le_bytes());
    buf[4..8].copy_from_slice(&0x12345678u32.to_le_bytes());
    
    let result = TPacketGCCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_ok(), "Powinno byc ok, bufor ma 8 bajtow");
    
    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_CHARACTER_DEL);
    assert_eq!({ packet.length }, 8);
    assert_eq!(packet.vid(), 0x12345678, "VID odczytany niepoprawnie");
}

#[test]
fn test_char_del_buffer_too_short() {
    let buf: [u8; 7] = [0; 7];
    
    let result = PacketGcCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_err(), "Powinno byc err, bufor jest za krotki");
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki", "Komunikat bledu nie zgadza sie");
}

#[test]
fn test_char_del_buffer_longer_than_needed() {
    let mut buf = [0u8; 12];
    buf[0..2].copy_from_slice(&HEADER_GC_CHARACTER_DEL.to_le_bytes());
    buf[2..4].copy_from_slice(&8u16.to_le_bytes());
    buf[4..8].copy_from_slice(&0x12345678u32.to_le_bytes());
    buf[8..12].copy_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    
    let result = PacketGcCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_ok(), "Dalsze bajty w buforze nie powinny przeszkadzac");
    
    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_CHARACTER_DEL);
    assert_eq!(packet.vid(), 0x12345678);
}
