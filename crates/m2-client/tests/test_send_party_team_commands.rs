#[path = "../src/send_party_team_commands.rs"]
mod send_party_team_commands;

use send_party_team_commands::*;
use std::mem::size_of;

#[test]
fn test_sizes() {
    assert_eq!(size_of::<TPacketCGPartyInvite>(), 8); // u16 (2) + u16 (2) + u32 (4) = 8
    assert_eq!(size_of::<TPacketCGPartyInviteAnswer>(), 9); // u16 (2) + u16 (2) + u32 (4) + u8 (1) = 9
    assert_eq!(size_of::<TPacketCGPartyRemove>(), 8); // u16 (2) + u16 (2) + u32 (4) = 8
    assert_eq!(size_of::<TPacketCGPartySetState>(), 10); // u16 (2) + u16 (2) + u32 (4) + u8 (1) + u8 (1) = 10
}

#[test]
fn test_send_party_invite_packet() {
    let packet = PythonNetworkStreamCommand::send_party_invite_packet(123).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_INVITE);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartyInvite>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() }, 123);
}

#[test]
fn test_send_party_invite_answer_packet() {
    let packet = PythonNetworkStreamCommand::send_party_invite_answer_packet(456, 1).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_INVITE_ANSWER);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartyInviteAnswer>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.leader_pid).read_unaligned() }, 456);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.accept).read_unaligned() }, 1);
}

#[test]
fn test_send_party_remove_packet() {
    let packet = PythonNetworkStreamCommand::send_party_remove_packet(789).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_REMOVE);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartyRemove>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.pid).read_unaligned() }, 789);
}

#[test]
fn test_send_party_exit_packet() {
    let packet = PythonNetworkStreamCommand::send_party_exit_packet(999).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_REMOVE);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartyRemove>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.pid).read_unaligned() }, 999);
}

#[test]
fn test_send_party_set_state_packet() {
    let packet = PythonNetworkStreamCommand::send_party_set_state_packet(321, 2, 3).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_SET_STATE);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartySetState>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() }, 321);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.byState).read_unaligned() }, 2);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.byFlag).read_unaligned() }, 3);
}

#[test]
fn test_send_party_set_role() {
    let packet = PythonNetworkStreamCommand::send_party_set_role(333, 4, 5).unwrap();
    assert_eq!(unsafe { std::ptr::addr_of!(packet.header).read_unaligned() }, CG_PARTY_SET_STATE);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.length).read_unaligned() }, size_of::<TPacketCGPartySetState>() as u16);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() }, 333);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.byState).read_unaligned() }, 5);
    assert_eq!(unsafe { std::ptr::addr_of!(packet.byFlag).read_unaligned() }, 4);
}
