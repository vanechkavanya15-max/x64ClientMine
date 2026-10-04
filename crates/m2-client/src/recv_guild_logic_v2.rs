use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------
// Packet Definitions (Binary compatible with Metin2)
// ---------------------------------------------------------

#[repr(C, packed)]
pub struct PacketGcGuild {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
pub struct PacketGcGuildSubGrade {
    pub grade_name: [u8; 9], // GUILD_GRADE_NAME_MAX_LEN (8) + 1
    pub auth_flag: u8,
}

#[repr(C, packed)]
pub struct PacketGcGuildSubMember {
    pub pid: u32,
    pub by_grade: u8,
    pub by_is_general: u8,
    pub by_job: u8,
    pub by_level: u8,
    pub dw_offer: u32,
    pub by_name_flag: u8,
    // if by_name_flag == 1, name [u8; 25] follows
}

#[repr(C, packed)]
pub struct PacketGcGuildInfo {
    pub member_count: u16,
    pub max_member_count: u16,
    pub guild_id: u32,
    pub master_pid: u32,
    pub exp: u32,
    pub level: u8,
    pub name: [u8; 13], // GUILD_NAME_MAX_LEN (12) + 1
    pub gold: u32,
    pub has_land: u8,
}

#[repr(C, packed)]
pub struct PacketGcGuildWar {
    pub dw_guild_self: u32,
    pub dw_guild_opp: u32,
    pub b_type: u8,
    pub b_war_state: u8,
}

#[repr(C, packed)]
pub struct PacketGuildWarPoint {
    pub dw_gain_guild_id: u32,
    pub dw_opponent_guild_id: u32,
    pub l_point: i32,
}

// ---------------------------------------------------------
// Constants & Enums
// ---------------------------------------------------------

pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const GUILD_SKILL_MAX_NUM: usize = 12;
pub const GULID_COMMENT_MAX_LEN: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildSubGc {
    Login = 0,
    Logout = 1,
    List = 2,
    Grade = 3,
    Add = 4,
    Remove = 5,
    GradeName = 6,
    GradeAuth = 7,
    Info = 8,
    Comments = 9,
    ChangeExp = 10,
    ChangeMemberGrade = 11,
    SkillInfo = 12,
    ChangeMemberGeneral = 13,
    GuildInvite = 14,
    War = 15,
    GuildName = 16,
    GuildWarList = 17,
    GuildWarEndList = 18,
    WarPoint = 19,
    MoneyChange = 20,
    Unknown,
}

impl From<u8> for GuildSubGc {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Login,
            1 => Self::Logout,
            2 => Self::List,
            3 => Self::Grade,
            4 => Self::Add,
            5 => Self::Remove,
            6 => Self::GradeName,
            7 => Self::GradeAuth,
            8 => Self::Info,
            9 => Self::Comments,
            10 => Self::ChangeExp,
            11 => Self::ChangeMemberGrade,
            12 => Self::SkillInfo,
            13 => Self::ChangeMemberGeneral,
            14 => Self::GuildInvite,
            15 => Self::War,
            16 => Self::GuildName,
            17 => Self::GuildWarList,
            18 => Self::GuildWarEndList,
            19 => Self::WarPoint,
            20 => Self::MoneyChange,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildWarType {
    Field = 0,
    Arena = 1,
    Shield = 2,
    Unknown,
}

impl From<u8> for GuildWarType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Field,
            1 => Self::Arena,
            2 => Self::Shield,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildWarState {
    None = 0,
    SendDeclare = 1,
    Refuse = 2,
    RecvDeclare = 3,
    WaitStart = 4,
    Cancel = 5,
    OnWar = 6,
    End = 7,
    Unknown,
}

impl From<u8> for GuildWarState {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::SendDeclare,
            2 => Self::Refuse,
            3 => Self::RecvDeclare,
            4 => Self::WaitStart,
            5 => Self::Cancel,
            6 => Self::OnWar,
            7 => Self::End,
            _ => Self::Unknown,
        }
    }
}

// ---------------------------------------------------------
// Internal State Structures
// ---------------------------------------------------------

