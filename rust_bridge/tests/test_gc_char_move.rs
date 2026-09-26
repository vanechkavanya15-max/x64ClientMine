use m2_client_bridge::net::gc::char_move::{
    m2_net_free_char_move, m2_net_parse_char_move, PacketError, TPacketGCMove,
    HEADER_GC_MOVE,
};
use std::ptr;

#[test]
fn test_parse_valid_buffer() {
    let mut buf = [0u8; 27];
    buf[0..2].copy_from_slice(&HEADER_GC_MOVE.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&27u16.to_le_bytes());           // length
    buf[4] = 1;                                               // bFunc
    buf[5] = 2;                                               // bArg
    buf[6] = 3;                                               // bRot
    buf[7..11].copy_from_slice(&0x01020304u32.to_le_bytes()); // dwVID
    buf[11..15].copy_from_slice(&0x05060708i32.to_le_bytes()); // lX
    buf[15..19].copy_from_slice(&0x090A0B0Ci32.to_le_bytes()); // lY
    buf[19..23].copy_from_slice(&0x0D0E0F10u32.to_le_bytes()); // dwTime
    buf[23..27].copy_from_slice(&0x11121314u32.to_le_bytes()); // dwDuration

    let result = TPacketGCMove::from_bytes(&buf);
    assert!(
        result.is_ok(),
        "Oczekiwano poprawnego parsowania, otrzymano: {:?}",
        result
    );

    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_MOVE);
    assert_eq!({ packet.length }, 27);
    assert_eq!({ packet.bFunc }, 1);
    assert_eq!({ packet.bArg }, 2);
    assert_eq!({ packet.bRot }, 3);
    assert_eq!(packet.dw_vid(), 0x01020304);
    assert_eq!(packet.l_x(), 0x05060708);
    assert_eq!(packet.l_y(), 0x090A0B0C);
    assert_eq!(packet.dw_time(), 0x0D0E0F10);
    assert_eq!(packet.dw_duration(), 0x11121314);
}

#[test]
fn test_parse_buffer_too_short() {
    let buf: [u8; 26] = [0; 26];
    let result = TPacketGCMove::from_bytes(&buf);

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
    let mut out_ptr: *mut TPacketGCMove = ptr::null_mut();
    let buf: [u8; 27] = [0; 27];

    // Data is null
    let res = unsafe { m2_net_parse_char_move(ptr::null(), 27, &mut out_ptr) };
    assert_eq!(res, false);
    assert!(out_ptr.is_null());

    // Out pointer is null
    let res2 = unsafe { m2_net_parse_char_move(buf.as_ptr(), 27, ptr::null_mut()) };
    assert_eq!(res2, false);

    // Free null pointer - should not panic
    unsafe { m2_net_free_char_move(ptr::null_mut()) };
}

#[test]
fn test_ffi_alloc_and_free() {
    let mut buf = [0u8; 27];
    buf[0..2].copy_from_slice(&HEADER_GC_MOVE.to_le_bytes());
    buf[2..4].copy_from_slice(&27u16.to_le_bytes());
    buf[4] = 1;
    buf[5] = 2;
    buf[6] = 3;
    buf[7..11].copy_from_slice(&0x01020304u32.to_le_bytes());
    buf[11..15].copy_from_slice(&0x05060708i32.to_le_bytes());
    buf[15..19].copy_from_slice(&0x090A0B0Ci32.to_le_bytes());
    buf[19..23].copy_from_slice(&0x0D0E0F10u32.to_le_bytes());
    buf[23..27].copy_from_slice(&0x11121314u32.to_le_bytes());

    let mut out_ptr: *mut TPacketGCMove = ptr::null_mut();

    let res = unsafe { m2_net_parse_char_move(buf.as_ptr(), 27, &mut out_ptr) };
    assert_eq!(res, true);
    assert!(!out_ptr.is_null());

    let packet = unsafe { &*out_ptr };
    assert_eq!({ packet.header }, HEADER_GC_MOVE);
    assert_eq!({ packet.length }, 27);
    assert_eq!({ packet.bFunc }, 1);
    assert_eq!({ packet.bArg }, 2);
    assert_eq!({ packet.bRot }, 3);
    assert_eq!(packet.dw_vid(), 0x01020304);
    assert_eq!(packet.l_x(), 0x05060708);
    assert_eq!(packet.l_y(), 0x090A0B0C);
    assert_eq!(packet.dw_time(), 0x0D0E0F10);
    assert_eq!(packet.dw_duration(), 0x11121314);

    unsafe { m2_net_free_char_move(out_ptr) };
}
