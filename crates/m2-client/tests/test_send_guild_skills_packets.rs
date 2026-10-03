#[path = "../src/send_guild_skills_packets.rs"]
mod send_guild_skills_packets;

use send_guild_skills_packets::*;

#[test]
fn test_guild_deposit_money_serialization() {
    let money = 1234567;
    let packet = serialize_guild_deposit_money(money);
    
    assert_eq!(packet.len(), 9);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 9u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::DEPOSIT_MONEY);
    assert_eq!(packet[5..9], money.to_le_bytes());
}

#[test]
fn test_guild_withdraw_money_serialization() {
    let money = 9876543;
    let packet = serialize_guild_withdraw_money(money);
    
    assert_eq!(packet.len(), 9);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 9u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::WITHDRAW_MONEY);
    assert_eq!(packet[5..9], money.to_le_bytes());
}

#[test]
fn test_guild_use_skill_serialization() {
    let skill_id = 42;
    let target_vid = 10005;
    let packet = serialize_guild_use_skill(skill_id, target_vid);
    
    assert_eq!(packet.len(), 13);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 13u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::USE_SKILL);
    assert_eq!(packet[5..9], skill_id.to_le_bytes());
    assert_eq!(packet[9..13], target_vid.to_le_bytes());
}

#[test]
fn test_guild_charge_gsp_serialization() {
    let money = 50000;
    let packet = serialize_guild_charge_gsp(money);
    
    assert_eq!(packet.len(), 9);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 9u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::CHARGE_GSP);
    assert_eq!(packet[5..9], money.to_le_bytes());
}

#[test]
fn test_guild_invite_answer_serialization() {
    let guild_id = 777;
    let answer = 1; // Accept
    let packet = serialize_guild_invite_answer(guild_id, answer);
    
    assert_eq!(packet.len(), 10);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 10u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::GUILD_INVITE_ANSWER);
    assert_eq!(packet[5..9], guild_id.to_le_bytes());
    assert_eq!(packet[9], answer);
}

#[test]
fn test_guild_invite_answer_decline() {
    let guild_id = 999;
    let answer = 0; // Decline
    let packet = serialize_guild_invite_answer(guild_id, answer);
    
    assert_eq!(packet.len(), 10);
    assert_eq!(packet[0..2], HEADER_CG_GUILD.to_le_bytes());
    assert_eq!(packet[2..4], 10u16.to_le_bytes());
    assert_eq!(packet[4], guild_sub::GUILD_INVITE_ANSWER);
    assert_eq!(packet[5..9], guild_id.to_le_bytes());
    assert_eq!(packet[9], answer);
}

#[test]
fn test_packet_cg_guild_struct_size_and_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGGuild>(), 5);
    
    let packet = TPacketCGGuild {
        header: HEADER_CG_GUILD,
        length: 5,
        by_sub_header: guild_sub::USE_SKILL,
    };
    
    assert_eq!({ packet.header }, 0x0720);
    assert_eq!({ packet.length }, 5);
    assert_eq!(packet.by_sub_header, 9);
}

#[test]
fn test_edge_case_zero_money() {
    let money = 0;
    let packet1 = serialize_guild_deposit_money(money);
    let packet2 = serialize_guild_withdraw_money(money);
    let packet3 = serialize_guild_charge_gsp(money);
    
    assert_eq!(packet1[5..9], [0, 0, 0, 0]);
    assert_eq!(packet2[5..9], [0, 0, 0, 0]);
    assert_eq!(packet3[5..9], [0, 0, 0, 0]);
}

#[test]
fn test_edge_case_max_money() {
    let money = u32::MAX;
    let packet = serialize_guild_deposit_money(money);
    
    assert_eq!(packet[5..9], [0xFF, 0xFF, 0xFF, 0xFF]);
}
