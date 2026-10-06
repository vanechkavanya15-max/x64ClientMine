#[path = "../src/recv_exchange_protocol.rs"]
mod recv_exchange_protocol;

use recv_exchange_protocol::*;

#[test]
fn test_tpcketgcexchange_size() {
    assert_eq!(
        std::mem::size_of::<TPacketGCExchange>(),
        50,
        "TPacketGCExchange size must match exactly the C++ layout"
    );
}

#[test]
fn test_exchange_start() {
    let mut logic = ExchangeLogic::new();
    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: 0,
        arg1: 12345, // target_vid
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    assert_eq!(logic.state, ExchangeState::Idle);
    let handled = logic.handle_packet(&packet);
    assert!(handled);
    assert_eq!(logic.state, ExchangeState::Open);
    assert_eq!(logic.target_vid, 12345);
}

#[test]
fn test_exchange_item_add() {
    let mut logic = ExchangeLogic::new();
    // Simulate start first
    logic.state = ExchangeState::Open;

    let packet_me = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::ItemAdd as u8,
        is_me: 1, // true
        arg1: 999, // vnum
        arg2: TItemPos { window_type: 1, cell: 2 }, // cell 2
        arg3: 10, // count
        al_values: [100, 200, 300],
        a_attr: [
            TPlayerItemAttribute { b_type: 1, s_value: 10 },
            TPlayerItemAttribute { b_type: 2, s_value: 20 },
            TPlayerItemAttribute { b_type: 3, s_value: 30 },
            TPlayerItemAttribute::default(),
            TPlayerItemAttribute::default(),
            TPlayerItemAttribute::default(),
            TPlayerItemAttribute::default(),
        ],
    };

    logic.handle_packet(&packet_me);

    assert_eq!(logic.self_data.item_vnum[2], 999);
    assert_eq!(logic.self_data.item_count[2], 10);
    assert_eq!(logic.self_data.item_metin[2][0], 100);
    assert_eq!(logic.self_data.item_metin[2][2], 300);
    
    // Copy out from packed structure to avoid unaligned reference error
    let attr_0 = logic.self_data.item_attr[2][0];
    let attr_1 = logic.self_data.item_attr[2][1];

    assert_eq!(attr_0.b_type, 1);
    let s_value_1 = attr_1.s_value;
    assert_eq!(s_value_1, 20);

    // Ensure target data was untouched
    assert_eq!(logic.target_data.item_vnum[2], 0);
}

#[test]
fn test_exchange_item_del() {
    let mut logic = ExchangeLogic::new();
    logic.state = ExchangeState::Open;
    logic.self_data.item_vnum[5] = 888;
    logic.self_data.item_count[5] = 1;

    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::ItemDel as u8,
        is_me: 1,
        arg1: 5, // cell
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    logic.handle_packet(&packet);
    assert_eq!(logic.self_data.item_vnum[5], 0);
    assert_eq!(logic.self_data.item_count[5], 0);
}

#[test]
fn test_exchange_elk_add() {
    let mut logic = ExchangeLogic::new();
    logic.state = ExchangeState::Open;

    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::ElkAdd as u8,
        is_me: 0, // target
        arg1: 50000,
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    logic.handle_packet(&packet);
    assert_eq!(logic.target_data.elk, 50000);
    assert_eq!(logic.self_data.elk, 0);
}

#[test]
fn test_exchange_accept() {
    let mut logic = ExchangeLogic::new();
    logic.state = ExchangeState::Open;

    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::Accept as u8,
        is_me: 1, // self
        arg1: 1, // accepted
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    logic.handle_packet(&packet);
    assert_eq!(logic.self_data.accept, 1);
    assert_eq!(logic.target_data.accept, 0);
    assert_eq!(logic.state, ExchangeState::Locked);
}

#[test]
fn test_exchange_end() {
    let mut logic = ExchangeLogic::new();
    logic.state = ExchangeState::Locked;

    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::End as u8,
        is_me: 0,
        arg1: 0,
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    logic.handle_packet(&packet);
    assert_eq!(logic.state, ExchangeState::Completed);
}

#[test]
fn test_m2_net_recv_exchange_ffi() {
    let packet = TPacketGCExchange {
        header: 0x051C,
        length: 50,
        subheader: ExchangeSubHeader::Start as u8,
        is_me: 0,
        arg1: 777,
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [TPlayerItemAttribute::default(); 7],
    };

    let result = m2_net_recv_exchange(&packet as *const _ as *const u8, std::mem::size_of::<TPacketGCExchange>());
    assert!(result);

    let logic = get_exchange_logic().read().unwrap();
    assert_eq!(logic.state, ExchangeState::Open);
    assert_eq!(logic.target_vid, 777);
}
