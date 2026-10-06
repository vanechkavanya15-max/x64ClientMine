#![allow(dead_code)]

use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;
pub const CHARACTER_NAME_MAX_LEN: usize = 24; // Assuming 24 as in common metin2
pub const GULID_COMMENT_MAX_LEN: usize = 50;

pub mod guild_sub_gc {
    pub const LOGIN: u8 = 0;
    pub const LOGOUT: u8 = 1;
    pub const LIST: u8 = 2;
    pub const GRADE: u8 = 3;
    pub const ADD: u8 = 4;
    pub const REMOVE: u8 = 5;
    pub const GRADE_NAME: u8 = 6;
    pub const GRADE_AUTH: u8 = 7;
    pub const INFO: u8 = 8;
    pub const COMMENTS: u8 = 9;
    pub const CHANGE_EXP: u8 = 10;
    pub const CHANGE_MEMBER_GRADE: u8 = 11;
    pub const SKILL_INFO: u8 = 12;
    pub const CHANGE_MEMBER_GENERAL: u8 = 13;
    pub const GUILD_INVITE: u8 = 14;
    pub const WAR: u8 = 15;
    pub const GUILD_NAME: u8 = 16;
    pub const GUILD_WAR_LIST: u8 = 17;
    pub const GUILD_WAR_END_LIST: u8 = 18;
    pub const WAR_POINT: u8 = 19;
    pub const MONEY_CHANGE: u8 = 20;
}

