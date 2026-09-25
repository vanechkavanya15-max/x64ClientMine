use m2_client_bridge::net::cg::item_pickup::CGItemPickup;

#[test]
fn test_cg_item_pickup_serialization() {
    // Tworzenie przykladowego pakietu z vid = 0x12345678
    let packet = CGItemPickup {
        header: 15,
        vid: 0x12345678,
    };

    let bytes = packet.to_bytes();

    // Sprawdzanie rozmiaru pakietu (1 bajt header + 4 bajty vid = 5 bajtow)
    assert_eq!(bytes.len(), 5);

    // Sprawdzanie poprawnosci naglowka
    assert_eq!(bytes[0], 15);

    // Sprawdzanie poprawnosci zapisu vid w formacie Little-Endian
    assert_eq!(bytes[1], 0x78);
    assert_eq!(bytes[2], 0x56);
    assert_eq!(bytes[3], 0x34);
    assert_eq!(bytes[4], 0x12);
}

#[test]
fn test_cg_item_pickup_new() {
    let packet = CGItemPickup::new(0xAABBCCDD);
    
    assert_eq!(packet.header, 15);
    assert_eq!(packet.vid, 0xAABBCCDD);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes, vec![15, 0xDD, 0xCC, 0xBB, 0xAA]);
}
