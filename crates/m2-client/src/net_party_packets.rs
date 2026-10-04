pub const HEADER_CG_PARTY_INVITE: u16 = 0x0701;
pub const HEADER_CG_PARTY_INVITE_ANSWER: u16 = 0x0702;
pub const HEADER_CG_PARTY_REMOVE: u16 = 0x0703;
pub const HEADER_CG_PARTY_SET_STATE: u16 = 0x0704;
pub const HEADER_CG_PARTY_USE_SKILL: u16 = 0x0705;
pub const HEADER_CG_PARTY_PARAMETER: u16 = 0x0706;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInvite {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyInviteAnswer {
    pub header: u16,
    pub length: u16,
    pub leader_pid: u32,
    pub accept: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartySetState {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
    pub by_state: u8,
    pub by_flag: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyUseSkill {
    pub header: u16,
    pub length: u16,
    pub by_skill_index: u8,
    pub dw_target_vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPartyParameter {
    pub header: u16,
    pub length: u16,
    pub b_distribute_mode: u8,
}

pub fn serialize_party_invite(vid: u32) -> [u8; std::mem::size_of::<TPacketCGPartyInvite>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartyInvite>()];
    let packet = TPacketCGPartyInvite {
        header: HEADER_CG_PARTY_INVITE,
        length: std::mem::size_of::<TPacketCGPartyInvite>() as u16,
        vid,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartyInvite>(),
        );
    }
    buf
}

pub fn serialize_party_invite_answer(leader_pid: u32, accept: u8) -> [u8; std::mem::size_of::<TPacketCGPartyInviteAnswer>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartyInviteAnswer>()];
    let packet = TPacketCGPartyInviteAnswer {
        header: HEADER_CG_PARTY_INVITE_ANSWER,
        length: std::mem::size_of::<TPacketCGPartyInviteAnswer>() as u16,
        leader_pid,
        accept,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartyInviteAnswer>(),
        );
    }
    buf
}

pub fn serialize_party_remove(pid: u32) -> [u8; std::mem::size_of::<TPacketCGPartyRemove>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartyRemove>()];
    let packet = TPacketCGPartyRemove {
        header: HEADER_CG_PARTY_REMOVE,
        length: std::mem::size_of::<TPacketCGPartyRemove>() as u16,
        pid,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartyRemove>(),
        );
    }
    buf
}

pub fn serialize_party_set_state(dw_vid: u32, by_state: u8, by_flag: u8) -> [u8; std::mem::size_of::<TPacketCGPartySetState>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartySetState>()];
    let packet = TPacketCGPartySetState {
        header: HEADER_CG_PARTY_SET_STATE,
        length: std::mem::size_of::<TPacketCGPartySetState>() as u16,
        dw_vid,
        by_state,
        by_flag,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartySetState>(),
        );
    }
    buf
}

pub fn serialize_party_use_skill(by_skill_index: u8, dw_target_vid: u32) -> [u8; std::mem::size_of::<TPacketCGPartyUseSkill>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartyUseSkill>()];
    let packet = TPacketCGPartyUseSkill {
        header: HEADER_CG_PARTY_USE_SKILL,
        length: std::mem::size_of::<TPacketCGPartyUseSkill>() as u16,
        by_skill_index,
        dw_target_vid,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartyUseSkill>(),
        );
    }
    buf
}

pub fn serialize_party_parameter(b_distribute_mode: u8) -> [u8; std::mem::size_of::<TPacketCGPartyParameter>()] {
    let mut buf = [0u8; std::mem::size_of::<TPacketCGPartyParameter>()];
    let packet = TPacketCGPartyParameter {
        header: HEADER_CG_PARTY_PARAMETER,
        length: std::mem::size_of::<TPacketCGPartyParameter>() as u16,
        b_distribute_mode,
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            buf.as_mut_ptr(),
            std::mem::size_of::<TPacketCGPartyParameter>(),
        );
    }
    buf
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_invite_packet(vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_invite(vid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_invite_answer_packet(leader_pid: u32, accept: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_invite_answer(leader_pid, accept);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_remove_packet(pid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_remove(pid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_set_state_packet(dw_vid: u32, by_state: u8, by_flag: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_set_state(dw_vid, by_state, by_flag);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_use_skill_packet(by_skill_index: u8, dw_target_vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_use_skill(by_skill_index, dw_target_vid);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}

#[no_mangle]
pub unsafe extern "C" fn m2_send_party_parameter_packet(b_distribute_mode: u8, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }
    let data = serialize_party_parameter(b_distribute_mode);
    std::ptr::copy_nonoverlapping(data.as_ptr(), out_buf, data.len());
    data.len()
}
