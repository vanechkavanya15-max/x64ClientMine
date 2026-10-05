#![allow(non_snake_case)]
#![allow(dead_code)]

use std::mem::size_of;

pub const GUILD_NAME_MAX_LEN: usize = 12;
pub const GUILD_GRADE_NAME_MAX_LEN: usize = 8;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const GUILD_SKILL_MAX_NUM: usize = 12;

pub mod guild_sub_gc {
    pub const LIST: u8 = 2;
    pub const GRADE: u8 = 3;
    pub const INFO: u8 = 8;
    pub const SKILL_INFO: u8 = 12;
    pub const WAR: u8 = 15;
}

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
pub struct TGuildSkillData {
    pub bySkillPoint: u8,
    pub bySkillLevel: [u8; GUILD_SKILL_MAX_NUM],
    pub wGuildPoint: u16,
    pub wMaxGuildPoint: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedGuildMember {
    pub pid: u32,
    pub byGrade: u8,
    pub byIsGeneral: u8,
    pub byJob: u8,
    pub byLevel: u8,
    pub dwOffer: u32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedGuildGrade {
    pub index: u8,
    pub auth_flag: u8,
    pub grade_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuildEvent {
    Info {
        guild_id: u32,
        master_pid: u32,
        member_count: u16,
        max_member_count: u16,
        exp: u32,
        level: u8,
        name: String,
        gold: u32,
        has_land: bool,
    },
    List(Vec<ParsedGuildMember>),
    Grade(Vec<ParsedGuildGrade>),
    SkillInfo {
        skill_point: u8,
        skill_levels: [u8; GUILD_SKILL_MAX_NUM],
        guild_point: u16,
        max_guild_point: u16,
    },
    War {
        guild_self: u32,
        guild_opp: u32,
        war_type: u8,
        war_state: u8,
    },
}

fn parse_c_string(bytes: &[u8]) -> String {
    let mut len = 0;
    while len < bytes.len() && bytes[len] != 0 {
        len += 1;
    }
    String::from_utf8_lossy(&bytes[..len]).into_owned()
}

pub fn parse_guild_packet(data: &[u8]) -> Result<GuildEvent, &'static str> {
    if data.len() < size_of::<TPacketGCGuild>() {
        return Err("Packet too short for TPacketGCGuild");
    }

    let header_ptr = data.as_ptr() as *const TPacketGCGuild;
    let header = unsafe { std::ptr::read_unaligned(header_ptr) };
    
    let expected_len = header.length as usize;
    if data.len() < expected_len {
        return Err("Data length less than packet length");
    }

    let payload = &data[size_of::<TPacketGCGuild>()..expected_len];

    match header.subheader {
        guild_sub_gc::INFO => {
            if payload.len() < size_of::<TPacketGCGuildInfo>() {
                return Err("Payload too short for INFO");
            }
            let info_ptr = payload.as_ptr() as *const TPacketGCGuildInfo;
            let info = unsafe { std::ptr::read_unaligned(info_ptr) };

            Ok(GuildEvent::Info {
                guild_id: { info.guild_id },
                master_pid: { info.master_pid },
                member_count: { info.member_count },
                max_member_count: { info.max_member_count },
                exp: { info.exp },
                level: { info.level },
                name: parse_c_string(&info.name),
                gold: { info.gold },
                has_land: { info.hasLand } != 0,
            })
        }
        guild_sub_gc::LIST => {
            let mut members = Vec::new();
            let mut offset = 0;
            
            while offset < payload.len() {
                if offset + size_of::<TPacketGCGuildSubMember>() > payload.len() {
                    return Err("Incomplete LIST member struct");
                }
                
                let member_ptr = payload[offset..].as_ptr() as *const TPacketGCGuildSubMember;
                let member = unsafe { std::ptr::read_unaligned(member_ptr) };
                offset += size_of::<TPacketGCGuildSubMember>();
                
                let mut name = None;
                if { member.byNameFlag } != 0 {
                    if offset + CHARACTER_NAME_MAX_LEN + 1 > payload.len() {
                        return Err("Incomplete LIST member name");
                    }
                    name = Some(parse_c_string(&payload[offset..offset + CHARACTER_NAME_MAX_LEN + 1]));
                    offset += CHARACTER_NAME_MAX_LEN + 1;
                }
                
                members.push(ParsedGuildMember {
                    pid: { member.pid },
                    byGrade: { member.byGrade },
                    byIsGeneral: { member.byIsGeneral },
                    byJob: { member.byJob },
                    byLevel: { member.byLevel },
                    dwOffer: { member.dwOffer },
                    name,
                });
            }
            
            Ok(GuildEvent::List(members))
        }
        guild_sub_gc::GRADE => {
            if payload.is_empty() {
                return Err("Payload too short for GRADE count");
            }
            let count = payload[0];
            let mut offset = 1;
            
            let mut grades = Vec::new();
            for _ in 0..count {
                if offset + 1 > payload.len() {
                    return Err("Incomplete GRADE index");
                }
                let index = payload[offset];
                offset += 1;
                
                if offset + size_of::<TPacketGCGuildSubGrade>() > payload.len() {
                    return Err("Incomplete GRADE struct");
                }
                let grade_ptr = payload[offset..].as_ptr() as *const TPacketGCGuildSubGrade;
                let grade = unsafe { std::ptr::read_unaligned(grade_ptr) };
                offset += size_of::<TPacketGCGuildSubGrade>();
                
                grades.push(ParsedGuildGrade {
                    index,
                    auth_flag: { grade.auth_flag },
                    grade_name: parse_c_string(&grade.grade_name),
                });
            }
            Ok(GuildEvent::Grade(grades))
        }
        guild_sub_gc::SKILL_INFO => {
            if payload.len() < size_of::<TGuildSkillData>() {
                return Err("Payload too short for SKILL_INFO");
            }
            let skill_ptr = payload.as_ptr() as *const TGuildSkillData;
            let skill = unsafe { std::ptr::read_unaligned(skill_ptr) };
            
            Ok(GuildEvent::SkillInfo {
                skill_point: { skill.bySkillPoint },
                skill_levels: { skill.bySkillLevel },
                guild_point: { skill.wGuildPoint },
                max_guild_point: { skill.wMaxGuildPoint },
            })
        }
        guild_sub_gc::WAR => {
            if payload.len() < size_of::<TPacketGCGuildWar>() {
                return Err("Payload too short for WAR");
            }
            let war_ptr = payload.as_ptr() as *const TPacketGCGuildWar;
            let war = unsafe { std::ptr::read_unaligned(war_ptr) };
            
            Ok(GuildEvent::War {
                guild_self: { war.dwGuildSelf },
                guild_opp: { war.dwGuildOpp },
                war_type: { war.bType },
                war_state: { war.bWarState },
            })
        }
        _ => Err("Unknown subheader"),
    }
}
