#![allow(unused_imports)]

use std::mem::size_of;

#[path = "../src/recv_guild_packets_complete.rs"]
mod recv_guild_packets_complete;

use recv_guild_packets_complete::*;

#[test]
fn test_parse_info_packet() {
    let mut payload = vec![0u8; size_of::<TPacketGCGuild>() + size_of::<TPacketGCGuildInfo>()];
    
    let header = unsafe { &mut *(payload.as_mut_ptr() as *mut TPacketGCGuild) };
    header.header = 0x0730; // GC_GUILD
    header.length = payload.len() as u16;
    header.subheader = guild_sub_gc::INFO;

    let info = unsafe { &mut *(payload[size_of::<TPacketGCGuild>()..].as_mut_ptr() as *mut TPacketGCGuildInfo) };
    info.guild_id = 12345;
    info.master_pid = 98765;
    info.member_count = 10;
    info.max_member_count = 50;
    info.exp = 100000;
    info.level = 5;
    info.gold = 500000;
    info.hasLand = 1;
    let name = b"MyGuild\0\0\0\0\0\0";
    info.name.copy_from_slice(name);

    match parse_guild_packet(&payload) {
        Ok(GuildEvent::Info {
            guild_id, master_pid, member_count, max_member_count, exp, level, name, gold, has_land
        }) => {
            assert_eq!(guild_id, 12345);
            assert_eq!(master_pid, 98765);
            assert_eq!(member_count, 10);
            assert_eq!(max_member_count, 50);
            assert_eq!(exp, 100000);
            assert_eq!(level, 5);
            assert_eq!(gold, 500000);
            assert_eq!(has_land, true);
            assert_eq!(name, "MyGuild");
        }
        _ => panic!("Expected Info event"),
    }
}

#[test]
fn test_parse_list_packet() {
    let packet_len = size_of::<TPacketGCGuild>() + size_of::<TPacketGCGuildSubMember>() + CHARACTER_NAME_MAX_LEN + 1;
    let mut payload = vec![0u8; packet_len];
    
    let header = unsafe { &mut *(payload.as_mut_ptr() as *mut TPacketGCGuild) };
    header.header = 0x0730;
    header.length = payload.len() as u16;
    header.subheader = guild_sub_gc::LIST;

    let member = unsafe { &mut *(payload[size_of::<TPacketGCGuild>()..].as_mut_ptr() as *mut TPacketGCGuildSubMember) };
    member.pid = 1111;
    member.byGrade = 1;
    member.byIsGeneral = 0;
    member.byJob = 2;
    member.byLevel = 99;
    member.dwOffer = 100;
    member.byNameFlag = 1;

    let name_offset = size_of::<TPacketGCGuild>() + size_of::<TPacketGCGuildSubMember>();
    let name_str = b"TestMember\0";
    payload[name_offset..name_offset + name_str.len()].copy_from_slice(name_str);

    match parse_guild_packet(&payload) {
        Ok(GuildEvent::List(members)) => {
            assert_eq!(members.len(), 1);
            let m = &members[0];
            assert_eq!(m.pid, 1111);
            assert_eq!(m.byGrade, 1);
            assert_eq!(m.byIsGeneral, 0);
            assert_eq!(m.byJob, 2);
            assert_eq!(m.byLevel, 99);
            assert_eq!(m.dwOffer, 100);
            assert_eq!(m.name.as_deref(), Some("TestMember"));
        }
        _ => panic!("Expected List event"),
    }
}

#[test]
fn test_parse_grade_packet() {
    let packet_len = size_of::<TPacketGCGuild>() + 1 + 1 + size_of::<TPacketGCGuildSubGrade>();
    let mut payload = vec![0u8; packet_len];
    
    let header = unsafe { &mut *(payload.as_mut_ptr() as *mut TPacketGCGuild) };
    header.header = 0x0730;
    header.length = payload.len() as u16;
    header.subheader = guild_sub_gc::GRADE;

    let payload_offset = size_of::<TPacketGCGuild>();
    payload[payload_offset] = 1; // count
    payload[payload_offset + 1] = 5; // index

    let grade = unsafe { &mut *(payload[payload_offset + 2..].as_mut_ptr() as *mut TPacketGCGuildSubGrade) };
    let grade_name = b"Leader\0\0\0";
    grade.grade_name.copy_from_slice(grade_name);
    grade.auth_flag = 0xFF;

    match parse_guild_packet(&payload) {
        Ok(GuildEvent::Grade(grades)) => {
            assert_eq!(grades.len(), 1);
            let g = &grades[0];
            assert_eq!(g.index, 5);
            assert_eq!(g.auth_flag, 0xFF);
            assert_eq!(g.grade_name, "Leader");
        }
        _ => panic!("Expected Grade event"),
    }
}

#[test]
fn test_parse_skill_info_packet() {
    let packet_len = size_of::<TPacketGCGuild>() + size_of::<TGuildSkillData>();
    let mut payload = vec![0u8; packet_len];
    
    let header = unsafe { &mut *(payload.as_mut_ptr() as *mut TPacketGCGuild) };
    header.header = 0x0730;
    header.length = payload.len() as u16;
    header.subheader = guild_sub_gc::SKILL_INFO;

    let skill = unsafe { &mut *(payload[size_of::<TPacketGCGuild>()..].as_mut_ptr() as *mut TGuildSkillData) };
    skill.bySkillPoint = 10;
    skill.bySkillLevel = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    skill.wGuildPoint = 1000;
    skill.wMaxGuildPoint = 5000;

    match parse_guild_packet(&payload) {
        Ok(GuildEvent::SkillInfo {
            skill_point, skill_levels, guild_point, max_guild_point
        }) => {
            assert_eq!(skill_point, 10);
            assert_eq!(skill_levels, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            assert_eq!(guild_point, 1000);
            assert_eq!(max_guild_point, 5000);
        }
        _ => panic!("Expected SkillInfo event"),
    }
}

#[test]
fn test_parse_war_packet() {
    let packet_len = size_of::<TPacketGCGuild>() + size_of::<TPacketGCGuildWar>();
    let mut payload = vec![0u8; packet_len];
    
    let header = unsafe { &mut *(payload.as_mut_ptr() as *mut TPacketGCGuild) };
    header.header = 0x0730;
    header.length = payload.len() as u16;
    header.subheader = guild_sub_gc::WAR;

    let war = unsafe { &mut *(payload[size_of::<TPacketGCGuild>()..].as_mut_ptr() as *mut TPacketGCGuildWar) };
    war.dwGuildSelf = 123;
    war.dwGuildOpp = 456;
    war.bType = 1;
    war.bWarState = 2;

    match parse_guild_packet(&payload) {
        Ok(GuildEvent::War {
            guild_self, guild_opp, war_type, war_state
        }) => {
            assert_eq!(guild_self, 123);
            assert_eq!(guild_opp, 456);
            assert_eq!(war_type, 1);
            assert_eq!(war_state, 2);
        }
        _ => panic!("Expected War event"),
    }
}

#[test]
fn test_invalid_packet_length() {
    let payload = vec![0u8; size_of::<TPacketGCGuild>() - 1];
    let res = parse_guild_packet(&payload);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Packet too short for TPacketGCGuild");
}
