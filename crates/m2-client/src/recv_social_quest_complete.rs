use std::collections::HashMap;

// Party
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyInvite {
    pub header: u16,
    pub length: u16,
    pub leader_pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyAdd {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyUpdate {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub state: u8,
    pub percent_hp: u8,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyRemove {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyLink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyUnlink {
    pub header: u16,
    pub length: u16,
    pub pid: u32,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPartyParameter {
    pub header: u16,
    pub length: u16,
    pub bDistributeMode: u8,
}

// Guild
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;
pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const GULID_COMMENT_MAX_LEN: usize = 50;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCGuild {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCGuildSubGrade {
    pub grade_name: [u8; GUILD_GRADE_NAME_MAX_LEN + 1],
    pub auth_flag: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCGuildSubMember {
    pub pid: u32,
    pub byGrade: u8,
    pub byIsGeneral: u8,
    pub byJob: u8,
    pub byLevel: u8,
    pub dwOffer: u32,
    pub byNameFlag: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCGuildInfo {
    pub member_count: u16,
    pub max_member_count: u16,
    pub guild_id: u32,
    pub master_pid: u32,
    pub exp: u32,
    pub level: u8,
    pub name: [u8; GUILD_NAME_MAX_LEN + 1],
    pub gold: u32,
    pub hasLand: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCGuildWar {
    pub dwGuildSelf: u32,
    pub dwGuildOpp: u32,
    pub bType: u8,
    pub bWarState: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGuildWarPoint {
    pub dwGainGuildID: u32,
    pub dwOpponentGuildID: u32,
    pub lPoint: i32,
}

pub mod guild_sub {
    pub const INVITE_ANSWER: u8 = 0;
    pub const INVITE: u8 = 1;
    pub const NAME: u8 = 2;
    pub const WAR_LIST: u8 = 3;
    pub const WAR_END_LIST: u8 = 4;
}

// Messenger
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessenger {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerListOffline {
    pub connected: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerListOnline {
    pub connected: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerLogin {
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMessengerLogout {
    pub length: u8,
}

pub mod messenger_sub {
    pub const LIST: u8 = 0;
    pub const LOGIN: u8 = 1;
    pub const LOGOUT: u8 = 2;
    pub const INVITE: u8 = 3; // Kept for mapping if needed
    pub const REMOVE_FRIEND: u8 = 4;
}

// Quest & Script
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCQuestInfo {
    pub header: u16,
    pub length: u16,
    pub index: u16,
    pub flag: u8,
}

pub const QUEST_SEND_IS_BEGIN: u8 = 1 << 0;
pub const QUEST_SEND_TITLE: u8 = 1 << 1;
pub const QUEST_SEND_CLOCK_NAME: u8 = 1 << 2;
pub const QUEST_SEND_CLOCK_VALUE: u8 = 1 << 3;
pub const QUEST_SEND_COUNTER_NAME: u8 = 1 << 4;
pub const QUEST_SEND_COUNTER_VALUE: u8 = 1 << 5;
pub const QUEST_SEND_ICON_FILE: u8 = 1 << 6;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCQuestConfirm {
    pub header: u16,
    pub length: u16,
    pub msg: [u8; 64 + 1],
    pub timeout: i32,
    pub requestPID: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCScript {
    pub header: u16,
    pub length: u16,
    pub skin: u8,
    pub src_size: u16,
}

pub fn parse_quest_confirm(data: &[u8]) -> Result<(String, i32, u32), &'static str> {
    if data.len() < std::mem::size_of::<TPacketGCQuestConfirm>() {
        return Err("Invalid length for QuestConfirm packet");
    }

    let pkt_ptr = data.as_ptr() as *const TPacketGCQuestConfirm;
    let pkt = unsafe { std::ptr::read_unaligned(pkt_ptr) };

    let mut actual_len = 0;
    for (i, &b) in pkt.msg.iter().enumerate() {
        if b == 0 {
            actual_len = i;
            break;
        }
        actual_len = i + 1;
    }

    let msg = String::from_utf8_lossy(&pkt.msg[..actual_len]).to_string();
    Ok((msg, pkt.timeout, pkt.requestPID))
}

pub fn parse_quest_info_flag(flag: u8, data: &[u8]) -> Result<HashMap<&'static str, String>, &'static str> {
    let mut offset = 0;
    let mut info = HashMap::new();

    if flag & QUEST_SEND_IS_BEGIN != 0 {
        if offset + 1 > data.len() {
            return Err("Not enough data for is_begin");
        }
        let is_begin = data[offset];
        offset += 1;
        info.insert("is_begin", is_begin.to_string());
    }

    if flag & QUEST_SEND_TITLE != 0 {
        if offset + 31 > data.len() {
            return Err("Not enough data for title");
        }
        let title_data = &data[offset..offset + 31];
        let mut len = 0;
        for (i, &b) in title_data.iter().enumerate() {
            if b == 0 {
                len = i;
                break;
            }
            len = i + 1;
        }
        info.insert("title", String::from_utf8_lossy(&title_data[..len]).to_string());
        offset += 31;
    }

    if flag & QUEST_SEND_CLOCK_NAME != 0 {
        if offset + 17 > data.len() {
            return Err("Not enough data for clock_name");
        }
        let clock_name_data = &data[offset..offset + 17];
        let mut len = 0;
        for (i, &b) in clock_name_data.iter().enumerate() {
            if b == 0 {
                len = i;
                break;
            }
            len = i + 1;
        }
        info.insert("clock_name", String::from_utf8_lossy(&clock_name_data[..len]).to_string());
        offset += 17;
    }

    if flag & QUEST_SEND_CLOCK_VALUE != 0 {
        if offset + 4 > data.len() {
            return Err("Not enough data for clock_value");
        }
        let mut val = [0u8; 4];
        val.copy_from_slice(&data[offset..offset + 4]);
        info.insert("clock_value", i32::from_le_bytes(val).to_string());
        offset += 4;
    }

    if flag & QUEST_SEND_COUNTER_NAME != 0 {
        if offset + 17 > data.len() {
            return Err("Not enough data for counter_name");
        }
        let counter_name_data = &data[offset..offset + 17];
        let mut len = 0;
        for (i, &b) in counter_name_data.iter().enumerate() {
            if b == 0 {
                len = i;
                break;
            }
            len = i + 1;
        }
        info.insert("counter_name", String::from_utf8_lossy(&counter_name_data[..len]).to_string());
        offset += 17;
    }

    if flag & QUEST_SEND_COUNTER_VALUE != 0 {
        if offset + 4 > data.len() {
            return Err("Not enough data for counter_value");
        }
        let mut val = [0u8; 4];
        val.copy_from_slice(&data[offset..offset + 4]);
        info.insert("counter_value", i32::from_le_bytes(val).to_string());
        offset += 4;
    }

    if flag & QUEST_SEND_ICON_FILE != 0 {
        if offset + 25 > data.len() {
            return Err("Not enough data for icon_file");
        }
        let icon_file_data = &data[offset..offset + 25];
        let mut len = 0;
        for (i, &b) in icon_file_data.iter().enumerate() {
            if b == 0 {
                len = i;
                break;
            }
            len = i + 1;
        }
        info.insert("icon_file", String::from_utf8_lossy(&icon_file_data[..len]).to_string());
    }

    Ok(info)
}

pub fn parse_script(data: &[u8]) -> Result<(u8, String), &'static str> {
    if data.len() < std::mem::size_of::<TPacketGCScript>() {
        return Err("Invalid length for Script packet");
    }

    let pkt_ptr = data.as_ptr() as *const TPacketGCScript;
    let pkt = unsafe { std::ptr::read_unaligned(pkt_ptr) };

    if (pkt.length as usize) < std::mem::size_of::<TPacketGCScript>() {
        return Err("Packet length is too small");
    }

    if data.len() < pkt.length as usize {
        return Err("Data length is smaller than packet length");
    }

    let str_len = pkt.length as usize - std::mem::size_of::<TPacketGCScript>();
    let str_data = &data[std::mem::size_of::<TPacketGCScript>()..std::mem::size_of::<TPacketGCScript>() + str_len];

    let mut actual_len = str_len;
    if actual_len > 0 && str_data[actual_len - 1] == 0 {
        actual_len -= 1;
    }

    let script_str = String::from_utf8_lossy(&str_data[..actual_len]).to_string();
    Ok((pkt.skin, script_str))
}
