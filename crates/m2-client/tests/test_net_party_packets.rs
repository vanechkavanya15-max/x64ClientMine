#[path = "../src/net_party_packets.rs"]
mod net_party_packets;

use net_party_packets::*;
use std::mem::size_of;

#[test]
fn test_serialize_party_invite() {
    let vid = 12345678;
    let data = serialize_party_invite(vid);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartyInvite>());
    
    let packet: TPacketCGPartyInvite = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartyInvite) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_INVITE);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartyInvite>() as u16);
    assert_eq!({ packet.vid }, vid);
}

#[test]
fn test_serialize_party_invite_answer() {
    let leader_pid = 87654321;
    let accept = 1;
    let data = serialize_party_invite_answer(leader_pid, accept);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartyInviteAnswer>());
    
    let packet: TPacketCGPartyInviteAnswer = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartyInviteAnswer) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_INVITE_ANSWER);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartyInviteAnswer>() as u16);
    assert_eq!({ packet.leader_pid }, leader_pid);
    assert_eq!({ packet.accept }, accept);
}

#[test]
fn test_serialize_party_remove() {
    let pid = 11223344;
    let data = serialize_party_remove(pid);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartyRemove>());
    
    let packet: TPacketCGPartyRemove = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartyRemove) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_REMOVE);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartyRemove>() as u16);
    assert_eq!({ packet.pid }, pid);
}

#[test]
fn test_serialize_party_set_state() {
    let dw_vid = 99887766;
    let by_state = 2;
    let by_flag = 3;
    let data = serialize_party_set_state(dw_vid, by_state, by_flag);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartySetState>());
    
    let packet: TPacketCGPartySetState = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartySetState) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_SET_STATE);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartySetState>() as u16);
    assert_eq!({ packet.dw_vid }, dw_vid);
    assert_eq!({ packet.by_state }, by_state);
    assert_eq!({ packet.by_flag }, by_flag);
}

#[test]
fn test_serialize_party_use_skill() {
    let by_skill_index = 5;
    let dw_target_vid = 55443322;
    let data = serialize_party_use_skill(by_skill_index, dw_target_vid);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartyUseSkill>());
    
    let packet: TPacketCGPartyUseSkill = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartyUseSkill) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_USE_SKILL);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartyUseSkill>() as u16);
    assert_eq!({ packet.by_skill_index }, by_skill_index);
    assert_eq!({ packet.dw_target_vid }, dw_target_vid);
}

#[test]
fn test_serialize_party_parameter() {
    let b_distribute_mode = 1;
    let data = serialize_party_parameter(b_distribute_mode);
    
    assert_eq!(data.len(), size_of::<TPacketCGPartyParameter>());
    
    let packet: TPacketCGPartyParameter = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketCGPartyParameter) };
    assert_eq!({ packet.header }, HEADER_CG_PARTY_PARAMETER);
    assert_eq!({ packet.length }, size_of::<TPacketCGPartyParameter>() as u16);
    assert_eq!({ packet.b_distribute_mode }, b_distribute_mode);
}
