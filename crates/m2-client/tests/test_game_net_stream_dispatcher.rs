use m2_client::game_net_stream_dispatcher::dispatch_packet_extended;
use m2_client::recv_target_info_v2::*;
use m2_client::recv_quest_dialog_v2::*;
use m2_client::recv_refine_blacksmith_v2::*;
use m2_client::recv_shop_logic_v2::*;
use m2_client::recv_exchange_logic_v2::*;
use m2_client::recv_party_logic_v2::*;
use m2_client::recv_warp_phase_v2::*;
use m2_client::recv_fishing_system_v2::*;
use m2_client::recv_dungeon_info_v2::*;
use m2_client::recv_messenger_friends_v2::*;
use m2_client::recv_ping_sync_v2::*;
use m2_client::recv_fly_bullet_v2::*;
use std::mem::size_of;
use std::ptr;

#[test]
fn test_dispatch_target_and_script() {
    // TARGET test
    let mut target_pkt = [0u8; size_of::<TPacketGCTarget>()];
    let packet = TPacketGCTarget {
        header: 0x0A10,
        length: size_of::<TPacketGCTarget>() as u16,
        dwVID: 1234,
        bHPPercent: 50,
    };
    unsafe {
        ptr::write_unaligned(target_pkt.as_mut_ptr() as *mut TPacketGCTarget, packet);
    }
    
    assert!(dispatch_packet_extended(0x0A10, &target_pkt));
    
    let target_state = m2_client::recv_target_info_v2::get_target_state().lock().unwrap();
    assert_eq!(target_state.target_vid, 1234);
    assert_eq!(target_state.hp_percent, 50);

    // SCRIPT test
    let header_size = size_of::<TPacketGCScript>();
    let text = "test script";
    let text_len = text.len();
    let total_len = header_size + text_len + 1; // +1 for null terminator
    
    let mut script_pkt = vec![0u8; total_len];
    let packet_script = TPacketGCScript {
        header: 0x0910,
        length: total_len as u16,
        skin: 1,
        src_size: text_len as u16,
    };
    
    unsafe {
        ptr::write_unaligned(script_pkt.as_mut_ptr() as *mut TPacketGCScript, packet_script);
        ptr::copy_nonoverlapping(text.as_ptr(), script_pkt.as_mut_ptr().add(header_size), text_len);
    }
    
    assert!(dispatch_packet_extended(0x0910, &script_pkt));

    // REFINE test
    let mut refine_pkt = [0u8; size_of::<TPacketGCRefineInformation>()];
    let packet_refine = TPacketGCRefineInformation {
        target_pos: 10,
        result_vnum: 200,
        cost: 1000,
        prob: 50,
        material_count: 0,
        materials: [TMaterial { vnum: 0, count: 0 }; REFINE_MATERIAL_MAX_NUM],
    };
    unsafe {
        ptr::write_unaligned(refine_pkt.as_mut_ptr() as *mut TPacketGCRefineInformation, packet_refine);
    }
    assert!(dispatch_packet_extended(0x051D, &refine_pkt));
}

#[test]
fn test_dispatch_shop_exchange() {
    // SHOP test - subheader GC_END to avoid payload length checks
    let mut shop_pkt = vec![0u8; size_of::<TPacketGCShop>()];
    let packet_shop = TPacketGCShop {
        header: 0x0810,
        length: size_of::<TPacketGCShop>() as u16,
        subheader: SHOP_SUBHEADER_GC_END,
    };
    unsafe {
        ptr::write_unaligned(shop_pkt.as_mut_ptr() as *mut TPacketGCShop, packet_shop);
    }
    
    // Check lengths inside SHOP parser: it expects shop_pkt.len() >= packet_shop.length
    assert!(dispatch_packet_extended(0x0810, &shop_pkt));
    
    let shop = SHOP_INSTANCE.read().unwrap();
    assert_eq!(shop.is_open, false);

    // EXCHANGE test - subheader End
    let mut exchange_pkt = [0u8; size_of::<TPacketGCExchange>()];
    let packet_exchange = TPacketGCExchange {
        header: 0x05, 
        length: size_of::<TPacketGCExchange>() as u16,
        subheader: 5, // ExchangeSubHeader::End
        is_me: false,
        arg1: 0,
        arg2: TItemPos::default(),
        arg3: 0,
        al_values: [0; 3],
        a_attr: [m2_client::recv_exchange_logic_v2::TPlayerItemAttribute::default(); 7],
    };
    unsafe {
        ptr::write_unaligned(exchange_pkt.as_mut_ptr() as *mut TPacketGCExchange, packet_exchange);
    }
    assert!(dispatch_packet_extended(0x051C, &exchange_pkt));
    
    let logic = get_exchange_logic().read().unwrap();
    assert_eq!(logic.state, ExchangeState::Completed);
}
