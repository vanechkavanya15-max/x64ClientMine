use m2_client_bridge::net::gc::char_del::PacketGcCharacterDelete;

#[test]
fn test_char_del_valid_buffer() {
    // HEADER_GC_CHARACTER_DEL = 2, w kodzie zrodlowym (zwykle, to przykladowy header uzywany dla pewnosci)
    // 0x02, a nastepnie VID = 0x12345678, ktory jako little endian to 0x78, 0x56, 0x34, 0x12
    let buf: [u8; 5] = [0x02, 0x78, 0x56, 0x34, 0x12];
    
    let result = PacketGcCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_ok(), "Powinno byc ok, bufor ma 5 bajtow");
    
    let packet = result.unwrap();
    assert_eq!(packet.header, 0x02, "Header powinnien wynosic 2");
    assert_eq!(packet.vid, 0x12345678, "VID odczytany niepoprawnie");
}

#[test]
fn test_char_del_buffer_too_short() {
    // Krotki bufor - tylko 4 bajty
    let buf: [u8; 4] = [0x02, 0x78, 0x56, 0x34];
    
    let result = PacketGcCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_err(), "Powinno byc err, bufor jest za krotki");
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki", "Komunikat bledu nie zgadza sie");
}

#[test]
fn test_char_del_buffer_longer_than_needed() {
    // Bufor ma wiecej niz 5 bajtow, metoda nadal powinna przeczytac pierwsze 5
    let buf: [u8; 8] = [0x02, 0x78, 0x56, 0x34, 0x12, 0x99, 0xaa, 0xbb];
    
    let result = PacketGcCharacterDelete::from_bytes(&buf);
    
    assert!(result.is_ok(), "Dalsze bajty w buforze nie powinny przeszkadzac");
    
    let packet = result.unwrap();
    assert_eq!(packet.header, 0x02);
    assert_eq!(packet.vid, 0x12345678);
}
