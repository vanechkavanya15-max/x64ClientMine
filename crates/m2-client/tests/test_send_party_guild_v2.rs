// test_send_party_guild_v2.rs
use m2_client::send_party_guild_v2::*;

#[test]
fn test_party_invite_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGPartyInvite>(), 8);
    let bytes = serialize_party_invite(12345);
    assert_eq!(bytes.len(), 8);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PARTY_INVITE);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 8);
    assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 12345);
}

#[test]
fn test_party_invite_answer_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGPartyInviteAnswer>(), 9);
    let bytes = serialize_party_invite_answer(99999, 1);
    assert_eq!(bytes.len(), 9);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PARTY_INVITE_ANSWER);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 9);
    assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 99999);
    assert_eq!(bytes[8], 1);
}

#[test]
fn test_party_remove_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGPartyRemove>(), 8);
    let bytes = serialize_party_remove(4321);
    assert_eq!(bytes.len(), 8);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PARTY_REMOVE);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 8);
    assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 4321);
}

#[test]
fn test_party_set_state_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGPartySetState>(), 10);
    let bytes = serialize_party_set_state(1001, 2, 1);
    assert_eq!(bytes.len(), 10);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PARTY_SET_STATE);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 10);
    assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 1001);
    assert_eq!(bytes[8], 2);
    assert_eq!(bytes[9], 1);
}

#[test]
fn test_party_use_skill_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGPartyUseSkill>(), 9);
    let bytes = serialize_party_use_skill(3, 5005);
    assert_eq!(bytes.len(), 9);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PARTY_USE_SKILL);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 9);
    assert_eq!(bytes[4], 3);
    assert_eq!(u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]), 5005);
}

#[test]
fn test_guild_add_member_packet_layout() {
    let bytes = serialize_guild_add_member(8888);
    assert_eq!(bytes.len(), 9);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_GUILD);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 9);
    assert_eq!(bytes[4], guild_sub::ADD_MEMBER);
    assert_eq!(u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]), 8888);
}

#[test]
fn test_guild_remove_member_packet_layout() {
    let bytes = serialize_guild_remove_member(7777);
    assert_eq!(bytes.len(), 9);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_GUILD);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 9);
    assert_eq!(bytes[4], guild_sub::REMOVE_MEMBER);
    assert_eq!(u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]), 7777);
}

#[test]
fn test_guild_war_declare_packet_layout() {
    assert_eq!(std::mem::size_of::<TPacketCGGuildWarDeclare>(), 10);
    let bytes = serialize_guild_war_declare(500, 2);
    assert_eq!(bytes.len(), 10);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_GUILD);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 10);
    assert_eq!(bytes[4], guild_sub::WAR_DECLARE);
    assert_eq!(u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]), 500);
    assert_eq!(bytes[9], 2);
}

#[test]
fn test_ffi_exports() {
    let mut buf = [0u8; 32];
    unsafe {
        let len = m2_send_party_invite(1234, buf.as_mut_ptr());
        assert_eq!(len, 8);
        assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), 1234);

        let len2 = m2_send_guild_war_declare(600, 1, buf.as_mut_ptr());
        assert_eq!(len2, 10);
        assert_eq!(buf[4], guild_sub::WAR_DECLARE);
        assert_eq!(u32::from_le_bytes([buf[5], buf[6], buf[7], buf[8]]), 600);
        assert_eq!(buf[9], 1);
    }
}
