#[path = "../src/send_exchange_trade_ops.rs"]
mod send_exchange_trade_ops;

use send_exchange_trade_ops::{
    m2_send_exchange_ops_start,
    m2_send_exchange_ops_item_add,
    m2_send_exchange_ops_item_del,
    m2_send_exchange_ops_elk_add,
    m2_send_exchange_ops_accept,
    m2_send_exchange_ops_cancel,
    TPacketCGExchange,
    HEADER_CG_EXCHANGE,
    EXCHANGE_SUBHEADER_CG_START,
    EXCHANGE_SUBHEADER_CG_ITEM_ADD,
    EXCHANGE_SUBHEADER_CG_ITEM_DEL,
    EXCHANGE_SUBHEADER_CG_ELK_ADD,
    EXCHANGE_SUBHEADER_CG_ACCEPT,
    EXCHANGE_SUBHEADER_CG_CANCEL,
};

#[test]
fn test_send_exchange_start() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_start(1234, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_START);
    assert_eq!({packet.arg1}, 1234);
    assert_eq!({packet.arg2}, 0);
    assert_eq!({packet.pos.cell}, 0xffff);

    let written_null = m2_send_exchange_ops_start(1234, std::ptr::null_mut());
    assert_eq!(written_null, 0);
}

#[test]
fn test_send_exchange_item_add() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_item_add(1, 42, 5, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_ITEM_ADD);
    assert_eq!({packet.pos.window_type}, 1);
    assert_eq!({packet.pos.cell}, 42);
    assert_eq!({packet.arg2}, 5);

    let written_null = m2_send_exchange_ops_item_add(1, 42, 5, std::ptr::null_mut());
    assert_eq!(written_null, 0);
}

#[test]
fn test_send_exchange_item_del() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_item_del(7, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_ITEM_DEL);
    assert_eq!({packet.arg1}, 7);

    let written_null = m2_send_exchange_ops_item_del(7, std::ptr::null_mut());
    assert_eq!(written_null, 0);
}

#[test]
fn test_send_exchange_elk_add() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_elk_add(9999, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_ELK_ADD);
    assert_eq!({packet.arg1}, 9999);

    let written_null = m2_send_exchange_ops_elk_add(9999, std::ptr::null_mut());
    assert_eq!(written_null, 0);
}

#[test]
fn test_send_exchange_accept() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_accept(out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_ACCEPT);

    let written_null = m2_send_exchange_ops_accept(std::ptr::null_mut());
    assert_eq!(written_null, 0);
}

#[test]
fn test_send_exchange_cancel() {
    let mut out_buf = [0u8; 128];
    let expected_len = std::mem::size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_ops_cancel(out_buf.as_mut_ptr());
    assert_eq!(written, expected_len);
    
    let packet: TPacketCGExchange = unsafe { std::ptr::read(out_buf.as_ptr() as *const _) };
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length} as usize, expected_len);
    assert_eq!({packet.subheader}, EXCHANGE_SUBHEADER_CG_CANCEL);

    let written_null = m2_send_exchange_ops_cancel(std::ptr::null_mut());
    assert_eq!(written_null, 0);
}