#[derive(Clone, Default)]
pub struct GuildGradeData {
    pub auth_flag: u8,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct GuildMemberData {
    pub pid: u32,
    pub grade: u8,
    pub is_general: u8,
    pub job: u8,
    pub level: u8,
    pub offer: u32,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct GuildSkillData {
    pub skill_point: u8,
    pub skill_level: [u8; GUILD_SKILL_MAX_NUM],
    pub guild_point: u16,
    pub max_guild_point: u16,
}

#[derive(Clone, Default)]
pub struct GuildCommentData {
    pub comment_id: u32,
    pub name: String,
    pub comment: String,
}

#[derive(Clone, Default)]
pub struct GuildStateData {
    pub is_enabled: bool,
    pub guild_id: u32,
    pub name: String,
    pub master_pid: u32,
    pub level: u8,
    pub exp: u32,
    pub gold: u64, // Extended precision for internal
    pub member_count: u16,
    pub max_member_count: u16,
    pub has_land: u8,

    pub members: HashMap<u32, GuildMemberData>,
    pub grades: HashMap<u8, GuildGradeData>,
    pub skills: GuildSkillData,
    pub comments: Vec<GuildCommentData>,
    
    // Maps a guild ID to its name
    pub guild_names: HashMap<u32, String>,
    
    // GVG Active Wars (Target Guild ID)
    pub wars: Vec<u32>,
    pub war_types: HashMap<u32, GuildWarType>,
}

pub fn get_guild_state() -> &'static Mutex<GuildStateData> {
    static STATE: OnceLock<Mutex<GuildStateData>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(GuildStateData::default()))
}

// ---------------------------------------------------------
// Helper functions
// ---------------------------------------------------------

fn parse_string(data: &[u8], max_len: usize) -> String {
    let limit = data.len().min(max_len);
    let mut end = limit;
    for (i, &b) in data.iter().enumerate().take(limit) {
        if b == 0 {
            end = i;
            break;
        }
    }
    String::from_utf8_lossy(&data[..end]).into_owned()
}

// ---------------------------------------------------------
// Sub-packet Parsers
// ---------------------------------------------------------

