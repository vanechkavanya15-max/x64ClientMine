use m2_client_bridge::net::cg::item_drop::{TItemPos, TPacketCGItemDrop};

#[test]
fn test_serialize_cg_item_drop() {
    // Tworzymy przykladowy pakiet HEADER_CG_ITEM_DROP
    let packet = TPacketCGItemDrop::new(1, 10, 500);

    // Serializujemy pakiet
    let bytes = packet.to_bytes();

    // Weryfikujemy dlugosc - zgodnie z `#pragma pack(1)` i definicjami w C++
    // header(1) + window_type(1) + cell(2) + elk(4) = 8 bajtow
    assert_eq!(bytes.len(), 8, "Rozmiar pakietu powinien wynosic 8 bajtow");

    // Weryfikujemy poszczegolne bajty (Little-Endian)
    // [0] header: 12
    assert_eq!(bytes[0], 12, "Niepoprawny naglowek");

    // [1] window_type: 1
    assert_eq!(bytes[1], 1, "Niepoprawny window_type");

    // [2..3] cell: 10 (0x0A 0x00)
    assert_eq!(bytes[2], 0x0A, "Niepoprawny cell bajt 1");
    assert_eq!(bytes[3], 0x00, "Niepoprawny cell bajt 2");

    // [4..7] count: 500 (0xF4 0x01 0x00 0x00)
    assert_eq!(bytes[4], 0xF4, "Niepoprawny count bajt 1");
    assert_eq!(bytes[5], 0x01, "Niepoprawny count bajt 2");
    assert_eq!(bytes[6], 0x00, "Niepoprawny count bajt 3");
    assert_eq!(bytes[7], 0x00, "Niepoprawny count bajt 4");
}

#[test]
fn test_serialize_cg_item_drop_count_1() {
    let packet = TPacketCGItemDrop::new(2, 5, 1);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes[0], 12);
    assert_eq!(bytes[1], 2);
    assert_eq!(bytes[2], 5);
    assert_eq!(bytes[3], 0);
    
    // count = 1 (0x01 0x00 0x00 0x00)
    assert_eq!(bytes[4], 0x01);
    assert_eq!(bytes[5], 0x00);
    assert_eq!(bytes[6], 0x00);
    assert_eq!(bytes[7], 0x00);
}

#[test]
fn test_serialize_cg_item_drop_count_2_000_000_000() {
    let packet = TPacketCGItemDrop::new(3, 100, 2_000_000_000);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes[0], 12);
    assert_eq!(bytes[1], 3);
    assert_eq!(bytes[2], 100);
    assert_eq!(bytes[3], 0);
    
    // count = 2_000_000_000 (0x00 0x94 0x35 0x77)
    assert_eq!(bytes[4], 0x00);
    assert_eq!(bytes[5], 0x94);
    assert_eq!(bytes[6], 0x35);
    assert_eq!(bytes[7], 0x77);
}
