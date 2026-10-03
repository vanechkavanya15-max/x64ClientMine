pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyAdd {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyUpdate {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub state: u8,
    pub percent_hp: u8,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyLink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyUnlink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPartyParameter {
    pub header: u16,
    pub length: u16,
    pub b_distribute_mode: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PartyPacketParseError {
    BufferTooSmall,
}

pub fn parse_packet_add(buffer: &[u8]) -> Result<TPacketGCPartyAdd, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyAdd>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    // Safety: we checked the buffer length. The structure is packed, so alignment doesn't matter.
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyAdd) };
    Ok(packet)
}

pub fn parse_packet_update(buffer: &[u8]) -> Result<TPacketGCPartyUpdate, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyUpdate>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyUpdate) };
    Ok(packet)
}

pub fn parse_packet_remove(buffer: &[u8]) -> Result<TPacketGCPartyRemove, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyRemove>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyRemove) };
    Ok(packet)
}

pub fn parse_packet_link(buffer: &[u8]) -> Result<TPacketGCPartyLink, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyLink>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyLink) };
    Ok(packet)
}

pub fn parse_packet_unlink(buffer: &[u8]) -> Result<TPacketGCPartyUnlink, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyUnlink>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyUnlink) };
    Ok(packet)
}

pub fn parse_packet_parameter(buffer: &[u8]) -> Result<TPacketGCPartyParameter, PartyPacketParseError> {
    if buffer.len() < std::mem::size_of::<TPacketGCPartyParameter>() {
        return Err(PartyPacketParseError::BufferTooSmall);
    }
    
    let packet = unsafe { std::ptr::read_unaligned(buffer.as_ptr() as *const TPacketGCPartyParameter) };
    Ok(packet)
}
