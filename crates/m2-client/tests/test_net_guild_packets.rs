use m2_client::net_guild_packets::*;

#[test]
fn test_guild_answer_make_guild_packet_valid_name() {
    let packet = TPacketCGAnswerMakeGuild::new("TestGuild").unwrap();
    let header = packet.header;
    let length = packet.length;
    assert_eq!(header, HEADER_CG_ANSWER_MAKE_GUILD);
    assert_eq!(length, std::mem::size_of::<TPacketCGAnswerMakeGuild>() as u16);
    
    let bytes = packet.to_bytes();
    assert_eq!(bytes.len(), std::mem::size_of::<TPacketCGAnswerMakeGuild>());
    
    // Check name string starts at offset 4 (header 2 + length 2)
    assert_eq!(&bytes[4..4+9], b"TestGuild");
}

#[test]
fn test_guild_answer_make_guild_packet_too_long() {
    // 13 chars, max is 12
    let packet_opt = TPacketCGAnswerMakeGuild::new("1234567890123");
    assert!(packet_opt.is_none());
}

#[test]
fn test_build_guild_change_member_grade_packet() {
    let bytes = build_guild_change_member_grade_packet(12345, 3).unwrap();
    
    // struct size (5) + pid (4) + grade (1) = 10 bytes
    assert_eq!(bytes.len(), 10);
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let sub_header = bytes[4];
    
    assert_eq!(header, HEADER_CG_GUILD);
    assert_eq!(length, 10);
    assert_eq!(sub_header, guild_sub::CHANGE_MEMBER_GRADE);
    
    let pid = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
    let grade = bytes[9];
    assert_eq!(pid, 12345);
    assert_eq!(grade, 3);
}

#[test]
fn test_build_guild_deposit_money_packet() {
    let bytes = build_guild_deposit_money_packet(50000).unwrap();
    
    // struct size (5) + money (4) = 9 bytes
    assert_eq!(bytes.len(), 9);
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let sub_header = bytes[4];
    
    assert_eq!(header, HEADER_CG_GUILD);
    assert_eq!(length, 9);
    assert_eq!(sub_header, guild_sub::DEPOSIT_MONEY);
    
    let money = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
    assert_eq!(money, 50000);
}

#[test]
fn test_build_guild_offer_packet() {
    let bytes = build_guild_offer_packet(1000).unwrap();
    
    // struct size (5) + exp (4) = 9 bytes
    assert_eq!(bytes.len(), 9);
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    let sub_header = bytes[4];
    
    assert_eq!(header, HEADER_CG_GUILD);
    assert_eq!(length, 9);
    assert_eq!(sub_header, guild_sub::OFFER);
    
    let exp = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
    assert_eq!(exp, 1000);
}

#[test]
fn test_build_guild_post_comment_packet_success() {
    let bytes = build_guild_post_comment_packet("Hello guild").unwrap();
    
    // "Hello guild\0" = 12 bytes. 
    // struct size (5) + size byte (1) + comment len (12) = 18 bytes
    assert_eq!(bytes.len(), 18);
    
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 18);
    
    assert_eq!(bytes[4], guild_sub::POST_COMMENT);
    assert_eq!(bytes[5], 12); // size byte
    
    assert_eq!(&bytes[6..17], b"Hello guild");
    assert_eq!(bytes[17], 0);
}

#[test]
fn test_build_guild_change_grade_name_packet() {
    let bytes = build_guild_change_grade_name_packet(1, "NewGrade").unwrap();
    
    // struct size (5) + grade_number (1) + name_len (9) = 15 bytes
    assert_eq!(bytes.len(), 15);
    
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 15);
    
    assert_eq!(bytes[4], guild_sub::CHANGE_GRADE_NAME);
    assert_eq!(bytes[5], 1); // grade number
    assert_eq!(&bytes[6..14], b"NewGrade");
}
