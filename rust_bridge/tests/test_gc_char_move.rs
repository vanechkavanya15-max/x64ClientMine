use m2_client_bridge::net::gc::char_move::{
    PacketError, TPacketGCCharacterMove, m2_net_free_char_move, m2_net_parse_char_move,
};
use std::ptr;

#[test]
fn test_parse_valid_buffer() {
    // Naglowek: 3 (HEADER_GC_CHARACTER_MOVE)
    // b_func: 1, b_arg: 2, b_rot: 3
    // dw_vid: 0x01020304
    // l_x: 0x05060708 (84281096)
    // l_y: 0x090A0B0C (151659276)
    // dw_time: 0x0D0E0F10
    // dw_duration: 0x11121314

    let buf: [u8; 24] = [
        0x03, 0x01, 0x02, 0x03, // Header, Func, Arg, Rot
        0x04, 0x03, 0x02, 0x01, // VID (Little Endian)
        0x08, 0x07, 0x06, 0x05, // X (Little Endian)
        0x0C, 0x0B, 0x0A, 0x09, // Y (Little Endian)
        0x10, 0x0F, 0x0E, 0x0D, // Time (Little Endian)
        0x14, 0x13, 0x12, 0x11, // Duration (Little Endian)
    ];

    let result = TPacketGCCharacterMove::from_bytes(&buf);
    assert!(
        result.is_ok(),
        "Oczekiwano poprawnego parsowania, otrzymano: {:?}",
        result
    );

    let packet = result.unwrap();
    assert_eq!(packet.header, 3);
    assert_eq!(packet.b_func, 1);
    assert_eq!(packet.b_arg, 2);
    assert_eq!(packet.b_rot, 3);
    assert_eq!({ packet.dw_vid }, 0x01020304);
    assert_eq!({ packet.l_x }, 0x05060708);
    assert_eq!({ packet.l_y }, 0x090A0B0C);
    assert_eq!({ packet.dw_time }, 0x0D0E0F10);
    assert_eq!({ packet.dw_duration }, 0x11121314);
}

#[test]
fn test_parse_buffer_too_short() {
    let buf: [u8; 23] = [0; 23];
    let result = TPacketGCCharacterMove::from_bytes(&buf);

    assert!(
        result.is_err(),
        "Oczekiwano bledu parsowania dla zbyt krotkiego bufora"
    );

    let err = result.unwrap_err();
    assert_eq!(
        err,
        PacketError::BufferTooShort,
        "Oczekiwano bledu BufferTooShort, otrzymano: {:?}",
        err
    );
}

#[test]
fn test_ffi_null_pointers() {
    let mut out_ptr: *mut TPacketGCCharacterMove = ptr::null_mut();
    let buf: [u8; 24] = [0; 24];

    // Data is null
    let res = unsafe { m2_net_parse_char_move(ptr::null(), 24, &mut out_ptr) };
    assert_eq!(res, false);
    assert!(out_ptr.is_null());

    // Out pointer is null
    let res2 = unsafe { m2_net_parse_char_move(buf.as_ptr(), 24, ptr::null_mut()) };
    assert_eq!(res2, false);

    // Free null pointer - should not panic
    unsafe { m2_net_free_char_move(ptr::null_mut()) };
}

#[test]
fn test_ffi_alloc_and_free() {
    let buf: [u8; 24] = [
        0x03, 0x01, 0x02, 0x03, // Header, Func, Arg, Rot
        0x04, 0x03, 0x02, 0x01, // VID (Little Endian)
        0x08, 0x07, 0x06, 0x05, // X (Little Endian)
        0x0C, 0x0B, 0x0A, 0x09, // Y (Little Endian)
        0x10, 0x0F, 0x0E, 0x0D, // Time (Little Endian)
        0x14, 0x13, 0x12, 0x11, // Duration (Little Endian)
    ];

    let mut out_ptr: *mut TPacketGCCharacterMove = ptr::null_mut();

    let res = unsafe { m2_net_parse_char_move(buf.as_ptr(), 24, &mut out_ptr) };
    assert_eq!(res, true);
    assert!(!out_ptr.is_null());

    let packet = unsafe { &*out_ptr };
    assert_eq!(packet.header, 3);
    assert_eq!(packet.b_func, 1);
    assert_eq!(packet.b_arg, 2);
    assert_eq!(packet.b_rot, 3);
    assert_eq!({ packet.dw_vid }, 0x01020304);
    assert_eq!({ packet.l_x }, 0x05060708);
    assert_eq!({ packet.l_y }, 0x090A0B0C);
    assert_eq!({ packet.dw_time }, 0x0D0E0F10);
    assert_eq!({ packet.dw_duration }, 0x11121314);

    unsafe { m2_net_free_char_move(out_ptr) };
}