pub mod guild_war_state {
    pub const NONE: u8 = 0;
    pub const SEND_DECLARE: u8 = 1;
    pub const REFUSE: u8 = 2;
    pub const RECV_DECLARE: u8 = 3;
    pub const WAIT_START: u8 = 4;
    pub const CANCEL: u8 = 5;
    pub const ON_WAR: u8 = 6;
    pub const END: u8 = 7;
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCGuild {
    pub header: u8,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCGuildSubGrade {
    pub grade_name: [u8; GUILD_GRADE_NAME_MAX_LEN + 1],
    pub auth_flag: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCGuildSubMember {
    pub pid: u32,
    pub by_grade: u8,
    pub by_is_general: u8,
    pub by_job: u8,
    pub by_level: u8,
    pub dw_offer: u32,
    pub by_name_flag: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCGuildInfo {
    pub member_count: u16,
    pub max_member_count: u16,
    pub guild_id: u32,
    pub master_pid: u32,
    pub exp: u32,
    pub level: u8,
    pub name: [u8; GUILD_NAME_MAX_LEN + 1],
    pub gold: u32,
    pub has_land: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCGuildWar {
    pub dw_guild_self: u32,
    pub dw_guild_opp: u32,
    pub b_type: u8,
    pub b_war_state: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGuildWarPoint {
    pub dw_gain_guild_id: u32,
    pub dw_opponent_guild_id: u32,
    pub l_point: i32,
}

#[derive(Clone)]
pub struct GuildMemberData {
    pub pid: u32,
    pub name: String,
    pub grade: u8,
    pub job: u8,
    pub level: u8,
    pub offer: u32,
    pub is_general: u8,
}

#[derive(Clone)]
pub struct GuildGradeData {
    pub grade_name: String,
    pub auth_flag: u8,
}

#[derive(Clone, Default)]
pub struct GuildInfoData {
    pub guild_name: String,
    pub guild_id: u32,
    pub master_pid: u32,
    pub guild_level: u8,
    pub current_exp: u32,
    pub current_member_count: u16,
    pub max_member_count: u16,
    pub guild_money: u32,
    pub has_land: u8,
}

#[derive(Clone)]
pub struct GuildComment {
    pub comment_id: u32,
    pub name: String,
    pub comment: String,
}

#[derive(Clone, Default)]
pub struct GuildSkillData {
    pub skill_point: u8,
    pub skill_level: [u8; 18], // Typical array
    pub guild_point: u16,
    pub max_guild_point: u16,
}

pub struct GuildClanSync {
    pub main_player_pid: u32,
    pub main_player_name: String,
    pub members: HashMap<u32, GuildMemberData>,
    pub grades: HashMap<u8, GuildGradeData>,
    pub info: GuildInfoData,
    pub comments: Vec<GuildComment>,
    pub skill_data: GuildSkillData,
    pub active_wars: HashSet<u32>,
    pub known_guild_names: HashMap<u32, String>,
    pub gvg_keys: HashSet<(u32, u32)>, // src, dst
    
    // UI/Messenger Mock Events
    pub messenger_login_events: Vec<String>,
    pub messenger_logout_events: Vec<String>,
    pub ui_events: Vec<String>,
    pub war_events: Vec<String>,
}

impl GuildClanSync {
    pub fn new(main_player_pid: u32, main_player_name: String) -> Self {
        Self {
            main_player_pid,
            main_player_name,
            members: HashMap::new(),
            grades: HashMap::new(),
            info: GuildInfoData::default(),
            comments: Vec::new(),
            skill_data: GuildSkillData::default(),
            active_wars: HashSet::new(),
            known_guild_names: HashMap::new(),
            gvg_keys: HashSet::new(),
            messenger_login_events: Vec::new(),
            messenger_logout_events: Vec::new(),
            ui_events: Vec::new(),
            war_events: Vec::new(),
        }
    }

    fn read_exact(&self, stream: &mut dyn Read, buf: &mut [u8]) -> Result<(), io::Error> {
        stream.read_exact(buf)
    }

    pub fn recv_guild(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pack_bytes = [0u8; std::mem::size_of::<TPacketGCGuild>()];
        if stream.read_exact(&mut pack_bytes).is_err() {
            return Ok(false);
        }

        let pack: TPacketGCGuild = unsafe { std::ptr::read_unaligned(pack_bytes.as_ptr() as *const TPacketGCGuild) };

        match pack.subheader {
            guild_sub_gc::LOGIN => self.recv_guild_sub_login(stream),
            guild_sub_gc::LOGOUT => self.recv_guild_sub_logout(stream),
            guild_sub_gc::REMOVE => self.recv_guild_sub_remove(stream),
            guild_sub_gc::LIST => self.recv_guild_sub_list(&pack, stream),
            guild_sub_gc::GRADE => self.recv_guild_sub_grade(stream),
            guild_sub_gc::GRADE_NAME => self.recv_guild_sub_grade_name(stream),
            guild_sub_gc::GRADE_AUTH => self.recv_guild_sub_grade_auth(stream),
            guild_sub_gc::INFO => self.recv_guild_sub_info(stream),
            guild_sub_gc::COMMENTS => self.recv_guild_sub_comments(stream),
            guild_sub_gc::CHANGE_EXP => self.recv_guild_sub_change_exp(stream),
            guild_sub_gc::CHANGE_MEMBER_GRADE => self.recv_guild_sub_change_member_grade(stream),
            guild_sub_gc::SKILL_INFO => self.recv_guild_sub_skill_info(stream),
            guild_sub_gc::CHANGE_MEMBER_GENERAL => self.recv_guild_sub_change_member_general(stream),
            guild_sub_gc::GUILD_INVITE => self.recv_guild_sub_invite(stream),
            guild_sub_gc::WAR => self.recv_guild_sub_war(stream),
            guild_sub_gc::GUILD_NAME => self.recv_guild_sub_name(&pack, stream),
            guild_sub_gc::GUILD_WAR_LIST => self.recv_guild_sub_war_list(&pack, stream),
            guild_sub_gc::GUILD_WAR_END_LIST => self.recv_guild_sub_war_end_list(&pack, stream),
            guild_sub_gc::WAR_POINT => self.recv_guild_sub_war_point(stream),
            guild_sub_gc::MONEY_CHANGE => self.recv_guild_sub_money_change(stream),
            _ => {
                // Unknown subheader
                Ok(true)
            }
        }
    }

    fn recv_guild_sub_login(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pid_bytes = [0u8; 4];
        if stream.read_exact(&mut pid_bytes).is_err() {
            return Ok(false);
        }
        let pid = u32::from_le_bytes(pid_bytes);

        if let Some(member) = self.members.get(&pid) {
            if member.name != self.main_player_name {
                self.messenger_login_events.push(member.name.clone());
            }
        }
        Ok(true)
    }

    fn recv_guild_sub_logout(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pid_bytes = [0u8; 4];
        if stream.read_exact(&mut pid_bytes).is_err() {
            return Ok(false);
        }
        let pid = u32::from_le_bytes(pid_bytes);

        if let Some(member) = self.members.get(&pid) {
            if member.name != self.main_player_name {
                self.messenger_logout_events.push(member.name.clone());
            }
        }
        Ok(true)
    }

    fn recv_guild_sub_remove(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pid_bytes = [0u8; 4];
        if stream.read_exact(&mut pid_bytes).is_err() {
            return Ok(false);
        }
        let pid = u32::from_le_bytes(pid_bytes);

        if self.main_player_pid == pid {
            self.members.clear();
            self.grades.clear();
            self.info = GuildInfoData::default();
            self.comments.clear();
            self.ui_events.push("DeleteGuild".to_string());
            self.messenger_logout_events.clear(); // remove all guild member
        } else {
            if let Some(member) = self.members.remove(&pid) {
                self.messenger_logout_events.push(member.name.clone());
            }
            self.ui_events.push("RefreshGuildWindowMemberPage".to_string());
        }
        Ok(true)
    }

    fn recv_guild_sub_list(&mut self, pack: &TPacketGCGuild, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut packet_size = pack.length as i32 - std::mem::size_of::<TPacketGCGuild>() as i32;

        while packet_size > 0 {
            let mut member_bytes = [0u8; std::mem::size_of::<TPacketGCGuildSubMember>()];
            if stream.read_exact(&mut member_bytes).is_err() {
                return Ok(false);
            }
            let member_pack: TPacketGCGuildSubMember = unsafe { std::ptr::read_unaligned(member_bytes.as_ptr() as *const TPacketGCGuildSubMember) };
            
            let mut name_str = String::new();
            if member_pack.by_name_flag != 0 {
                let mut sz_name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
                if stream.read_exact(&mut sz_name).is_err() {
                    return Ok(false);
                }
                if let Some(end) = sz_name.iter().position(|&c| c == 0) {
                    name_str = String::from_utf8_lossy(&sz_name[..end]).into_owned();
                }
                packet_size -= (CHARACTER_NAME_MAX_LEN + 1) as i32;
            } else {
                let pid = member_pack.pid;
                if let Some(existing) = self.members.get(&pid) {
                    name_str = existing.name.clone();
                }
            }

            self.members.insert(member_pack.pid, GuildMemberData {
                pid: member_pack.pid,
                grade: member_pack.by_grade,
                name: name_str.clone(),
                job: member_pack.by_job,
                level: member_pack.by_level,
                offer: member_pack.dw_offer,
                is_general: member_pack.by_is_general,
            });

            if name_str != self.main_player_name {
                // mock Messenger AppendGuildMember
            }

            packet_size -= std::mem::size_of::<TPacketGCGuildSubMember>() as i32;
        }

        self.ui_events.push("RefreshGuildWindowInfoPage".to_string());
        self.ui_events.push("RefreshGuildWindowMemberPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_grade(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut count_bytes = [0u8; 1];
        if stream.read_exact(&mut count_bytes).is_err() {
            return Ok(false);
        }
        let count = count_bytes[0];

        for _ in 0..count {
            let mut index_bytes = [0u8; 1];
            if stream.read_exact(&mut index_bytes).is_err() {
                return Ok(false);
            }
            let index = index_bytes[0];

            let mut grade_bytes = [0u8; std::mem::size_of::<TPacketGCGuildSubGrade>()];
            if stream.read_exact(&mut grade_bytes).is_err() {
                return Ok(false);
            }
            let grade_pack: TPacketGCGuildSubGrade = unsafe { std::ptr::read_unaligned(grade_bytes.as_ptr() as *const TPacketGCGuildSubGrade) };

            let mut name_str = String::new();
            if let Some(end) = grade_pack.grade_name.iter().position(|&c| c == 0) {
                name_str = String::from_utf8_lossy(&grade_pack.grade_name[..end]).into_owned();
            }

            self.grades.insert(index, GuildGradeData {
                grade_name: name_str,
                auth_flag: grade_pack.auth_flag,
            });
        }
        self.ui_events.push("RefreshGuildWindowGradePage".to_string());
        self.ui_events.push("RefreshGuildWindowMemberPageGradeComboBox".to_string());
        Ok(true)
    }

    fn recv_guild_sub_grade_name(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut num_bytes = [0u8; 1];
        if stream.read_exact(&mut num_bytes).is_err() { return Ok(false); }
        let grade_num = num_bytes[0];

        let mut name_bytes = [0u8; GUILD_GRADE_NAME_MAX_LEN + 1];
        if stream.read_exact(&mut name_bytes).is_err() { return Ok(false); }

        let name_str = if let Some(end) = name_bytes.iter().position(|&c| c == 0) {
            String::from_utf8_lossy(&name_bytes[..end]).into_owned()
        } else {
            String::from_utf8_lossy(&name_bytes).into_owned()
        };

        if let Some(grade) = self.grades.get_mut(&grade_num) {
            grade.grade_name = name_str;
        }

        self.ui_events.push("RefreshGuildGrade".to_string());
        self.ui_events.push("RefreshGuildWindowGradePage".to_string());
        self.ui_events.push("RefreshGuildWindowMemberPageGradeComboBox".to_string());
        Ok(true)
    }

    fn recv_guild_sub_grade_auth(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut num_bytes = [0u8; 1];
        if stream.read_exact(&mut num_bytes).is_err() { return Ok(false); }
        let grade_num = num_bytes[0];

        let mut auth_bytes = [0u8; 1];
        if stream.read_exact(&mut auth_bytes).is_err() { return Ok(false); }
        let auth_flag = auth_bytes[0];

        if let Some(grade) = self.grades.get_mut(&grade_num) {
            grade.auth_flag = auth_flag;
        }

        self.ui_events.push("RefreshGuildGrade".to_string());
        self.ui_events.push("RefreshGuildWindowGradePage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_info(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut info_bytes = [0u8; std::mem::size_of::<TPacketGCGuildInfo>()];
        if stream.read_exact(&mut info_bytes).is_err() { return Ok(false); }
        let info_pack: TPacketGCGuildInfo = unsafe { std::ptr::read_unaligned(info_bytes.as_ptr() as *const TPacketGCGuildInfo) };

        let name_str = if let Some(end) = info_pack.name.iter().position(|&c| c == 0) {
            String::from_utf8_lossy(&info_pack.name[..end]).into_owned()
        } else {
            String::from_utf8_lossy(&info_pack.name).into_owned()
        };

        self.info = GuildInfoData {
            guild_name: name_str,
            guild_id: { let v = info_pack.guild_id; v },
            master_pid: { let v = info_pack.master_pid; v },
            guild_level: { let v = info_pack.level; v },
            current_exp: { let v = info_pack.exp; v },
            current_member_count: { let v = info_pack.member_count; v },
            max_member_count: { let v = info_pack.max_member_count; v },
            guild_money: { let v = info_pack.gold; v },
            has_land: { let v = info_pack.has_land; v },
        };

        self.ui_events.push("RefreshGuildWindowInfoPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_comments(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut count_bytes = [0u8; 1];
        if stream.read_exact(&mut count_bytes).is_err() { return Ok(false); }
        let count = count_bytes[0];

        self.comments.clear();

        for _ in 0..count {
            let mut id_bytes = [0u8; 4];
            if stream.read_exact(&mut id_bytes).is_err() { return Ok(false); }
            let id = u32::from_le_bytes(id_bytes);

            let mut name_bytes = [0u8; CHARACTER_NAME_MAX_LEN + 1];
            if stream.read_exact(&mut name_bytes).is_err() { return Ok(false); }
            let name_str = if let Some(end) = name_bytes.iter().position(|&c| c == 0) {
                String::from_utf8_lossy(&name_bytes[..end]).into_owned()
            } else {
                String::from_utf8_lossy(&name_bytes).into_owned()
            };

            let mut comment_bytes = [0u8; GULID_COMMENT_MAX_LEN + 1];
            if stream.read_exact(&mut comment_bytes).is_err() { return Ok(false); }
            let comment_str = if let Some(end) = comment_bytes.iter().position(|&c| c == 0) {
                String::from_utf8_lossy(&comment_bytes[..end]).into_owned()
            } else {
                String::from_utf8_lossy(&comment_bytes).into_owned()
            };

            self.comments.push(GuildComment {
                comment_id: id,
                name: name_str,
                comment: comment_str,
            });
        }

        self.ui_events.push("RefreshGuildWindowBoardPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_change_exp(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut level_bytes = [0u8; 1];
        if stream.read_exact(&mut level_bytes).is_err() { return Ok(false); }
        
        let mut exp_bytes = [0u8; 4];
        if stream.read_exact(&mut exp_bytes).is_err() { return Ok(false); }
        
        self.info.guild_level = level_bytes[0];
        self.info.current_exp = u32::from_le_bytes(exp_bytes);

        self.ui_events.push("RefreshGuildWindowInfoPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_change_member_grade(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pid_bytes = [0u8; 4];
        if stream.read_exact(&mut pid_bytes).is_err() { return Ok(false); }
        let pid = u32::from_le_bytes(pid_bytes);

        let mut grade_bytes = [0u8; 1];
        if stream.read_exact(&mut grade_bytes).is_err() { return Ok(false); }
        
        if let Some(member) = self.members.get_mut(&pid) {
            member.grade = grade_bytes[0];
        }

        self.ui_events.push("RefreshGuildWindowMemberPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_skill_info(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut sp_bytes = [0u8; 1];
        if stream.read_exact(&mut sp_bytes).is_err() { return Ok(false); }
        
        let mut sl_bytes = [0u8; 18];
        if stream.read_exact(&mut sl_bytes).is_err() { return Ok(false); }

        let mut gp_bytes = [0u8; 2];
        if stream.read_exact(&mut gp_bytes).is_err() { return Ok(false); }

        let mut maxgp_bytes = [0u8; 2];
        if stream.read_exact(&mut maxgp_bytes).is_err() { return Ok(false); }

        self.skill_data.skill_point = sp_bytes[0];
        self.skill_data.skill_level = sl_bytes;
        self.skill_data.guild_point = u16::from_le_bytes(gp_bytes);
        self.skill_data.max_guild_point = u16::from_le_bytes(maxgp_bytes);

        self.ui_events.push("RefreshGuildWindowSkillPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_change_member_general(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut pid_bytes = [0u8; 4];
        if stream.read_exact(&mut pid_bytes).is_err() { return Ok(false); }
        let pid = u32::from_le_bytes(pid_bytes);

        let mut flag_bytes = [0u8; 1];
        if stream.read_exact(&mut flag_bytes).is_err() { return Ok(false); }
        
        if let Some(member) = self.members.get_mut(&pid) {
            member.is_general = flag_bytes[0];
        }

        self.ui_events.push("RefreshGuildWindowMemberPage".to_string());
        Ok(true)
    }

    fn recv_guild_sub_invite(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut id_bytes = [0u8; 4];
        if stream.read_exact(&mut id_bytes).is_err() { return Ok(false); }
        let id = u32::from_le_bytes(id_bytes);

        let mut name_bytes = [0u8; GUILD_NAME_MAX_LEN + 1];
        if stream.read_exact(&mut name_bytes).is_err() { return Ok(false); }
        
        let name_str = if let Some(end) = name_bytes.iter().position(|&c| c == 0) {
            String::from_utf8_lossy(&name_bytes[..end]).into_owned()
        } else {
            String::from_utf8_lossy(&name_bytes).into_owned()
        };

        self.ui_events.push(format!("RecvGuildInviteQuestion({}, {})", id, name_str));
        Ok(true)
    }

    fn recv_guild_sub_war(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut war_bytes = [0u8; std::mem::size_of::<TPacketGCGuildWar>()];
        if stream.read_exact(&mut war_bytes).is_err() { return Ok(false); }
        let war_pack: TPacketGCGuildWar = unsafe { std::ptr::read_unaligned(war_bytes.as_ptr() as *const TPacketGCGuildWar) };
        
        let dw_guild_opp = {let dw_guild_opp = war_pack.dw_guild_opp; dw_guild_opp};
        let dw_guild_self = {let dw_guild_self = war_pack.dw_guild_self; dw_guild_self};
        let b_type = {let b_type = war_pack.b_type; b_type};

        match war_pack.b_war_state {
            guild_war_state::SEND_DECLARE => {
                self.war_events.push(format!("BINARY_GuildWar_OnSendDeclare({})", dw_guild_opp));
            }
            guild_war_state::RECV_DECLARE => {
                self.war_events.push(format!("BINARY_GuildWar_OnRecvDeclare({}, {})", dw_guild_opp, b_type));
            }
            guild_war_state::ON_WAR => {
                self.war_events.push(format!("BINARY_GuildWar_OnStart({}, {})", dw_guild_self, dw_guild_opp));
                self.active_wars.insert(dw_guild_opp);
            }
            guild_war_state::END => {
                self.war_events.push(format!("BINARY_GuildWar_OnEnd({}, {})", dw_guild_self, dw_guild_opp));
                self.active_wars.remove(&dw_guild_opp);
            }
            _ => {}
        }
        Ok(true)
    }

    fn recv_guild_sub_name(&mut self, pack: &TPacketGCGuild, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut packet_size = pack.length as i32 - std::mem::size_of::<TPacketGCGuild>() as i32;
        let item_size = std::mem::size_of::<u32>() as i32 + GUILD_NAME_MAX_LEN as i32;

        while packet_size > 0 {
            let mut id_bytes = [0u8; 4];
            if stream.read_exact(&mut id_bytes).is_err() { return Ok(false); }
            let id = u32::from_le_bytes(id_bytes);

            let mut name_bytes = [0u8; GUILD_NAME_MAX_LEN];
            if stream.read_exact(&mut name_bytes).is_err() { return Ok(false); }

            let name_str = if let Some(end) = name_bytes.iter().position(|&c| c == 0) {
                String::from_utf8_lossy(&name_bytes[..end]).into_owned()
            } else {
                String::from_utf8_lossy(&name_bytes).into_owned()
            };

            self.known_guild_names.insert(id, name_str);

            packet_size -= item_size;
        }
        Ok(true)
    }

    fn recv_guild_sub_war_list(&mut self, pack: &TPacketGCGuild, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut packet_size = pack.length as i32 - std::mem::size_of::<TPacketGCGuild>() as i32;
        let item_size = std::mem::size_of::<u32>() as i32 * 2;

        while packet_size > 0 {
            let mut src_bytes = [0u8; 4];
            if stream.read_exact(&mut src_bytes).is_err() { return Ok(false); }
            let src_id = u32::from_le_bytes(src_bytes);

            let mut dst_bytes = [0u8; 4];
            if stream.read_exact(&mut dst_bytes).is_err() { return Ok(false); }
            let dst_id = u32::from_le_bytes(dst_bytes);

            self.gvg_keys.insert((src_id, dst_id));

            packet_size -= item_size;
        }
        Ok(true)
    }

    fn recv_guild_sub_war_end_list(&mut self, pack: &TPacketGCGuild, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut packet_size = pack.length as i32 - std::mem::size_of::<TPacketGCGuild>() as i32;
        let item_size = std::mem::size_of::<u32>() as i32 * 2;

        while packet_size > 0 {
            let mut src_bytes = [0u8; 4];
            if stream.read_exact(&mut src_bytes).is_err() { return Ok(false); }
            let src_id = u32::from_le_bytes(src_bytes);

            let mut dst_bytes = [0u8; 4];
            if stream.read_exact(&mut dst_bytes).is_err() { return Ok(false); }
            let dst_id = u32::from_le_bytes(dst_bytes);

            self.gvg_keys.remove(&(src_id, dst_id));

            packet_size -= item_size;
        }
        Ok(true)
    }

    fn recv_guild_sub_war_point(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut point_bytes = [0u8; std::mem::size_of::<TPacketGuildWarPoint>()];
        if stream.read_exact(&mut point_bytes).is_err() { return Ok(false); }
        let point_pack: TPacketGuildWarPoint = unsafe { std::ptr::read_unaligned(point_bytes.as_ptr() as *const TPacketGuildWarPoint) };
        let gain = {let gain = point_pack.dw_gain_guild_id; gain};
        let opp = {let opp = point_pack.dw_opponent_guild_id; opp};
        let l_point = {let l_point = point_pack.l_point; l_point};

        self.war_events.push(format!("BINARY_GuildWar_OnRecvPoint({}, {}, {})", gain, opp, l_point));
        Ok(true)
    }

    fn recv_guild_sub_money_change(&mut self, stream: &mut dyn Read) -> Result<bool, io::Error> {
        let mut money_bytes = [0u8; 4];
        if stream.read_exact(&mut money_bytes).is_err() { return Ok(false); }
        
        self.info.guild_money = u32::from_le_bytes(money_bytes);

        self.ui_events.push("RefreshGuildWindowInfoPage".to_string());
        Ok(true)
    }
}
