use m2_client::net_mark_server_protocol_handler::{
    MarkLoginPacket, NetMarkServerProtocolHandler, DEFAULT_MARK_SERVER_PORT, HEADER_CG_MARK_IDX_LIST,
    HEADER_CG_MARK_LOGIN, HEADER_GC_MARK_BLOCK, HEADER_GC_MARK_IDX_LIST,
};

#[test]
fn test_mark_login_packet_serialization() {
    let pkt = MarkLoginPacket::new(12345, 99999);
    let bytes = pkt.to_bytes();

    assert_eq!(bytes[0], HEADER_CG_MARK_LOGIN);
    let handle = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
    let key = u32::from_le_bytes(bytes[5..9].try_into().unwrap());
    assert_eq!(handle, 12345);
    assert_eq!(key, 99999);
}

#[test]
fn test_mark_request_packet_generation() {
    let handler = NetMarkServerProtocolHandler::new("127.0.0.1", DEFAULT_MARK_SERVER_PORT);
    let req = handler.build_request_idx_list_packet();
    assert_eq!(req, vec![HEADER_CG_MARK_IDX_LIST]);
}

#[test]
fn test_mark_idx_list_response_parsing() {
    let mut handler = NetMarkServerProtocolHandler::new("127.0.0.1", 10020);

    // Pakiet z dwoma wpisami gildii: gildia 10 -> mark 1, gildia 20 -> mark 5
    let mut payload = Vec::new();
    payload.push(HEADER_GC_MARK_IDX_LIST);
    payload.extend_from_slice(&2u32.to_le_bytes()); // count = 2
    // Wpis 1
    payload.extend_from_slice(&10u32.to_le_bytes());
    payload.extend_from_slice(&1u32.to_le_bytes());
    // Wpis 2
    payload.extend_from_slice(&20u32.to_le_bytes());
    payload.extend_from_slice(&5u32.to_le_bytes());

    let count = handler.parse_idx_list_response(&payload).expect("Parsowanie powinno przejsc");
    assert_eq!(count, 2);
    assert_eq!(handler.get_mark_id(10), Some(1));
    assert_eq!(handler.get_mark_id(20), Some(5));
    assert_eq!(handler.get_mark_id(30), None);
}

#[test]
fn test_mark_block_response_parsing() {
    let mut handler = NetMarkServerProtocolHandler::new("127.0.0.1", 10020);

    let mut payload = Vec::new();
    payload.push(HEADER_GC_MARK_BLOCK);
    payload.extend_from_slice(&42u32.to_le_bytes()); // image_index = 42
    payload.extend_from_slice(&[255, 0, 0, 255]); // 1 czerwony piksel

    let img_idx = handler.parse_mark_block_response(&payload).expect("Powinno sparsowac blok");
    assert_eq!(img_idx, 42);
    assert_eq!(handler.downloaded_blocks.get(&42).unwrap(), &[255, 0, 0, 255]);
}
