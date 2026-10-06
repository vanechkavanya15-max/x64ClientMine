#[path = "../src/fishing_gameplay_minigame_v2.rs"]
mod fishing_gameplay_minigame_v2;

use fishing_gameplay_minigame_v2::*;
use std::mem::size_of;

#[test]
fn test_fishing_struct_sizes_and_alignment() {
    assert_eq!(size_of::<TPacketCGFishing>(), 5); // 2 + 2 + 1
    assert_eq!(size_of::<TPacketGCFishing>(), 10); // 2 + 2 + 1 + 4 + 1
}

#[test]
fn test_fishing_state_machine_success() {
    let mut minigame = FishingGameplayMinigame::new();
    assert_eq!(minigame.current_state, FishingState::Idle);

    let start_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::START,
        info: 0,
        dir: 5,
    };
    let res = minigame.handle_gc_fishing_packet(&start_packet).unwrap();
    assert_eq!(res, None);
    assert_eq!(minigame.current_state, FishingState::Waiting);
    assert_eq!(minigame.last_direction, 5);

    let react_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::REACT,
        info: 0,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&react_packet).unwrap();
    assert_eq!(res, None);
    assert_eq!(minigame.current_state, FishingState::Reacting);
    assert_eq!(minigame.is_fish_emoticon_visible, true);

    let success_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::SUCCESS,
        info: 0,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&success_packet).unwrap();
    assert_eq!(res, None);
    assert_eq!(minigame.current_state, FishingState::Success);
    assert_eq!(minigame.is_fish_emoticon_visible, false);
}

#[test]
fn test_fishing_state_machine_fail() {
    let mut minigame = FishingGameplayMinigame::new();
    
    // Force to reacting state
    minigame.start_fishing(0).unwrap();
    minigame.react_fishing().unwrap();

    let fail_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::FAIL,
        info: 0,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&fail_packet).unwrap();
    assert_eq!(res, Some("OnFishingFailure".to_string()));
    assert_eq!(minigame.current_state, FishingState::Fail);
}

#[test]
fn test_fishing_state_machine_stop() {
    let mut minigame = FishingGameplayMinigame::new();
    
    minigame.start_fishing(0).unwrap();

    let stop_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::STOP,
        info: 0,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&stop_packet).unwrap();
    assert_eq!(res, None);
    assert_eq!(minigame.current_state, FishingState::Idle);
}

#[test]
fn test_fishing_fish_item_notify() {
    let mut minigame = FishingGameplayMinigame::new();
    
    // 1. Unknown fish
    let unk_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::FISH,
        info: 0,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&unk_packet).unwrap();
    assert_eq!(res, Some("OnFishingNotifyUnknown".to_string()));
    
    // 2. Known fish, currently fishing -> Notify
    minigame.start_fishing(0).unwrap();
    let notify_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::FISH,
        info: 27799,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&notify_packet).unwrap();
    assert_eq!(res, Some("OnFishingNotify(27799, FISH)".to_string()));
    
    // 3. Known fish, NOT fishing -> Success
    minigame.stop_fishing().unwrap();
    let success_notify_packet = TPacketGCFishing {
        header: packet_headers::GC_FISHING,
        length: size_of::<TPacketGCFishing>() as u16,
        subheader: fishing_sub_gc::FISH,
        info: 27799,
        dir: 0,
    };
    let res = minigame.handle_gc_fishing_packet(&success_notify_packet).unwrap();
    assert_eq!(res, Some("OnFishingSuccess(27799, FISH)".to_string()));
}

#[test]
fn test_create_cg_packet() {
    let packet = FishingGameplayMinigame::create_cg_fishing_packet(12);
    let header = packet.header;
    let length = packet.length;
    let dir = packet.dir;
    assert_eq!(header, packet_headers::CG_FISHING);
    assert_eq!(length, 5);
    assert_eq!(dir, 12);
}
