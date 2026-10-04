use m2_client::tcp_stream_reassembler::TcpStreamReassembler;

// Constants matching the C++ source
const HEADER_GC_QUICKSLOT_ADD: u16 = 0x0519; // GC::QUICKSLOT_ADD from Packet.h
const HEADER_GC_CHAT: u16 = 0x0603;          // GC::CHAT from Packet.h

#[test]
fn test_reassemble_gc_chat_byte_by_byte() {
    let mut reassembler = TcpStreamReassembler::new(1024);

    // Build a mock TPacketGCChat (header + length + type + vid + empire) + dynamic string
    // length = 2 (header) + 2 (length) + 1 (type) + 4 (vid) + 1 (empire) = 10
    // plus say "hello" = 5 bytes
    // total packet length = 15 bytes
    let packet_len: u16 = 15;
    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&HEADER_GC_CHAT.to_le_bytes());
    packet_data.extend_from_slice(&packet_len.to_le_bytes());
    packet_data.push(0); // type (CHAT_TYPE_TALKING)
    packet_data.extend_from_slice(&1234u32.to_le_bytes()); // vid
    packet_data.push(1); // empire
    packet_data.extend_from_slice(b"hello");

    assert_eq!(packet_data.len(), 15);

    // Send it byte by byte
    for (i, byte) in packet_data.iter().enumerate() {
        reassembler.write(&[*byte]);

        if i < packet_data.len() - 1 {
            assert!(reassembler.read_packet().is_none(), "Should not return packet before it's fully received");
        } else {
            let received = reassembler.read_packet().expect("Should return packet when fully received");
            assert_eq!(received, packet_data, "Reassembled packet should match the sent one");
        }
    }

    assert!(reassembler.read_packet().is_none(), "No more packets should be available");
}

#[test]
fn test_reassemble_gc_quickslot_add_byte_by_byte() {
    let mut reassembler = TcpStreamReassembler::new(1024);

    // Build a mock TPacketGCQuickSlotAdd
    // length = 2 (header) + 2 (length) + 1 (pos) + 2 (TQuickSlot) = 7 bytes
    let packet_len: u16 = 7;
    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&HEADER_GC_QUICKSLOT_ADD.to_le_bytes());
    packet_data.extend_from_slice(&packet_len.to_le_bytes());
    packet_data.push(5); // pos
    packet_data.push(1); // TQuickSlot Type
    packet_data.push(2); // TQuickSlot Position

    assert_eq!(packet_data.len(), 7);

    // Send it byte by byte
    for (i, byte) in packet_data.iter().enumerate() {
        reassembler.write(&[*byte]);

        if i < packet_data.len() - 1 {
            assert!(reassembler.read_packet().is_none(), "Should not return packet before it's fully received");
        } else {
            let received = reassembler.read_packet().expect("Should return packet when fully received");
            assert_eq!(received, packet_data, "Reassembled packet should match the sent one");
        }
    }

    assert!(reassembler.read_packet().is_none(), "No more packets should be available");
}

#[test]
fn test_reassemble_multiple_packets_and_compaction() {
    // Small buffer to force compaction
    let mut reassembler = TcpStreamReassembler::new(10);

    // Packet 1: 8 bytes
    let packet1_len: u16 = 8;
    let mut packet1 = Vec::new();
    packet1.extend_from_slice(&0xAAAAu16.to_le_bytes());
    packet1.extend_from_slice(&packet1_len.to_le_bytes());
    packet1.extend_from_slice(&[1, 2, 3, 4]);

    // Packet 2: 6 bytes
    let packet2_len: u16 = 6;
    let mut packet2 = Vec::new();
    packet2.extend_from_slice(&0xBBBBu16.to_le_bytes());
    packet2.extend_from_slice(&packet2_len.to_le_bytes());
    packet2.extend_from_slice(&[5, 6]);

    // Write both at once
    let mut combined = Vec::new();
    combined.extend_from_slice(&packet1);
    combined.extend_from_slice(&packet2);

    reassembler.write(&combined);

    // Read first packet
    let p1 = reassembler.read_packet().expect("First packet should be available");
    assert_eq!(p1, packet1);

    // Read second packet
    let p2 = reassembler.read_packet().expect("Second packet should be available");
    assert_eq!(p2, packet2);

    assert!(reassembler.read_packet().is_none(), "No more packets should be available");
}

#[test]
fn test_reassemble_1_byte_fragments() {
    let mut reassembler = TcpStreamReassembler::new(1024);

    let header: u16 = 0x9999;
    let packet_len: u16 = 6;
    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&header.to_le_bytes());
    packet_data.extend_from_slice(&packet_len.to_le_bytes());
    packet_data.extend_from_slice(&[0x42, 0x43]);

    assert_eq!(packet_data.len(), 6);

    for (i, byte) in packet_data.iter().enumerate() {
        reassembler.write(&[*byte]);

        if i < packet_data.len() - 1 {
            assert!(reassembler.read_packet().is_none(), "Should not return packet before it's fully received");
        } else {
            let received = reassembler.read_packet().expect("Should return packet when fully received");
            assert_eq!(received, packet_data, "Reassembled packet should match the sent one");
        }
    }

    assert!(reassembler.read_packet().is_none(), "No more packets should be available");
}
