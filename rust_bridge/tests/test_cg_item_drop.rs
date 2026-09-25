use m2_client_bridge::net::cg::item_drop::{ItemPos, PacketCgItemDrop};

#[test]
fn test_serialize_cg_item_drop() {
    // Tworzymy przykladowy pakiet HEADER_CG_ITEM_DROP
    let packet = PacketCgItemDrop {
        header: PacketCgItemDrop::HEADER,
        pos: ItemPos {
            window_type: 1, // EQUIPMENT
            cell: 10,
        },
        elk: 500,
    };

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

    // [4..7] elk: 500 (0xF4 0x01 0x00 0x00)
    assert_eq!(bytes[4], 0xF4, "Niepoprawny elk bajt 1");
    assert_eq!(bytes[5], 0x01, "Niepoprawny elk bajt 2");
    assert_eq!(bytes[6], 0x00, "Niepoprawny elk bajt 3");
    assert_eq!(bytes[7], 0x00, "Niepoprawny elk bajt 4");
}
