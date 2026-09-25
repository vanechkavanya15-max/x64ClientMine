use m2_client::guild_mark_downloader::*;

#[test]
fn test_integration_guild_mark_downloader() {
    let mut instance = GuildMarkDownloader::new();
    assert!(!instance.is_ready());
}

#[test]
fn test_downloader_connect_and_recv_phase() {
    let mut downloader = GuildMarkDownloader::new();
    assert_eq!(downloader.state, State::Offline);

    let connected = downloader.connect(12345, 67890);
    assert!(connected);
    assert_eq!(downloader.state, State::Login);
    assert_eq!(downloader.todo, Todo::RecvMark);

    // Inject GCPhase packet
    let phase_packet = PacketGCPhase {
        header: HEADER_GC_PHASE,
        phase: PHASE_LOGIN,
    };
    
    let packet_bytes = unsafe {
        std::slice::from_raw_parts(&phase_packet as *const _ as *const u8, std::mem::size_of::<PacketGCPhase>())
    };
    
    downloader.recv_buffer.extend_from_slice(packet_bytes);
    
    downloader.process();
    
    // Security mode should be enabled after PHASE_LOGIN
    assert!(downloader.security_mode);
    
    // Todo is RecvMark, so it should have sent HEADER_CG_MARK_IDXLIST
    assert_eq!(downloader.send_buffer.len(), std::mem::size_of::<PacketCGMarkIdxList>());
    let sent_packet = unsafe {
        std::ptr::read_unaligned(downloader.send_buffer.as_ptr() as *const PacketCGMarkIdxList)
    };
    assert_eq!(sent_packet.header, HEADER_CG_MARK_IDXLIST);
}

#[test]
fn test_downloader_recv_handshake() {
    let mut downloader = GuildMarkDownloader::new();
    downloader.connect(111, 222);
    
    let handshake_packet = PacketGCHandshake {
        header: HEADER_GC_HANDSHAKE,
        handshake: 555,
        time: 777,
        delta: 999,
    };
    
    let packet_bytes = unsafe {
        std::slice::from_raw_parts(&handshake_packet as *const _ as *const u8, std::mem::size_of::<PacketGCHandshake>())
    };
    
    downloader.recv_buffer.extend_from_slice(packet_bytes);
    downloader.process();
    
    assert_eq!(downloader.send_buffer.len(), std::mem::size_of::<PacketCGMarkLogin>());
    let sent_packet = unsafe {
        std::ptr::read_unaligned(downloader.send_buffer.as_ptr() as *const PacketCGMarkLogin)
    };
    assert_eq!(sent_packet.header, HEADER_CG_MARK_LOGIN);
    let handle = sent_packet.handle;
    assert_eq!(handle, 111);
    let random_key = sent_packet.random_key;
    assert_eq!(random_key, 222);
}

#[test]
fn test_downloader_recv_ping() {
    let mut downloader = GuildMarkDownloader::new();
    downloader.connect(111, 222);
    
    let ping_packet = PacketGCPing {
        header: HEADER_GC_PING,
    };
    
    let packet_bytes = unsafe {
        std::slice::from_raw_parts(&ping_packet as *const _ as *const u8, std::mem::size_of::<PacketGCPing>())
    };
    
    downloader.recv_buffer.extend_from_slice(packet_bytes);
    downloader.process();
    
    assert_eq!(downloader.send_buffer.len(), std::mem::size_of::<PacketCGPong>());
    let sent_packet = unsafe {
        std::ptr::read_unaligned(downloader.send_buffer.as_ptr() as *const PacketCGPong)
    };
    assert_eq!(sent_packet.header, HEADER_CG_PONG);
}

#[test]
fn test_downloader_recv_mark_index() {
    let mut downloader = GuildMarkDownloader::new();
    downloader.connect(111, 222);
    
    let mark_idx_packet = PacketGCMarkIdxList {
        header: HEADER_GC_MARK_IDXLIST,
        buf_size: 0, // unused in rust mock
        count: 2,
    };
    
    let packet_bytes = unsafe {
        std::slice::from_raw_parts(&mark_idx_packet as *const _ as *const u8, std::mem::size_of::<PacketGCMarkIdxList>())
    };
    
    downloader.recv_buffer.extend_from_slice(packet_bytes);
    // Add data for 2 marks
    downloader.recv_buffer.extend_from_slice(&10u16.to_le_bytes()); // guild_id 10
    downloader.recv_buffer.extend_from_slice(&20u16.to_le_bytes()); // mark_id 20
    downloader.recv_buffer.extend_from_slice(&30u16.to_le_bytes()); // guild_id 30
    downloader.recv_buffer.extend_from_slice(&40u16.to_le_bytes()); // mark_id 40
    
    downloader.process();
    
    assert_eq!(downloader.marks.get(&10), Some(&20));
    assert_eq!(downloader.marks.get(&30), Some(&40));
    
    // Should have sent CRC list request
    assert_eq!(downloader.send_buffer.len(), std::mem::size_of::<PacketCGMarkCrcList>());
    let sent_packet = unsafe {
        std::ptr::read_unaligned(downloader.send_buffer.as_ptr() as *const PacketCGMarkCrcList)
    };
    assert_eq!(sent_packet.header, HEADER_CG_MARK_CRCLIST);
    assert_eq!(sent_packet.img_idx, 0);
    assert_eq!(downloader.current_requesting_image_index, 1);
}

#[test]
fn test_downloader_recv_symbol_data() {
    let mut downloader = GuildMarkDownloader::new();
    downloader.connect_to_recv_symbol(1, 2, vec![100, 200]);
    
    let data = vec![1, 2, 3, 4, 5];
    let symbol_data_packet = PacketGCGuildSymbolData {
        header: HEADER_GC_GUILD_SYMBOL_DATA,
        size: (std::mem::size_of::<PacketGCGuildSymbolData>() + data.len()) as u16,
        guild_id: 100,
    };
    
    let packet_bytes = unsafe {
        std::slice::from_raw_parts(&symbol_data_packet as *const _ as *const u8, std::mem::size_of::<PacketGCGuildSymbolData>())
    };
    
    downloader.recv_buffer.extend_from_slice(packet_bytes);
    downloader.recv_buffer.extend_from_slice(&data);
    
    downloader.process();
    
    assert!(downloader.recv_buffer.is_empty());
}
