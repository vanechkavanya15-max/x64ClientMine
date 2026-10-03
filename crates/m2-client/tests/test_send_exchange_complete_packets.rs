use m2_client::send_exchange_complete_packets::*;

#[test]
fn test_packet_size_and_layout() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
    assert_eq!(std::mem::size_of::<TPacketCGExchange>(), 13);
}

#[test]
fn test_m2_send_exchange_start() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_start(12345, out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    let arg1 = packet.arg1;
    let arg2 = packet.arg2;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_START);
    assert_eq!(arg1, 12345);
    assert_eq!(arg2, 0);
}

#[test]
fn test_m2_send_exchange_item_add() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_item_add(1, 15, 2, out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    let pos_window_type = packet.pos.window_type;
    let pos_cell = packet.pos.cell;
    let arg2 = packet.arg2;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_ITEM_ADD);
    assert_eq!(pos_window_type, 1);
    assert_eq!(pos_cell, 15);
    assert_eq!(arg2, 2);
}

#[test]
fn test_m2_send_exchange_item_del() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_item_del(5, out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    let arg1 = packet.arg1;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_ITEM_DEL);
    assert_eq!(arg1, 5);
}

#[test]
fn test_m2_send_exchange_elk_add() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_elk_add(999999, out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    let arg1 = packet.arg1;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_ELK_ADD);
    assert_eq!(arg1, 999999);
}

#[test]
fn test_m2_send_exchange_accept() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_accept(out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_ACCEPT);
}

#[test]
fn test_m2_send_exchange_exit() {
    let mut out_buf = [0u8; 13];
    let len = m2_send_exchange_new_exit(out_buf.as_mut_ptr());
    
    assert_eq!(len, 13);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    let header = packet.header;
    let length = packet.length;
    let subheader = packet.subheader;
    assert_eq!(header, HEADER_CG_EXCHANGE);
    assert_eq!(length, 13);
    assert_eq!(subheader, EXCHANGE_SUB_EXIT);
}

#[test]
fn test_null_buffer() {
    assert_eq!(m2_send_exchange_new_start(12345, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_new_item_add(1, 15, 2, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_new_item_del(5, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_new_elk_add(999999, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_new_accept(std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_new_exit(std::ptr::null_mut()), 0);
}
