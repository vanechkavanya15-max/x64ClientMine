use m2_client::phase_game_dispatcher_v2::*;
use std::mem::size_of;

#[test]
fn test_packet_too_short() {
    let buf: [u8; 2] = [0, 0];
    let res = m2_net_dispatch_game_packet_v2(buf.as_ptr(), buf.len());
    assert_eq!(res, 0);
}

#[test]
fn test_packet_invalid_length() {
    let hdr = TDynamicSizePacketHeader {
        header: gc::PHASE,
        length: 2, // too small
    };
    let buf: *const u8 = &hdr as *const _ as *const u8;
    let res = m2_net_dispatch_game_packet_v2(buf, size_of::<TDynamicSizePacketHeader>());
    assert_eq!(res, 0);
}

#[test]
fn test_valid_phase_packet() {
    #[repr(C, packed)]
    struct PhaseData {
        hdr: TDynamicSizePacketHeader,
        payload: [u8; 5 - size_of::<TDynamicSizePacketHeader>()],
    }
    let data = PhaseData {
        hdr: TDynamicSizePacketHeader {
            header: gc::PHASE,
            length: 5 as u16,
        },
        payload: [0; 5 - size_of::<TDynamicSizePacketHeader>()],
    };
    
    let buf: *const u8 = &data as *const _ as *const u8;
    let res = m2_net_dispatch_game_packet_v2(buf, size_of::<PhaseData>());
    assert_eq!(res, gc::PHASE);
}

#[test]
fn test_incomplete_packet() {
    #[repr(C, packed)]
    struct PhaseData {
        hdr: TDynamicSizePacketHeader,
        payload: [u8; 5 - size_of::<TDynamicSizePacketHeader>()],
    }
    let data = PhaseData {
        hdr: TDynamicSizePacketHeader {
            header: gc::PHASE,
            length: 5 as u16,
        },
        payload: [0; 5 - size_of::<TDynamicSizePacketHeader>()],
    };
    
    let buf: *const u8 = &data as *const _ as *const u8;
    // Pass len < length
    let res = m2_net_dispatch_game_packet_v2(buf, size_of::<TDynamicSizePacketHeader>());
    assert_eq!(res, 0);
}

#[test]
fn test_unknown_packet() {
    #[repr(C, packed)]
    struct UnknownData {
        hdr: TDynamicSizePacketHeader,
        payload: [u8; 10],
    }
    let data = UnknownData {
        hdr: TDynamicSizePacketHeader {
            header: 0xFFFF,
            length: size_of::<UnknownData>() as u16,
        },
        payload: [0; 10],
    };
    
    let buf: *const u8 = &data as *const _ as *const u8;
    let res = m2_net_dispatch_game_packet_v2(buf, size_of::<UnknownData>());
    assert_eq!(res, 0xFFFF);
}
