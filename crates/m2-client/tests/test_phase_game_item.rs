use m2_client::phase_game_item::*;

#[test]
fn test_integration_phase_game_item() {
    let mut instance = PhaseGameItem::new();
    assert!(instance.is_ready());

    // Test Ground Add
    let ground_add = TPacketGCItemGroundAdd {
        header: 1,
        lx: 100,
        ly: 200,
        lz: 300,
        dwVID: 10,
        dwVnum: 5001,
    };
    assert!(instance.recv_item_ground_add_packet(&ground_add));
    assert_eq!(instance.ground_items.len(), 1);
    
    // Safety check with unaligned ptrs for packed struct
    let added_item = instance.ground_items.get(&10).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(added_item.dwVnum).read_unaligned() }, 5001);

    // Test Ownership
    let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let test_name = b"TestPlayer";
    name[..test_name.len()].copy_from_slice(test_name);

    let ownership = TPacketGCItemOwnership {
        header: 2,
        dwVID: 10,
        szName: name,
    };
    assert!(instance.recv_item_ownership(&ownership));
    assert_eq!(instance.ownerships.get(&10).unwrap(), "TestPlayer");

    // Test Ground Del
    let ground_del = TPacketGCItemGroundDel {
        header: 3,
        vid: 10,
    };
    assert!(instance.recv_item_ground_del_packet(&ground_del));
    assert_eq!(instance.ground_items.len(), 0);

    // Test Item Set
    let cell = TItemPos { window_type: 1, cell: 42 };
    let item_set = TPacketGCItemSet {
        header: 4,
        cell,
        vnum: 999,
        count: 5,
        alSockets: [1, 2, 3],
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
    assert!(instance.recv_item_set_packet(&item_set));
    assert_eq!(instance.items.len(), 1);
    let item = instance.items.get(&42).unwrap();
    assert_eq!(item.vnum, 999);
    assert_eq!(item.count, 5);
    assert_eq!(item.flags, 0);

    // Test Item Set 2
    let cell2 = TItemPos { window_type: 1, cell: 43 };
    let item_set2 = TPacketGCItemSet2 {
        header: 5,
        cell: cell2,
        vnum: 1000,
        count: 1,
        flags: 123,
        anti_flags: 456,
        highlight: true,
        alSockets: [4, 5, 6],
        aAttr: [TPlayerItemAttribute { bType: 0, sValue: 0 }; 7],
    };
    assert!(instance.recv_item_set_packet2(&item_set2));
    assert_eq!(instance.items.len(), 2);
    let item2 = instance.items.get(&43).unwrap();
    assert_eq!(item2.vnum, 1000);
    assert_eq!(item2.flags, 123);
    assert_eq!(item2.anti_flags, 456);

    // Test Item Use
    let item_use = TPacketGCItemUse {
        header: 6,
        cell,
        ch_vid: 1,
        victim_vid: 2,
        vnum: 999,
    };
    assert!(instance.recv_item_use_packet(&item_use));
}
