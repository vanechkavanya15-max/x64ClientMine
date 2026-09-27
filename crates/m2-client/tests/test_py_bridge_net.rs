use std::ffi::CString;
use m2_client::py_bridge_net::{
    m2_net_Connect, m2_net_Disconnect, m2_net_GetMainStream,
    m2_net_SendChatPacket, m2_net_SendAttackPacket, m2_net_SendUseSkillPacket, m2_net_SendMovePacket,
    NETWORK_STREAM
};
use m2_client::send_chat_packets_v2::CG_CHAT;
use m2_client::send_combat_packets_v2::{HEADER_CG_ATTACK, HEADER_CG_USE_SKILL};
use m2_client::send_movement_packets_v2::TPacketCGMove;

#[test]
fn test_m2_net_connect_disconnect() {
    // Reset state initially just in case
    m2_net_Disconnect();
    assert!(!m2_net_GetMainStream());
    
    let ip = CString::new("127.0.0.1").unwrap();
    let port = 50000;
    
    // Connect
    assert!(m2_net_Connect(ip.as_ptr(), port));
    assert!(m2_net_GetMainStream());
    
    // Verify internal state
    {
        let state = NETWORK_STREAM.lock().unwrap();
        assert!(state.is_online);
        assert_eq!(state.ip, "127.0.0.1");
        assert_eq!(state.port, 50000);
    }
    
    // Disconnect
    m2_net_Disconnect();
    assert!(!m2_net_GetMainStream());
    
    {
        let state = NETWORK_STREAM.lock().unwrap();
        assert!(!state.is_online);
        assert_eq!(state.ip, "");
        assert_eq!(state.port, 0);
    }
}

#[test]
fn test_m2_net_send_chat_packet() {
    let mut buf = [0u8; 1024];
    let chat_msg = CString::new("Hello World").unwrap();
    let chat_type = 1; // np. CHAT_TYPE_SHOUT
    
    let size = m2_net_SendChatPacket(chat_msg.as_ptr(), chat_type, buf.as_mut_ptr());
    
    // Size should be header size (5 bytes) + len("Hello World") (11) + 1 for null = 17?
    // Wait, let's verify actual size returned by the function.
    assert!(size > 0);
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let length = u16::from_le_bytes([buf[2], buf[3]]);
    let type_ = buf[4];
    
    assert_eq!(header, CG_CHAT);
    assert_eq!(length as usize, size);
    assert_eq!(type_, chat_type);
}

#[test]
fn test_m2_net_send_attack_packet() {
    let mut buf = [0u8; 1024];
    let attack_type = 2;
    let target_vid = 9999;
    
    let size = m2_net_SendAttackPacket(attack_type, target_vid, buf.as_mut_ptr());
    assert_eq!(size, std::mem::size_of::<m2_client::send_combat_packets_v2::TPacketCGAttack>());
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let b_type = buf[2];
    let victim_vid = u32::from_le_bytes([buf[3], buf[4], buf[5], buf[6]]);
    
    assert_eq!(header, HEADER_CG_ATTACK);
    assert_eq!(b_type, attack_type);
    assert_eq!(victim_vid, target_vid);
}

#[test]
fn test_m2_net_send_use_skill_packet() {
    let mut buf = [0u8; 1024];
    let skill_id = 1; // Three-way cut
    let target_vid = 8888;
    
    let size = m2_net_SendUseSkillPacket(skill_id, target_vid, buf.as_mut_ptr());
    assert_eq!(size, std::mem::size_of::<m2_client::send_combat_packets_v2::TPacketCGUseSkill>());
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let vnum = u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]);
    let vid = u32::from_le_bytes([buf[6], buf[7], buf[8], buf[9]]);
    
    assert_eq!(header, HEADER_CG_USE_SKILL);
    assert_eq!(vnum, skill_id);
    assert_eq!(vid, target_vid);
}

#[test]
fn test_m2_net_send_move_packet() {
    let mut buf = [0u8; 1024];
    let func = 1;
    let x = 1000;
    let y = 2000;
    let rot = 180.0;
    let time = 123456;
    
    let size = m2_net_SendMovePacket(func, x, y, rot, time, buf.as_mut_ptr());
    assert_eq!(size, TPacketCGMove::SIZE);
    
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    let length = u16::from_le_bytes([buf[2], buf[3]]);
    let f = buf[4];
    let b_rot = buf[6];
    
    assert_eq!(header, 0x0301); // CG_MOVE_HEADER
    assert_eq!(length as usize, TPacketCGMove::SIZE);
    assert_eq!(f, func);
    assert_eq!(b_rot, (180.0 / 5.0) as u8); // Normalization check
}

#[test]
fn test_m2_net_connect_null_ptr() {
    assert!(!m2_net_Connect(std::ptr::null(), 80));
}

#[test]
fn test_m2_net_send_chat_null_ptr() {
    let mut buf = [0u8; 1024];
    assert_eq!(m2_net_SendChatPacket(std::ptr::null(), 0, buf.as_mut_ptr()), 0);
    
    let chat_msg = CString::new("Hello").unwrap();
    assert_eq!(m2_net_SendChatPacket(chat_msg.as_ptr(), 0, std::ptr::null_mut()), 0);
}