pub fn parse_guild_packet(packet_data: &[u8]) -> Result<(), &'static str> {
    if packet_data.len() < std::mem::size_of::<PacketGcGuild>() {
        return Err("Packet too small for TPacketGCGuild");
    }

    let ptr = packet_data.as_ptr() as *const PacketGcGuild;
    let header = unsafe { ptr.read_unaligned() };

    let sub_data = &packet_data[std::mem::size_of::<PacketGcGuild>()..];
    let mut state = get_guild_state().lock().unwrap();

    match GuildSubGc::from(header.subheader) {
        GuildSubGc::Login => {
            if sub_data.len() < 4 {
                return Err("Login: missing pid");
            }
            let pid = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            // In a real client this would notify messenger
            let _ = pid; 
        }
        GuildSubGc::Logout => {
            if sub_data.len() < 4 {
                return Err("Logout: missing pid");
            }
            let pid = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            let _ = pid;
        }
        GuildSubGc::Remove => {
            if sub_data.len() < 4 {
                return Err("Remove: missing pid");
            }
            let pid = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            state.members.remove(&pid);
        }
        GuildSubGc::List => {
            let mut offset = 0;
            while offset < sub_data.len() {
                if sub_data.len() - offset < std::mem::size_of::<PacketGcGuildSubMember>() {
                    break; // or Err
                }
                
                let mem_ptr = sub_data[offset..].as_ptr() as *const PacketGcGuildSubMember;
                let mem_pack = unsafe { mem_ptr.read_unaligned() };
                
                let pid = mem_pack.pid;
                let by_grade = mem_pack.by_grade;
                let by_is_general = mem_pack.by_is_general;
                let by_job = mem_pack.by_job;
                let by_level = mem_pack.by_level;
                let dw_offer = mem_pack.dw_offer;
                let by_name_flag = mem_pack.by_name_flag;
                
                offset += std::mem::size_of::<PacketGcGuildSubMember>();
                
                let mut name_str = String::new();
                if by_name_flag != 0 {
                    if sub_data.len() - offset < CHARACTER_NAME_MAX_LEN + 1 {
                        break;
                    }
                    name_str = parse_string(&sub_data[offset..], CHARACTER_NAME_MAX_LEN);
                    offset += CHARACTER_NAME_MAX_LEN + 1;
                } else {
                    if let Some(existing) = state.members.get(&pid) {
                        name_str = existing.name.clone();
                    }
                }
                
                state.members.insert(pid, GuildMemberData {
                    pid,
                    grade: by_grade,
                    is_general: by_is_general,
                    job: by_job,
                    level: by_level,
                    offer: dw_offer,
                    name: name_str,
                });
            }
        }
        GuildSubGc::Grade => {
            if sub_data.is_empty() {
                return Err("Grade: missing count");
            }
            let count = sub_data[0];
            let mut offset = 1;
            
            for _ in 0..count {
                if sub_data.len() - offset < 1 {
                    break;
                }
                let index = sub_data[offset];
                offset += 1;
                
                if sub_data.len() - offset < std::mem::size_of::<PacketGcGuildSubGrade>() {
                    break;
                }
                
                let grade_ptr = sub_data[offset..].as_ptr() as *const PacketGcGuildSubGrade;
                let grade_pack = unsafe { grade_ptr.read_unaligned() };
                let auth_flag = grade_pack.auth_flag;
                let name = parse_string(&grade_pack.grade_name, GUILD_GRADE_NAME_MAX_LEN);
                
                state.grades.insert(index, GuildGradeData {
                    auth_flag,
                    name,
                });
                
                offset += std::mem::size_of::<PacketGcGuildSubGrade>();
            }
        }
        GuildSubGc::GradeName => {
            if sub_data.len() < 1 + GUILD_GRADE_NAME_MAX_LEN + 1 {
                return Err("GradeName: invalid size");
            }
            let index = sub_data[0];
            let name = parse_string(&sub_data[1..], GUILD_GRADE_NAME_MAX_LEN);
            
            let entry = state.grades.entry(index).or_default();
            entry.name = name;
        }
        GuildSubGc::GradeAuth => {
            if sub_data.len() < 2 {
                return Err("GradeAuth: invalid size");
            }
            let index = sub_data[0];
            let auth = sub_data[1];
            
            let entry = state.grades.entry(index).or_default();
            entry.auth_flag = auth;
        }
        GuildSubGc::Info => {
            if sub_data.len() < std::mem::size_of::<PacketGcGuildInfo>() {
                return Err("Info: invalid size");
            }
            let info_ptr = sub_data.as_ptr() as *const PacketGcGuildInfo;
            let info = unsafe { info_ptr.read_unaligned() };
            
            state.is_enabled = true;
            state.member_count = info.member_count;
            state.max_member_count = info.max_member_count;
            state.guild_id = info.guild_id;
            state.master_pid = info.master_pid;
            state.exp = info.exp;
            state.level = info.level;
            state.name = parse_string(&info.name, GUILD_NAME_MAX_LEN);
            state.gold = info.gold as u64;
            state.has_land = info.has_land;
        }
        GuildSubGc::Comments => {
            if sub_data.is_empty() {
                return Err("Comments: missing count");
            }
            let count = sub_data[0];
            let mut offset = 1;
            
            state.comments.clear();
            
            for _ in 0..count {
                if sub_data.len() - offset < 4 + (CHARACTER_NAME_MAX_LEN + 1) + (GULID_COMMENT_MAX_LEN + 1) {
                    break;
                }
                
                let comment_id = u32::from_le_bytes(sub_data[offset..offset+4].try_into().unwrap());
                offset += 4;
                
                let name = parse_string(&sub_data[offset..], CHARACTER_NAME_MAX_LEN);
                offset += CHARACTER_NAME_MAX_LEN + 1;
                
                let comment = parse_string(&sub_data[offset..], GULID_COMMENT_MAX_LEN);
                offset += GULID_COMMENT_MAX_LEN + 1;
                
                state.comments.push(GuildCommentData {
                    comment_id,
                    name,
                    comment,
                });
            }
        }
        GuildSubGc::ChangeExp => {
            if sub_data.len() < 5 {
                return Err("ChangeExp: invalid size");
            }
            state.level = sub_data[0];
            state.exp = u32::from_le_bytes(sub_data[1..5].try_into().unwrap());
        }
        GuildSubGc::ChangeMemberGrade => {
            if sub_data.len() < 5 {
                return Err("ChangeMemberGrade: invalid size");
            }
            let pid = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            let grade = sub_data[4];
            if let Some(mem) = state.members.get_mut(&pid) {
                mem.grade = grade;
            }
        }
        GuildSubGc::SkillInfo => {
            if sub_data.len() < 1 + GUILD_SKILL_MAX_NUM + 2 + 2 {
                return Err("SkillInfo: invalid size");
            }
            state.skills.skill_point = sub_data[0];
            state.skills.skill_level.copy_from_slice(&sub_data[1..1+GUILD_SKILL_MAX_NUM]);
            let off = 1 + GUILD_SKILL_MAX_NUM;
            state.skills.guild_point = u16::from_le_bytes(sub_data[off..off+2].try_into().unwrap());
            state.skills.max_guild_point = u16::from_le_bytes(sub_data[off+2..off+4].try_into().unwrap());
        }
        GuildSubGc::ChangeMemberGeneral => {
            if sub_data.len() < 5 {
                return Err("ChangeMemberGeneral: invalid size");
            }
            let pid = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            let flag = sub_data[4];
            if let Some(mem) = state.members.get_mut(&pid) {
                mem.is_general = flag;
            }
        }
        GuildSubGc::GuildInvite => {
            if sub_data.len() < 4 + GUILD_NAME_MAX_LEN {
                return Err("GuildInvite: invalid size");
            }
            // Real client would show a dialog
        }
        GuildSubGc::War => {
            if sub_data.len() < std::mem::size_of::<PacketGcGuildWar>() {
                return Err("War: invalid size");
            }
            let war_ptr = sub_data.as_ptr() as *const PacketGcGuildWar;
            let war = unsafe { war_ptr.read_unaligned() };
            
            let dw_guild_opp = war.dw_guild_opp;
            match GuildWarState::from(war.b_war_state) {
                GuildWarState::OnWar => {
                    if !state.wars.contains(&dw_guild_opp) {
                        state.wars.push(dw_guild_opp);
                        state.war_types.insert(dw_guild_opp, GuildWarType::from(war.b_type));
                    }
                }
                GuildWarState::End => {
                    state.wars.retain(|&w| w != dw_guild_opp);
                    state.war_types.remove(&dw_guild_opp);
                }
                _ => {} // Other states handled by UI typically
            }
        }
        GuildSubGc::GuildName => {
            let item_size = 4 + GUILD_NAME_MAX_LEN;
            let mut offset = 0;
            while offset + item_size <= sub_data.len() {
                let id = u32::from_le_bytes(sub_data[offset..offset+4].try_into().unwrap());
                offset += 4;
                let name = parse_string(&sub_data[offset..], GUILD_NAME_MAX_LEN);
                offset += GUILD_NAME_MAX_LEN;
                
                state.guild_names.insert(id, name);
            }
        }
        GuildSubGc::GuildWarList | GuildSubGc::GuildWarEndList => {
            let item_size = 8;
            let mut offset = 0;
            while offset + item_size <= sub_data.len() {
                let _src = u32::from_le_bytes(sub_data[offset..offset+4].try_into().unwrap());
                let _dst = u32::from_le_bytes(sub_data[offset+4..offset+8].try_into().unwrap());
                offset += 8;
                // In a real client this modifies global instance GVG keys
            }
        }
        GuildSubGc::WarPoint => {
            if sub_data.len() < std::mem::size_of::<PacketGuildWarPoint>() {
                return Err("WarPoint: invalid size");
            }
            // Handled via Python phase UI updates usually
        }
        GuildSubGc::MoneyChange => {
            if sub_data.len() < 4 {
                return Err("MoneyChange: invalid size");
            }
            let money = u32::from_le_bytes(sub_data[0..4].try_into().unwrap());
            state.gold = money as u64;
        }
        GuildSubGc::Unknown => {
            return Err("Unknown subheader");
        }
        _ => {}
    }

    Ok(())
}

