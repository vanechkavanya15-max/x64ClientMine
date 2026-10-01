use m2_client::character_create_flow::{TPacketCGPlayerCreate, Job};

#[test]
fn test_packet_size() {
    assert_eq!(std::mem::size_of::<TPacketCGPlayerCreate>(), 73, "Packet size must be exactly 73 bytes");
}

#[test]
fn test_warrior_stats() {
    let pkt = TPacketCGPlayerCreate::new(0, "Hero", Job::WarriorM, 0);
    assert_eq!(pkt.stat_str, 6);
    assert_eq!(pkt.stat_dex, 3);
    assert_eq!(pkt.stat_vit, 4);
    assert_eq!(pkt.stat_int, 3);
}

#[test]
fn test_sura_stats() {
    let pkt = TPacketCGPlayerCreate::new(1, "SuraGuy", Job::SuraM, 1);
    assert_eq!(pkt.stat_str, 3);
    assert_eq!(pkt.stat_dex, 3);
    assert_eq!(pkt.stat_vit, 3);
    assert_eq!(pkt.stat_int, 7);
}

#[test]
fn test_packet_header_and_length() {
    let pkt = TPacketCGPlayerCreate::new(2, "TestName", Job::NinjaF, 0);
    // Unaligned access bypass
    let hdr = { pkt.header };
    let len = { pkt.length };
    assert_eq!(hdr, 0x0201);
    assert_eq!(len, 73);
    assert_eq!(pkt.index, 2);
}

#[test]
fn test_name_truncation() {
    let long_name = "a".repeat(100);
    let pkt = TPacketCGPlayerCreate::new(0, &long_name, Job::ShamanF, 0);
    
    for i in 0..60 {
        assert_eq!(pkt.name[i], b'a');
    }
    assert_eq!(pkt.name[60], 0);
}

#[test]
fn test_as_bytes() {
    let pkt = TPacketCGPlayerCreate::new(3, "Bytes", Job::WarriorF, 1);
    let bytes = pkt.as_bytes();
    assert_eq!(bytes.len(), 73);
    assert_eq!(bytes[0], 0x01);
    assert_eq!(bytes[1], 0x02);
}
