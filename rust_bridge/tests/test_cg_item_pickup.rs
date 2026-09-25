use m2_client_bridge::net::cg::item_pickup::TPacketCGItemPickup;
use std::mem::size_of;

#[test]
fn test_cg_item_pickup_serialization() {
    // Sprawdzanie rozmiaru calej struktury
    assert_eq!(size_of::<TPacketCGItemPickup>(), 5);

    // Tworzenie przykladowego pakietu z vid = 0x12345678
    let packet = TPacketCGItemPickup {
        header: 15,
        dw_vid: 0x12345678,
    };

    let bytes = packet.to_bytes();

    // Sprawdzanie rozmiaru wyjsciowej tablicy (1 bajt header + 4 bajty vid = 5 bajtow)
    assert_eq!(bytes.len(), 5);

    // Sprawdzanie poprawnosci naglowka
    assert_eq!(bytes[0], 15);

    // Sprawdzanie poprawnosci zapisu dw_vid w formacie Little-Endian
    assert_eq!(bytes[1], 0x78);
    assert_eq!(bytes[2], 0x56);
    assert_eq!(bytes[3], 0x34);
    assert_eq!(bytes[4], 0x12);
}

#[test]
fn test_cg_item_pickup_new() {
    let packet = TPacketCGItemPickup::new(0xAABBCCDD);
    
    assert_eq!(packet.header, 15);
    
    let vid = { packet.dw_vid };
    assert_eq!(vid, 0xAABBCCDD);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes, [15, 0xDD, 0xCC, 0xBB, 0xAA]);
}
