use std::mem;
use m2_client::recv_item_trade_complete::*;

#[test]
fn test_packet_sizes() {
    // Assert sizes based on C++ packed structs + fields
    assert_eq!(mem::size_of::<TItemPos>(), 3);
    assert_eq!(mem::size_of::<TPlayerItemAttribute>(), 3);
    
    // TShopItemData / packet_shop_item
    assert_eq!(mem::size_of::<packet_shop_item>(), 4 + 4 + 1 + 1 + (4 * 3) + (3 * 7)); // 43

    assert_eq!(mem::size_of::<TMaterial>(), 8);
    // TRefineTable: 4 + 4 + 1 + 4 + 4 + (8 * 5) = 57
    assert_eq!(mem::size_of::<TRefineTable>(), 57);
}

#[test]
fn test_item_ground_add_del() {
    let mut add_packet = TPacketGCItemGroundAdd {
        header: 0,
        length: mem::size_of::<TPacketGCItemGroundAdd>() as u16,
        l_x: 100,
        l_y: 200,
        l_z: 300,
        dw_vid: 1234,
        dw_vnum: 5678,
    };
    
    // Test Add
    let buf = &add_packet as *const _ as *const u8;
    assert!(m2_net_recv_item_ground_add(buf, mem::size_of::<TPacketGCItemGroundAdd>()));
    
    {
        let state = get_item_ground_state().read().unwrap();
        let item = state.items.get(&1234).unwrap();
        assert_eq!(item.vnum, 5678);
        assert_eq!(item.x, 100);
        assert_eq!(item.y, 200);
        assert_eq!(item.z, 300);
    }
    
    // Test Del
    let del_packet = TPacketGCItemGroundDel {
        header: 0,
        length: mem::size_of::<TPacketGCItemGroundDel>() as u16,
        vid: 1234,
    };
    let buf_del = &del_packet as *const _ as *const u8;
    assert!(m2_net_recv_item_ground_del(buf_del, mem::size_of::<TPacketGCItemGroundDel>()));
    
    {
        let state = get_item_ground_state().read().unwrap();
        assert!(state.items.get(&1234).is_none());
    }
}

#[test]
fn test_shop_start() {
    let shop_start = TPacketGCShopStart {
        items: [packet_shop_item::default(); SHOP_HOST_ITEM_MAX_NUM],
    };
    let mut payload = Vec::new();
    let shop_header = TPacketGCShop {
        header: 0,
        length: (mem::size_of::<TPacketGCShop>() + 4 + mem::size_of::<TPacketGCShopStart>()) as u16,
        subheader: 0, // START
    };
    
    // Build payload
    payload.extend_from_slice(unsafe { std::slice::from_raw_parts(&shop_header as *const _ as *const u8, mem::size_of::<TPacketGCShop>()) });
    payload.extend_from_slice(&999u32.to_ne_bytes()); // owner_vid
    payload.extend_from_slice(unsafe { std::slice::from_raw_parts(&shop_start as *const _ as *const u8, mem::size_of::<TPacketGCShopStart>()) });
    
    assert!(m2_net_recv_shop(payload.as_ptr(), payload.len()));
    
    {
        let state = get_shop_state().read().unwrap();
        assert_eq!(state.owner_vid, 999);
    }
}

#[test]
fn test_refine_info() {
    let mut materials = [TMaterial { vnum: 0, count: 0 }; REFINE_MATERIAL_MAX_NUM];
    materials[0] = TMaterial { vnum: 123, count: 2 };
    
    let refine_packet = TPacketGCRefineInformation {
        header: 0,
        length: mem::size_of::<TPacketGCRefineInformation>() as u16,
        type_: 1,
        pos: 5,
        refine_table: TRefineTable {
            src_vnum: 10,
            result_vnum: 11,
            material_count: 1,
            cost: 1000,
            prob: 80,
            materials,
        },
    };
    
    let buf = &refine_packet as *const _ as *const u8;
    assert!(m2_net_recv_refine_info(buf, mem::size_of::<TPacketGCRefineInformation>()));
    
    {
        let state = get_refine_state().read().unwrap();
        assert_eq!(state.type_, 1);
        assert_eq!(state.pos, 5);
        let info = state.info.as_ref().unwrap();
        assert_eq!(info.src_vnum, 10);
        assert_eq!(info.result_vnum, 11);
        assert_eq!(info.cost, 1000);
        assert_eq!(info.prob, 80);
        assert_eq!(info.materials[0].vnum, 123);
        assert_eq!(info.materials[0].count, 2);
    }
}

#[test]
fn test_item_update() {
    let update_packet = TPacketGCItemUpdate {
        header: 0,
        length: mem::size_of::<TPacketGCItemUpdate>() as u16,
        cell: TItemPos { window_type: 1, cell: 2 },
        count: 5,
        al_sockets: [1, 2, 3],
        a_attr: [TPlayerItemAttribute { b_type: 1, s_value: 2 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    };
    
    let buf = &update_packet as *const _ as *const u8;
    assert!(m2_net_recv_item_update(buf, mem::size_of::<TPacketGCItemUpdate>()));
}
