use m2_client_bridge::net::cg::item_use::{PacketCgItemUse, ItemPos};

#[test]
fn test_cg_item_use_serialization() {
    // Przypadek 1: Zwykle wywolanie
    let packet = PacketCgItemUse {
        header: PacketCgItemUse::HEADER,
        pos: ItemPos {
            window_type: 1, // EQUIPMENT = 1
            cell: 12, // jakis przykladowy slot
        },
    };
    
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 4, "Pakiet musi miec dokladnie 4 bajty dlugosci (1 header, 1 window_type, 2 cell)");
    assert_eq!(bytes[0], 11, "Header musi byc rowny 11 (HEADER_CG_ITEM_USE)");
    assert_eq!(bytes[1], 1, "window_type musi byc 1");
    // cell = 12 (0x000C). W little-endian: 0x0C 0x00
    assert_eq!(bytes[2], 12, "Dolny bajt cell ma wynosic 12");
    assert_eq!(bytes[3], 0, "Gorny bajt cell ma wynosic 0");
    
    // Przypadek 2: Skrajne wartosci
    let packet_edge = PacketCgItemUse {
        header: PacketCgItemUse::HEADER,
        pos: ItemPos {
            window_type: 255, // max u8
            cell: 0xFFFF, // WORD_MAX
        },
    };
    
    let bytes_edge = packet_edge.to_bytes();
    assert_eq!(bytes_edge.len(), 4);
    assert_eq!(bytes_edge[0], 11);
    assert_eq!(bytes_edge[1], 255);
    // cell = 0xFFFF w little-endian to 0xFF 0xFF
    assert_eq!(bytes_edge[2], 255);
    assert_eq!(bytes_edge[3], 255);
}
