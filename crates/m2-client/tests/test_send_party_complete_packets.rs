use m2_client::send_party_complete_packets::*;

#[test]
fn test_cg_party_invite_struct_size_and_fields() {
    let packet = TPacketCGPartyInvite::new(12345);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartyInvite>(), 8);
    assert_eq!({ packet.header }, CG_PARTY_INVITE);
    assert_eq!({ packet.length }, 8);
    assert_eq!({ packet.vid }, 12345);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInvite, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInvite, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInvite, vid), 4);
}

#[test]
fn test_cg_party_invite_answer_struct_size_and_fields() {
    let packet = TPacketCGPartyInviteAnswer::new(54321, 1);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartyInviteAnswer>(), 9);
    assert_eq!({ packet.header }, CG_PARTY_INVITE_ANSWER);
    assert_eq!({ packet.length }, 9);
    assert_eq!({ packet.leader_pid }, 54321);
    assert_eq!({ packet.accept }, 1);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInviteAnswer, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInviteAnswer, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInviteAnswer, leader_pid), 4);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyInviteAnswer, accept), 8);
}

#[test]
fn test_cg_party_remove_struct_size_and_fields() {
    let packet = TPacketCGPartyRemove::new(999);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartyRemove>(), 8);
    assert_eq!({ packet.header }, CG_PARTY_REMOVE);
    assert_eq!({ packet.length }, 8);
    assert_eq!({ packet.pid }, 999);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartyRemove, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyRemove, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyRemove, pid), 4);
}

#[test]
fn test_cg_party_set_state_struct_size_and_fields() {
    let packet = TPacketCGPartySetState::new(123, 2, 4);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartySetState>(), 10);
    assert_eq!({ packet.header }, CG_PARTY_SET_STATE);
    assert_eq!({ packet.length }, 10);
    assert_eq!({ packet.dw_vid }, 123);
    assert_eq!({ packet.by_state }, 2);
    assert_eq!({ packet.by_flag }, 4);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartySetState, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartySetState, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartySetState, dw_vid), 4);
    assert_eq!(std::mem::offset_of!(TPacketCGPartySetState, by_state), 8);
    assert_eq!(std::mem::offset_of!(TPacketCGPartySetState, by_flag), 9);
}

#[test]
fn test_cg_party_use_skill_struct_size_and_fields() {
    let packet = TPacketCGPartyUseSkill::new(7, 333);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartyUseSkill>(), 9);
    assert_eq!({ packet.header }, CG_PARTY_USE_SKILL);
    assert_eq!({ packet.length }, 9);
    assert_eq!({ packet.by_skill_index }, 7);
    assert_eq!({ packet.dw_target_vid }, 333);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartyUseSkill, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyUseSkill, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyUseSkill, by_skill_index), 4);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyUseSkill, dw_target_vid), 5);
}

#[test]
fn test_cg_party_parameter_struct_size_and_fields() {
    let packet = TPacketCGPartyParameter::new(1);
    
    assert_eq!(std::mem::size_of::<TPacketCGPartyParameter>(), 5);
    assert_eq!({ packet.header }, CG_PARTY_PARAMETER);
    assert_eq!({ packet.length }, 5);
    assert_eq!({ packet.b_distribute_mode }, 1);
    
    assert_eq!(std::mem::offset_of!(TPacketCGPartyParameter, header), 0);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyParameter, length), 2);
    assert_eq!(std::mem::offset_of!(TPacketCGPartyParameter, b_distribute_mode), 4);
}
