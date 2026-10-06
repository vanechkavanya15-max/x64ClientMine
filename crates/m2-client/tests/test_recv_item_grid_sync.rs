#[path = "../src/recv_item_grid_sync.rs"]
mod recv_item_grid_sync;

use recv_item_grid_sync::*;

struct MockPlayer {
    item_data_calls: Vec<(TItemPos, TItemData)>,
    item_count_calls: Vec<(TItemPos, u8)>,
    item_metin_socket_calls: Vec<(TItemPos, usize, i32)>,
    item_attribute_calls: Vec<(TItemPos, usize, u8, i16)>,
}

impl MockPlayer {
    fn new() -> Self {
        Self {
            item_data_calls: Vec::new(),
            item_count_calls: Vec::new(),
            item_metin_socket_calls: Vec::new(),
            item_attribute_calls: Vec::new(),
        }
    }
}

impl AbstractPlayer for MockPlayer {
    fn set_item_data(&mut self, item_pos: TItemPos, item_data: TItemData) {
        self.item_data_calls.push((item_pos, item_data));
    }

    fn set_item_count(&mut self, item_pos: TItemPos, count: u8) {
        self.item_count_calls.push((item_pos, count));
    }

    fn set_item_metin_socket(&mut self, item_pos: TItemPos, socket_index: usize, metin_number: i32) {
        self.item_metin_socket_calls.push((item_pos, socket_index, metin_number));
    }

    fn set_item_attribute(&mut self, item_pos: TItemPos, attr_index: usize, attr_type: u8, attr_value: i16) {
        self.item_attribute_calls.push((item_pos, attr_index, attr_type, attr_value));
    }
}

#[test]
fn test_recv_item_set_packet() {
    let mut player = MockPlayer::new();
    let packet = TPacketGCItemSet {
        header: 1,
        pos: TItemPos { window_type: 1, cell: 2 },
        vnum: 1234,
        count: 5,
        flags: 10,
        anti_flags: 20,
        highlight: 1,
        alSockets: [100, 200, 300],
        aAttr: [
            TPlayerItemAttribute { bType: 1, sValue: 10 },
            TPlayerItemAttribute { bType: 2, sValue: 20 },
            TPlayerItemAttribute { bType: 3, sValue: 30 },
            TPlayerItemAttribute { bType: 4, sValue: 40 },
            TPlayerItemAttribute { bType: 5, sValue: 50 },
            TPlayerItemAttribute { bType: 6, sValue: 60 },
            TPlayerItemAttribute { bType: 7, sValue: 70 },
        ],
    };

    let result = recv_item_set_packet(&packet, &mut player);
    assert!(result.is_ok());
    assert_eq!(player.item_data_calls.len(), 1);
    
    let (pos, data) = &player.item_data_calls[0];
    assert_eq!(pos.window_type, 1);
    assert_eq!({let c = pos.cell; c}, 2);
    assert_eq!({let c = data.vnum; c}, 1234);
    assert_eq!(data.count, 5);
    assert_eq!({let c = data.flags; c}, 10);
    assert_eq!({let c = data.anti_flags; c}, 20);
    assert_eq!({let c = data.alSockets; c}, [100, 200, 300]);
    for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
        assert_eq!(data.aAttr[i].bType, (i + 1) as u8);
        assert_eq!({let c = data.aAttr[i].sValue; c}, ((i + 1) * 10) as i16);
    }
}

#[test]
fn test_recv_item_del_packet() {
    let mut player = MockPlayer::new();
    let packet = TPacketGCItemDel {
        header: 2,
        pos: TItemPos { window_type: 2, cell: 5 },
    };

    let result = recv_item_del_packet(&packet, &mut player);
    assert!(result.is_ok());
    assert_eq!(player.item_data_calls.len(), 1);
    
    let (pos, data) = &player.item_data_calls[0];
    assert_eq!(pos.window_type, 2);
    assert_eq!({let c = pos.cell; c}, 5);
    assert_eq!({let c = data.vnum; c}, 0);
    assert_eq!(data.count, 0);
    assert_eq!({let c = data.flags; c}, 0);
    assert_eq!({let c = data.anti_flags; c}, 0);
    assert_eq!({let c = data.alSockets; c}, [0, 0, 0]);
    for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
        assert_eq!(data.aAttr[i].bType, 0);
        assert_eq!({let c = data.aAttr[i].sValue; c}, 0);
    }
}

#[test]
fn test_recv_item_use_packet() {
    let mut player = MockPlayer::new();
    let packet = TPacketGCItemUse {
        header: 3,
        Cell: TItemPos { window_type: 1, cell: 10 },
        ch_vid: 100,
        victim_vid: 200,
        vnum: 555,
    };

    let result = recv_item_use_packet(&packet, &mut player);
    assert!(result.is_ok());
}

#[test]
fn test_recv_item_update_packet() {
    let mut player = MockPlayer::new();
    let packet = TPacketGCItemUpdate {
        header: 4,
        Cell: TItemPos { window_type: 1, cell: 20 },
        count: 99,
        alSockets: [11, 22, 33],
        aAttr: [
            TPlayerItemAttribute { bType: 1, sValue: 111 },
            TPlayerItemAttribute { bType: 2, sValue: 222 },
            TPlayerItemAttribute { bType: 3, sValue: 333 },
            TPlayerItemAttribute { bType: 4, sValue: 444 },
            TPlayerItemAttribute { bType: 5, sValue: 555 },
            TPlayerItemAttribute { bType: 6, sValue: 666 },
            TPlayerItemAttribute { bType: 7, sValue: 777 },
        ],
    };

    let result = recv_item_update_packet(&packet, &mut player);
    assert!(result.is_ok());
    
    assert_eq!(player.item_count_calls.len(), 1);
    assert_eq!(player.item_count_calls[0].0.window_type, 1);
    assert_eq!({let c = player.item_count_calls[0].0.cell; c}, 20);
    assert_eq!(player.item_count_calls[0].1, 99);
    
    assert_eq!(player.item_metin_socket_calls.len(), 3);
    for i in 0..3 {
        assert_eq!({let c = player.item_metin_socket_calls[i].0.cell; c}, 20);
        assert_eq!(player.item_metin_socket_calls[i].1, i);
        assert_eq!(player.item_metin_socket_calls[i].2, (i + 1) as i32 * 11);
    }
    
    assert_eq!(player.item_attribute_calls.len(), 7);
    for i in 0..7 {
        assert_eq!({let c = player.item_attribute_calls[i].0.cell; c}, 20);
        assert_eq!(player.item_attribute_calls[i].1, i);
        assert_eq!(player.item_attribute_calls[i].2, (i + 1) as u8);
        assert_eq!(player.item_attribute_calls[i].3, ((i + 1) * 111) as i16);
    }
}

#[test]
fn test_struct_packing() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
    assert_eq!(std::mem::size_of::<TPlayerItemAttribute>(), 3);
    assert_eq!(std::mem::size_of::<TItemData>(), 4 + 1 + 4 + 4 + (3 * 4) + (7 * 3));
    assert_eq!(std::mem::size_of::<TPacketGCItemSet>(), 1 + 3 + 4 + 1 + 4 + 4 + 1 + (3 * 4) + (7 * 3));
    assert_eq!(std::mem::size_of::<TPacketGCItemDel>(), 1 + 3);
    assert_eq!(std::mem::size_of::<TPacketGCItemUse>(), 1 + 3 + 4 + 4 + 4);
    assert_eq!(std::mem::size_of::<TPacketGCItemUpdate>(), 1 + 3 + 1 + (3 * 4) + (7 * 3));
}
