#[path = "../src/recv_guild_clan_sync.rs"]
mod recv_guild_clan_sync;

use recv_guild_clan_sync::*;
use std::io::Cursor;

fn create_guild_packet(subheader: u8, payload: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    let header = TPacketGCGuild {
        header: 0xDE, // Dummy header
        length: (std::mem::size_of::<TPacketGCGuild>() + payload.len()) as u16,
        subheader,
    };
    let header_bytes: [u8; std::mem::size_of::<TPacketGCGuild>()] = unsafe { std::mem::transmute(header) };
    data.extend_from_slice(&header_bytes);
    data.extend_from_slice(payload);
    data
}

#[test]
fn test_recv_guild_sub_login() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());
    
    // Add member
    sync.members.insert(101, GuildMemberData {
        pid: 101,
        name: "OtherPlayer".to_string(),
        grade: 1,
        job: 1,
        level: 50,
        offer: 0,
        is_general: 0,
    });

    let payload = 101u32.to_le_bytes();
    let packet_data = create_guild_packet(guild_sub_gc::LOGIN, &payload);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result);
    assert_eq!(sync.messenger_login_events.len(), 1);
    assert_eq!(sync.messenger_login_events[0], "OtherPlayer");
}

#[test]
fn test_recv_guild_sub_remove_main_player() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());
    
    sync.members.insert(100, GuildMemberData {
        pid: 100,
        name: "MainPlayer".to_string(),
        grade: 1,
        job: 1,
        level: 50,
        offer: 0,
        is_general: 0,
    });
    sync.info.guild_name = "TestGuild".to_string();

    let payload = 100u32.to_le_bytes();
    let packet_data = create_guild_packet(guild_sub_gc::REMOVE, &payload);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result);
    assert!(sync.members.is_empty());
    assert!(sync.info.guild_name.is_empty());
    assert!(sync.ui_events.contains(&"DeleteGuild".to_string()));
}

#[test]
fn test_recv_guild_sub_remove_other_player() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());
    
    sync.members.insert(101, GuildMemberData {
        pid: 101,
        name: "OtherPlayer".to_string(),
        grade: 1,
        job: 1,
        level: 50,
        offer: 0,
        is_general: 0,
    });

    let payload = 101u32.to_le_bytes();
    let packet_data = create_guild_packet(guild_sub_gc::REMOVE, &payload);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result);
    assert!(!sync.members.contains_key(&101));
    assert_eq!(sync.messenger_logout_events[0], "OtherPlayer");
    assert!(sync.ui_events.contains(&"RefreshGuildWindowMemberPage".to_string()));
}

#[test]
fn test_recv_guild_sub_info() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());

    let mut info_pack = TPacketGCGuildInfo {
        member_count: 5,
        max_member_count: 50,
        guild_id: 10,
        master_pid: 200,
        exp: 5000,
        level: 15,
        name: [0u8; GUILD_NAME_MAX_LEN + 1],
        gold: 100000,
        has_land: 1,
    };
    let name_str = b"EpicGuild\0";
    info_pack.name[..name_str.len()].copy_from_slice(name_str);

    let payload: [u8; std::mem::size_of::<TPacketGCGuildInfo>()] = unsafe { std::mem::transmute(info_pack) };
    let packet_data = create_guild_packet(guild_sub_gc::INFO, &payload);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result);
    assert_eq!(sync.info.guild_id, 10);
    assert_eq!(sync.info.guild_name, "EpicGuild");
    assert_eq!(sync.info.has_land, 1);
    assert_eq!(sync.info.guild_money, 100000);
    assert!(sync.ui_events.contains(&"RefreshGuildWindowInfoPage".to_string()));
}

#[test]
fn test_recv_guild_sub_war_list() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());

    let mut payload = Vec::new();
    payload.extend_from_slice(&10u32.to_le_bytes()); // src
    payload.extend_from_slice(&20u32.to_le_bytes()); // dst
    payload.extend_from_slice(&30u32.to_le_bytes()); // src
    payload.extend_from_slice(&40u32.to_le_bytes()); // dst

    let packet_data = create_guild_packet(guild_sub_gc::GUILD_WAR_LIST, &payload);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result);
    assert!(sync.gvg_keys.contains(&(10, 20)));
    assert!(sync.gvg_keys.contains(&(30, 40)));
    assert_eq!(sync.gvg_keys.len(), 2);
}

#[test]
fn test_recv_guild_sub_invalid_subheader() {
    let mut sync = GuildClanSync::new(100, "MainPlayer".to_string());

    let packet_data = create_guild_packet(255, &[]);
    
    let mut cursor = Cursor::new(packet_data);
    let result = sync.recv_guild(&mut cursor).unwrap();
    
    assert!(result); // Returns true (ignores unknown subheader)
}
