use m2_client::phase_select::{
    PhaseSelect, TSimplePlayerInformation, CHARACTER_NAME_MAX_LEN,
    HEADER_CG_CHANGE_NAME, HEADER_CG_EMPIRE, HEADER_CG_PLAYER_CREATE,
    HEADER_CG_PLAYER_DESTROY, HEADER_CG_PLAYER_SELECT, PRIVATE_CODE_LENGTH,
};

#[test]
fn test_integration_phase_select() {
    let instance = PhaseSelect::new();
    assert!(instance.is_ready());
}

#[test]
fn test_send_select_empire_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.send_select_empire_packet(1));
    assert_eq!(phase.get_empire_id(), 1);
    assert_eq!(phase.sent_packets.len(), 1);
    assert_eq!(phase.sent_packets[0], vec![HEADER_CG_EMPIRE, 1]);
}

#[test]
fn test_send_select_character_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.send_select_character_packet(2));
    assert_eq!(phase.sent_packets.len(), 1);
    assert_eq!(phase.sent_packets[0], vec![HEADER_CG_PLAYER_SELECT, 2]);
}

#[test]
fn test_send_destroy_character_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.send_destroy_character_packet(1, "1234567")); // max is 7 chars because size is 8
    assert_eq!(phase.sent_packets.len(), 1);
    
    let mut expected_packet = vec![HEADER_CG_PLAYER_DESTROY, 1];
    let mut code_bytes = [0u8; PRIVATE_CODE_LENGTH];
    code_bytes[..7].copy_from_slice(b"1234567");
    expected_packet.extend_from_slice(&code_bytes);
    
    assert_eq!(phase.sent_packets[0], expected_packet);
}

#[test]
fn test_send_create_character_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.send_create_character_packet(0, "TestHero", 1, 0, 10, 20, 30, 40));
    assert_eq!(phase.sent_packets.len(), 1);
    
    let mut expected_packet = vec![HEADER_CG_PLAYER_CREATE, 0];
    let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_str = b"TestHero";
    name_bytes[..name_str.len()].copy_from_slice(name_str);
    expected_packet.extend_from_slice(&name_bytes);
    expected_packet.extend_from_slice(&(1u16.to_le_bytes()));
    expected_packet.push(0);
    expected_packet.push(10);
    expected_packet.push(20);
    expected_packet.push(30);
    expected_packet.push(40);
    
    assert_eq!(phase.sent_packets[0], expected_packet);
}

#[test]
fn test_send_change_name_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.send_change_name_packet(0, "NewHero"));
    assert_eq!(phase.sent_packets.len(), 1);
    
    let mut expected_packet = vec![HEADER_CG_CHANGE_NAME, 0];
    let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_str = b"NewHero";
    name_bytes[..name_str.len()].copy_from_slice(name_str);
    expected_packet.extend_from_slice(&name_bytes);
    
    assert_eq!(phase.sent_packets[0], expected_packet);
}

#[test]
fn test_recv_player_create_success_packet() {
    let mut phase = PhaseSelect::new();
    let mut info = TSimplePlayerInformation::default();
    info.dw_id = 999;
    
    assert!(phase.recv_player_create_success_packet(2, info));
    assert!(phase.on_create_success_called);
    
    let dw_id = unsafe { std::ptr::addr_of!(phase.simple_player_info[2].dw_id).read_unaligned() };
    assert_eq!(dw_id, 999);
}

#[test]
fn test_recv_player_create_success_packet_out_of_range() {
    let mut phase = PhaseSelect::new();
    let info = TSimplePlayerInformation::default();
    
    assert!(phase.recv_player_create_success_packet(4, info));
    assert!(!phase.on_create_success_called); // returns true but does not update state
}

#[test]
fn test_recv_player_create_failure_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.recv_player_create_failure_packet(2));
    assert_eq!(phase.on_create_failure_called, Some(2));
}

#[test]
fn test_recv_player_destroy_success_packet() {
    let mut phase = PhaseSelect::new();
    let mut info = TSimplePlayerInformation::default();
    info.dw_id = 123;
    phase.simple_player_info[1] = info;
    phase.guild_id[1] = 456;
    phase.guild_name[1] = "Guild".to_string();
    
    assert!(phase.recv_player_destroy_success_packet(1));
    assert_eq!(phase.on_delete_success_called, Some(1));
    
    let dw_id = unsafe { std::ptr::addr_of!(phase.simple_player_info[1].dw_id).read_unaligned() };
    assert_eq!(dw_id, 0);
    assert_eq!(phase.guild_id[1], 0);
    assert_eq!(phase.guild_name[1], String::new());
}

#[test]
fn test_recv_player_destroy_success_packet_out_of_range() {
    let mut phase = PhaseSelect::new();
    assert!(!phase.recv_player_destroy_success_packet(4)); // Should return false
}

#[test]
fn test_recv_player_destroy_failure_packet() {
    let mut phase = PhaseSelect::new();
    assert!(phase.recv_player_destroy_failure_packet());
    assert!(phase.on_delete_failure_called);
}

#[test]
fn test_recv_change_name_success() {
    let mut phase = PhaseSelect::new();
    let mut info = TSimplePlayerInformation::default();
    info.dw_id = 555;
    info.b_change_name = 1;
    phase.simple_player_info[2] = info;
    
    assert!(phase.recv_change_name(555, "ChangedName"));
    assert_eq!(phase.on_change_name_called, Some((2, "ChangedName".to_string())));
    
    let b_change_name = unsafe { std::ptr::addr_of!(phase.simple_player_info[2].b_change_name).read_unaligned() };
    assert_eq!(b_change_name, 0);
    
    let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    expected_name[..11].copy_from_slice(b"ChangedName");
    
    let sz_name = unsafe { std::ptr::addr_of!(phase.simple_player_info[2].sz_name).read_unaligned() };
    assert_eq!(sz_name, expected_name);
}

#[test]
fn test_recv_change_name_failure_wrong_pid() {
    let mut phase = PhaseSelect::new();
    let mut info = TSimplePlayerInformation::default();
    info.dw_id = 555;
    phase.simple_player_info[2] = info;
    
    assert!(phase.recv_change_name(111, "ChangedName"));
    assert_eq!(phase.on_create_failure_called, Some(100)); // triggers on_create_failure with 100
}
