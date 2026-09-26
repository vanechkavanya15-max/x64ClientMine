use std::mem;

use m2_client::recv_exchange_logic_v2::*;

#[test]
fn test_exchange_packet_layout() {
    assert_eq!(mem::size_of::<TItemPos>(), 3);
    assert_eq!(mem::size_of::<TPlayerItemAttribute>(), 3);
    // header(1) + length(2) + subheader(1) + is_me(1) + arg1(4) + arg2(3) + arg3(4) + alValues(12) + aAttr(21)
    // 1 + 2 + 1 + 1 + 4 + 3 + 4 + 12 + 21 = 49
    assert_eq!(mem::size_of::<TPacketGCExchange>(), 49);
}

#[test]
fn test_exchange_state_machine() {
    let mut logic = ExchangeLogic::new();
    assert_eq!(logic.state, ExchangeState::Idle);

    let mut packet = TPacketGCExchange {
        header: 0,
        length: 0,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: false,
        arg1: 0,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute { b_type: 0, s_value: 0 }; 7],
    };

    assert!(logic.handle_packet(&packet));
    assert_eq!(logic.state, ExchangeState::Open);

    packet.subheader = ExchangeSubHeader::ItemAdd as u8;
    assert!(logic.handle_packet(&packet));
    assert_eq!(logic.state, ExchangeState::Open);

    packet.subheader = ExchangeSubHeader::Accept as u8;
    assert!(logic.handle_packet(&packet));
    assert_eq!(logic.state, ExchangeState::Locked);

    packet.subheader = ExchangeSubHeader::End as u8;
    assert!(logic.handle_packet(&packet));
    assert_eq!(logic.state, ExchangeState::Completed);
}

#[test]
fn test_business_logic_item_add() {
    let mut logic = ExchangeLogic::new();
    
    // Start Exchange
    let mut packet = TPacketGCExchange {
        header: 0,
        length: 0,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: false,
        arg1: 1000,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };
    logic.handle_packet(&packet);
    
    // Add Item to Target
    packet.subheader = ExchangeSubHeader::ItemAdd as u8;
    packet.is_me = false;
    packet.arg1 = 19; // VNUM
    packet.arg2 = TItemPos { window_type: 0, cell: 2 }; // Cell 2
    packet.arg3 = 1; // Count
    packet.al_values = [100, 200, 300];
    packet.a_attr[0] = TPlayerItemAttribute { b_type: 1, s_value: 5 };
    
    logic.handle_packet(&packet);
    
    assert_eq!(logic.target_data.item_vnum[2], 19);
    assert_eq!(logic.target_data.item_count[2], 1);
    assert_eq!(logic.target_data.item_metin[2][0], 100);
    assert_eq!(logic.target_data.item_metin[2][1], 200);
    assert_eq!(logic.target_data.item_metin[2][2], 300);
    assert_eq!(logic.target_data.item_attr[2][0].b_type, 1);
    let val = logic.target_data.item_attr[2][0].s_value;
    assert_eq!(val, 5);
}

#[test]
fn test_business_logic_item_del() {
    let mut logic = ExchangeLogic::new();
    
    // Start Exchange
    let mut packet = TPacketGCExchange {
        header: 0,
        length: 0,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: false,
        arg1: 1000,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };
    logic.handle_packet(&packet);
    
    // Add Item to Self
    packet.subheader = ExchangeSubHeader::ItemAdd as u8;
    packet.is_me = true;
    packet.arg1 = 19; 
    packet.arg2 = TItemPos { window_type: 0, cell: 2 }; 
    packet.arg3 = 1; 
    logic.handle_packet(&packet);
    
    assert_eq!(logic.self_data.item_vnum[2], 19);
    
    // Delete Item
    packet.subheader = ExchangeSubHeader::ItemDel as u8;
    packet.is_me = true;
    packet.arg1 = 2; // Cell 2
    logic.handle_packet(&packet);
    
    assert_eq!(logic.self_data.item_vnum[2], 0);
    assert_eq!(logic.self_data.item_count[2], 0);
}

#[test]
fn test_business_logic_elk_add() {
    let mut logic = ExchangeLogic::new();
    
    // Start Exchange
    let mut packet = TPacketGCExchange {
        header: 0,
        length: 0,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: false,
        arg1: 1000,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };
    logic.handle_packet(&packet);
    
    // Add Elk to Self
    packet.subheader = ExchangeSubHeader::ElkAdd as u8;
    packet.is_me = true;
    packet.arg1 = 50000; 
    logic.handle_packet(&packet);
    
    assert_eq!(logic.self_data.elk, 50000);
}

#[test]
fn test_ffi_recv_exchange_start() {
    let mut logic = get_exchange_logic().write().unwrap();
    logic.state = ExchangeState::Idle;
    drop(logic);

    let packet = TPacketGCExchange {
        header: 0x2A,
        length: mem::size_of::<TPacketGCExchange>() as u16,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: false,
        arg1: 1234,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute { b_type: 0, s_value: 0 }; 7],
    };

    let buf = &packet as *const _ as *const u8;
    let len = mem::size_of::<TPacketGCExchange>();

    let res = m2_net_recv_exchange(buf, len);
    assert!(res);

    let logic = get_exchange_logic().read().unwrap();
    assert_eq!(logic.state, ExchangeState::Open);
    assert_eq!(logic.target_vid, 1234);
}

#[test]
fn test_ffi_invalid_buffer_size() {
    let buf: [u8; 10] = [0; 10];
    let res = m2_net_recv_exchange(buf.as_ptr(), buf.len());
    assert!(!res);
}

#[test]
fn test_ffi_null_pointer() {
    let res = m2_net_recv_exchange(std::ptr::null(), 50);
    assert!(!res);
}

#[test]
fn test_unknown_subheader() {
    let mut logic = ExchangeLogic::new();
    let packet = TPacketGCExchange {
        header: 0,
        length: 0,
        subheader: 99, // Unknown
        is_me: false,
        arg1: 0,
        arg2: TItemPos { window_type: 0, cell: 0 },
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute { b_type: 0, s_value: 0 }; 7],
    };

    let result = logic.handle_packet(&packet);
    assert!(result); // C++ returns true for unknown subheaders to continue parsing
    assert_eq!(logic.state, ExchangeState::Idle); // State shouldn't change
}
