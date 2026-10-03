// Testy dla pakietow administracyjnych gildii.

#[path = "../src/send_guild_admin_packets.rs"]
mod send_guild_admin_packets;

use send_guild_admin_packets::*;

#[test]
fn test_guild_add_member() {
    let vid = 12345;
    let packet = build_guild_add_member_packet(vid).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>();
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::ADD_MEMBER);
    
    let parsed_vid = u32::from_ne_bytes(body.try_into().unwrap());
    assert_eq!(parsed_vid, vid);
}

#[test]
fn test_guild_remove_member() {
    let pid = 54321;
    let packet = build_guild_remove_member_packet(pid).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>();
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::REMOVE_MEMBER);
    
    let parsed_pid = u32::from_ne_bytes(body.try_into().unwrap());
    assert_eq!(parsed_pid, pid);
}

#[test]
fn test_guild_change_grade_name_normal() {
    let grade = 3;
    let name = "Officer";
    let packet = build_guild_change_grade_name_packet(grade, name).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u8>() + GUILD_GRADE_NAME_MAX_LEN + 1;
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::CHANGE_GRADE_NAME);
    
    assert_eq!(body[0], grade);
    
    let parsed_name = std::ffi::CStr::from_bytes_until_nul(&body[1..]).unwrap().to_str().unwrap();
    assert_eq!(parsed_name, name);
}

#[test]
fn test_guild_change_grade_name_truncation() {
    let grade = 2;
    let name = "VeryLongNameIndeed"; // Powinno byc przyciete do 8 bajtow + nul
    let packet = build_guild_change_grade_name_packet(grade, name).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u8>() + GUILD_GRADE_NAME_MAX_LEN + 1;
    assert_eq!(packet.len(), expected_len);
    
    let parsed_name = std::ffi::CStr::from_bytes_until_nul(&packet[std::mem::size_of::<TPacketCGGuild>() + 1..]).unwrap().to_str().unwrap();
    assert_eq!(parsed_name, &name[..8]);
}

#[test]
fn test_guild_change_grade_authority() {
    let grade = 5;
    let auth = 0x0F;
    let packet = build_guild_change_grade_authority_packet(grade, auth).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + 2;
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::CHANGE_GRADE_AUTHORITY);
    
    assert_eq!(body[0], grade);
    assert_eq!(body[1], auth);
}

#[test]
fn test_guild_post_comment_normal() {
    let message = "Hello guild!";
    let packet = build_guild_post_comment_packet(message).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + 1 + message.len() + 1;
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::POST_COMMENT);
    
    let size = body[0] as usize;
    assert_eq!(size, message.len() + 1);
    
    let parsed_msg = std::ffi::CStr::from_bytes_with_nul(&body[1..]).unwrap().to_str().unwrap();
    assert_eq!(parsed_msg, message);
}

#[test]
fn test_guild_post_comment_too_long() {
    let message = "A".repeat(300);
    let result = build_guild_post_comment_packet(&message);
    assert_eq!(result.unwrap_err(), GuildPacketError::CommentTooLong);
}

#[test]
fn test_guild_delete_comment() {
    let index = 42;
    let packet = build_guild_delete_comment_packet(index).unwrap();
    
    let expected_len = std::mem::size_of::<TPacketCGGuild>() + std::mem::size_of::<u32>();
    assert_eq!(packet.len(), expected_len);
    
    let (head, body) = packet.split_at(std::mem::size_of::<TPacketCGGuild>());
    let header = unsafe { std::ptr::read_unaligned(head.as_ptr() as *const TPacketCGGuild) };
    
    assert_eq!({ header.header }, HEADER_CG_GUILD);
    assert_eq!({ header.length }, expected_len as u16);
    assert_eq!({ header.by_sub_header }, guild_sub::DELETE_COMMENT);
    
    let parsed_index = u32::from_ne_bytes(body.try_into().unwrap());
    assert_eq!(parsed_index, index);
}
