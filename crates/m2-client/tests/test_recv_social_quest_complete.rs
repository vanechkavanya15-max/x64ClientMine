use m2_client::recv_social_quest_complete::*;

#[test]
fn test_party_packets() {
    let invite = TPacketGCPartyInvite {
        header: 0x0710,
        length: std::mem::size_of::<TPacketGCPartyInvite>() as u16,
        leader_pid: 12345,
    };
    assert_eq!({invite.leader_pid}, 12345);

    let mut add = TPacketGCPartyAdd {
        header: 0x0711,
        length: std::mem::size_of::<TPacketGCPartyAdd>() as u16,
        pid: 67890,
        name: [0; CHARACTER_NAME_MAX_LEN + 1],
    };
    let name_str = b"Warrior123";
    add.name[..name_str.len()].copy_from_slice(name_str);
    assert_eq!({add.pid}, 67890);
    assert_eq!({add.name[0]}, b'W');

    let update = TPacketGCPartyUpdate {
        header: 0x0712,
        length: std::mem::size_of::<TPacketGCPartyUpdate>() as u16,
        pid: 67890,
        state: 1,
        percent_hp: 85,
        affects: [0; PARTY_AFFECT_SLOT_MAX_NUM],
    };
    assert_eq!({update.percent_hp}, 85);
}

#[test]
fn test_guild_packets() {
    let mut info = TPacketGCGuildInfo {
        member_count: 10,
        max_member_count: 50,
        guild_id: 1,
        master_pid: 999,
        exp: 10000,
        level: 5,
        name: [0; GUILD_NAME_MAX_LEN + 1],
        gold: 50000,
        hasLand: 0,
    };
    let guild_name = b"MyGuild";
    info.name[..guild_name.len()].copy_from_slice(guild_name);
    assert_eq!({info.guild_id}, 1);
    assert_eq!({info.name[0]}, b'M');
}

#[test]
fn test_messenger_packets() {
    let list = TPacketGCMessengerListOnline {
        connected: 1,
        length: 10,
    };
    assert_eq!(list.connected, 1);
    assert_eq!(list.length, 10);
}

#[test]
fn test_quest_confirm() {
    let mut pkt = TPacketGCQuestConfirm {
        header: 0x0911,
        length: std::mem::size_of::<TPacketGCQuestConfirm>() as u16,
        msg: [0; 65],
        timeout: 30,
        requestPID: 123,
    };
    let msg = b"Do you want to enter the dungeon?";
    pkt.msg[..msg.len()].copy_from_slice(msg);

    let data = unsafe {
        std::slice::from_raw_parts(
            &pkt as *const _ as *const u8,
            std::mem::size_of::<TPacketGCQuestConfirm>(),
        )
    };

    let result = parse_quest_confirm(data).unwrap();
    assert_eq!(result.0, "Do you want to enter the dungeon?");
    assert_eq!(result.1, 30);
    assert_eq!(result.2, 123);
}

#[test]
fn test_quest_info_flag() {
    let flag = QUEST_SEND_IS_BEGIN | QUEST_SEND_TITLE | QUEST_SEND_CLOCK_VALUE;
    let mut data = vec![];
    data.push(1); // is_begin
    let mut title = [0u8; 31];
    let title_str = b"Kill 10 Boars";
    title[..title_str.len()].copy_from_slice(title_str);
    data.extend_from_slice(&title);
    let clock_val: i32 = 600;
    data.extend_from_slice(&clock_val.to_le_bytes());

    let result = parse_quest_info_flag(flag, &data).unwrap();
    assert_eq!(result.get("is_begin").unwrap(), "1");
    assert_eq!(result.get("title").unwrap(), "Kill 10 Boars");
    assert_eq!(result.get("clock_value").unwrap(), "600");
}

#[test]
fn test_script_parse() {
    let pkt = TPacketGCScript {
        header: 0x0910,
        length: (std::mem::size_of::<TPacketGCScript>() + 12) as u16,
        skin: 1,
        src_size: 11,
    };
    
    let mut data = vec![];
    data.extend_from_slice(unsafe {
        std::slice::from_raw_parts(
            &pkt as *const _ as *const u8,
            std::mem::size_of::<TPacketGCScript>(),
        )
    });
    let script_content = b"Hello World\0";
    data.extend_from_slice(script_content);

    let result = parse_script(&data).unwrap();
    assert_eq!(result.0, 1);
    assert_eq!(result.1, "Hello World");
}
