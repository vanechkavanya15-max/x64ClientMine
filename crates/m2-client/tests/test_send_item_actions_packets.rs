#[path = "../src/send_item_actions_packets.rs"]
mod send_item_actions_packets;

use send_item_actions_packets::{
    TPacketCGItemDrop, TPacketCGItemDrop2, TPacketCGItemMove, TPacketCGItemPickUp,
    TPacketCGItemUseToItem, TPacketCGGiveItem, TItemPos,
};

#[test]
fn test_titempos_size() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
}

#[test]
fn test_packet_item_drop() {
    assert_eq!(std::mem::size_of::<TPacketCGItemDrop>(), 11);

    let pkt = TPacketCGItemDrop::new(1, 10, 500);
    assert_eq!({pkt.header}, 0x0503);
    assert_eq!({pkt.length}, 11);
    assert_eq!(pkt.pos.window_type, 1);
    assert_eq!({pkt.pos.cell}, 10);
    assert_eq!({pkt.elk}, 500);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 11);
    assert_eq!(&bytes[0..2], &0x0503_u16.to_le_bytes()); // header
    assert_eq!(&bytes[2..4], &11_u16.to_le_bytes());     // length
    assert_eq!(bytes[4], 1);                             // window_type
    assert_eq!(&bytes[5..7], &10_u16.to_le_bytes());     // cell
    assert_eq!(&bytes[7..11], &500_u32.to_le_bytes());   // elk
}

#[test]
fn test_packet_item_drop_new() {
    assert_eq!(std::mem::size_of::<TPacketCGItemDrop2>(), 12);

    let pkt = TPacketCGItemDrop2::new(2, 20, 1000, 5);
    assert_eq!({pkt.header}, 0x050A);
    assert_eq!({pkt.length}, 12);
    assert_eq!(pkt.pos.window_type, 2);
    assert_eq!({pkt.pos.cell}, 20);
    assert_eq!({pkt.gold}, 1000);
    assert_eq!(pkt.count, 5);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 12);
    assert_eq!(&bytes[0..2], &0x050A_u16.to_le_bytes());
    assert_eq!(&bytes[2..4], &12_u16.to_le_bytes());
    assert_eq!(bytes[4], 2);
    assert_eq!(&bytes[5..7], &20_u16.to_le_bytes());
    assert_eq!(&bytes[7..11], &1000_u32.to_le_bytes());
    assert_eq!(bytes[11], 5);
}

#[test]
fn test_packet_item_move() {
    assert_eq!(std::mem::size_of::<TPacketCGItemMove>(), 11);

    let pkt = TPacketCGItemMove::new(1, 0, 1, 1, 10);
    assert_eq!({pkt.header}, 0x0504);
    assert_eq!({pkt.length}, 11);
    assert_eq!(pkt.pos.window_type, 1);
    assert_eq!({pkt.pos.cell}, 0);
    assert_eq!(pkt.change_pos.window_type, 1);
    assert_eq!({pkt.change_pos.cell}, 1);
    assert_eq!(pkt.num, 10);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 11);
    assert_eq!(&bytes[0..2], &0x0504_u16.to_le_bytes());
    assert_eq!(&bytes[2..4], &11_u16.to_le_bytes());
    assert_eq!(bytes[4], 1);
    assert_eq!(&bytes[5..7], &0_u16.to_le_bytes());
    assert_eq!(bytes[7], 1);
    assert_eq!(&bytes[8..10], &1_u16.to_le_bytes());
    assert_eq!(bytes[10], 10);
}

#[test]
fn test_packet_item_pickup() {
    assert_eq!(std::mem::size_of::<TPacketCGItemPickUp>(), 8);

    let pkt = TPacketCGItemPickUp::new(9999);
    assert_eq!({pkt.header}, 0x0502);
    assert_eq!({pkt.length}, 8);
    assert_eq!({pkt.vid}, 9999);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[0..2], &0x0502_u16.to_le_bytes());
    assert_eq!(&bytes[2..4], &8_u16.to_le_bytes());
    assert_eq!(&bytes[4..8], &9999_u32.to_le_bytes());
}

#[test]
fn test_packet_item_use_to_item() {
    assert_eq!(std::mem::size_of::<TPacketCGItemUseToItem>(), 10);

    let pkt = TPacketCGItemUseToItem::new(1, 5, 2, 10);
    assert_eq!({pkt.header}, 0x050B);
    assert_eq!({pkt.length}, 10);
    assert_eq!(pkt.source_pos.window_type, 1);
    assert_eq!({pkt.source_pos.cell}, 5);
    assert_eq!(pkt.target_pos.window_type, 2);
    assert_eq!({pkt.target_pos.cell}, 10);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 10);
    assert_eq!(&bytes[0..2], &0x050B_u16.to_le_bytes());
    assert_eq!(&bytes[2..4], &10_u16.to_le_bytes());
    assert_eq!(bytes[4], 1);
    assert_eq!(&bytes[5..7], &5_u16.to_le_bytes());
    assert_eq!(bytes[7], 2);
    assert_eq!(&bytes[8..10], &10_u16.to_le_bytes());
}

#[test]
fn test_packet_give_item() {
    assert_eq!(std::mem::size_of::<TPacketCGGiveItem>(), 12);

    let pkt = TPacketCGGiveItem::new(12345, 1, 90, 20);
    assert_eq!({pkt.header}, 0x0507);
    assert_eq!({pkt.length}, 12);
    assert_eq!({pkt.dwTargetVID}, 12345);
    assert_eq!(pkt.ItemPos.window_type, 1);
    assert_eq!({pkt.ItemPos.cell}, 90);
    assert_eq!(pkt.byItemCount, 20);

    let bytes = pkt.to_bytes();
    assert_eq!(bytes.len(), 12);
    assert_eq!(&bytes[0..2], &0x0507_u16.to_le_bytes());
    assert_eq!(&bytes[2..4], &12_u16.to_le_bytes());
    assert_eq!(&bytes[4..8], &12345_u32.to_le_bytes());
    assert_eq!(bytes[8], 1);
    assert_eq!(&bytes[9..11], &90_u16.to_le_bytes());
    assert_eq!(bytes[11], 20);
}