// ---------------------------------------------------------
// FFI Export
// ---------------------------------------------------------

#[no_mangle]
pub extern "C" fn m2_guild_get_info(
    out_id: *mut u32,
    out_level: *mut u8,
    out_gold: *mut u64,
) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        
        if !state.is_enabled {
            return false;
        }

        if !out_id.is_null() {
            unsafe { *out_id = state.guild_id };
        }
        if !out_level.is_null() {
            unsafe { *out_level = state.level };
        }
        if !out_gold.is_null() {
            unsafe { *out_gold = state.gold };
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_has_land() -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        state.is_enabled && state.has_land != 0
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_get_skill_point() -> u8 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        state.skills.skill_point
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_get_skill_level(idx: u8) -> u8 {
    safe_ffi_call(0, || {
        let state = get_guild_state().lock().unwrap();
        if (idx as usize) < state.skills.skill_level.len() {
            state.skills.skill_level[idx as usize]
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_get_exp(out_cur: *mut u32, out_max: *mut u32) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        if !out_cur.is_null() {
            unsafe { *out_cur = state.exp };
        }
        if !out_max.is_null() {
            unsafe { *out_max = (state.level as u32).max(1) * 10000 };
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_get_member_count(out_cur: *mut u16, out_max: *mut u16) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        if !out_cur.is_null() {
            unsafe { *out_cur = state.member_count };
        }
        if !out_max.is_null() {
            unsafe { *out_max = state.max_member_count.max(32) };
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_get_grade_name(
    grade_idx: u8,
    out_buf: *mut std::os::raw::c_char,
    max_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }
        let state = get_guild_state().lock().unwrap();
        let name = if let Some(g) = state.grades.get(&grade_idx) {
            g.name.clone()
        } else {
            format!("Grade {}", grade_idx)
        };
        let bytes = name.as_bytes();
        let copy_len = bytes.len().min(max_len.saturating_sub(1));
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const std::os::raw::c_char, out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_guild_main_player_has_authority(_auth_flag: u8) -> bool {
    safe_ffi_call(false, || {
        let state = get_guild_state().lock().unwrap();
        state.is_enabled
    })
}

/// Helper for testing reset
pub fn test_clear_guild_state() {
    let mut state = get_guild_state().lock().unwrap();
    *state = GuildStateData::default();
}
