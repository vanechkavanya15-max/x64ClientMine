use std::collections::HashMap;

pub const GUILD_AUTH_ADD_MEMBER: u8 = 1 << 0;
pub const GUILD_AUTH_REMOVE_MEMBER: u8 = 1 << 1;
pub const GUILD_AUTH_NOTICE: u8 = 1 << 2;
pub const GUILD_AUTH_SKILL: u8 = 1 << 3;

pub const GUILD_SKILL_MAX_NUM: usize = 12;
pub const ENEMY_GUILD_SLOT_MAX_COUNT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildSubHeader {
    Login,
    Logout,
    List,
    Grade,
    Add,
    Remove,
    GradeName,
    GradeAuth,
    Info,
    Comments,
    ChangeExp,
    ChangeMemberGrade,
    SkillInfo,
    ChangeMemberGeneral,
    GuildInvite,
    War,
    GuildName,
    GuildWarList,
    GuildWarEndList,
    WarPoint,
    Unknown(u8),
}

impl From<u8> for GuildSubHeader {
    fn from(value: u8) -> Self {
        match value {
            0 => GuildSubHeader::Login,
            1 => GuildSubHeader::Logout,
            2 => GuildSubHeader::List,
            3 => GuildSubHeader::Grade,
            4 => GuildSubHeader::Add,
            5 => GuildSubHeader::Remove,
            6 => GuildSubHeader::GradeName,
            7 => GuildSubHeader::GradeAuth,
            8 => GuildSubHeader::Info,
            9 => GuildSubHeader::Comments,
            10 => GuildSubHeader::ChangeExp,
            11 => GuildSubHeader::ChangeMemberGrade,
            12 => GuildSubHeader::SkillInfo,
            13 => GuildSubHeader::ChangeMemberGeneral,
            14 => GuildSubHeader::GuildInvite,
            15 => GuildSubHeader::War,
            16 => GuildSubHeader::GuildName,
            17 => GuildSubHeader::GuildWarList,
            18 => GuildSubHeader::GuildWarEndList,
            19 => GuildSubHeader::WarPoint,
            v => GuildSubHeader::Unknown(v),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GuildInfo {
    pub guild_id: u32,
    pub guild_name: String,
    pub master_pid: u32,
    pub guild_level: u32,
    pub current_experience: u32,
    pub current_member_count: u32,
    pub max_member_count: u32,
    pub guild_money: u32,
    pub has_land: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GuildGradeData {
    pub authority_flag: u8,
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GuildMemberData {
    pub pid: u32,
    pub name: String,
    pub grade: u8,
    pub job: u8,
    pub level: u8,
    pub general_flag: u8,
    pub offer: u32,
}

#[derive(Debug, Clone, Default)]
pub struct GuildBoardCommentData {
    pub comment_id: u32,
    pub name: String,
    pub comment: String,
}

#[derive(Debug, Clone, Default)]
pub struct GuildSkillData {
    pub skill_point: u8,
    pub skill_level: [u8; GUILD_SKILL_MAX_NUM],
    pub guild_point: u16,     // Dragon Power Points
    pub max_guild_point: u16, // Max Dragon Power Points
}

pub struct GuildProtocolManager {
    pub info: GuildInfo,
    pub grades: HashMap<u8, GuildGradeData>,
    pub members: Vec<GuildMemberData>,
    pub board_comments: Vec<GuildBoardCommentData>,
    pub skill_data: GuildSkillData,
    pub names: HashMap<u32, String>,
    pub enemy_guild_ids: [u32; ENEMY_GUILD_SLOT_MAX_COUNT],

    pub member_level_summary: u32,
    pub member_level_average: u32,
    pub member_experience_summary: u32,

    pub is_enabled: bool,
}

impl Default for GuildProtocolManager {
    fn default() -> Self {
        Self {
            info: GuildInfo::default(),
            grades: HashMap::new(),
            members: Vec::new(),
            board_comments: Vec::new(),
            skill_data: GuildSkillData::default(),
            names: HashMap::new(),
            enemy_guild_ids: [0; ENEMY_GUILD_SLOT_MAX_COUNT],
            member_level_summary: 0,
            member_level_average: 0,
            member_experience_summary: 0,
            is_enabled: false,
        }
    }
}

impl GuildProtocolManager {
    pub fn enable_guild(&mut self) {
        self.is_enabled = true;
    }

    pub fn set_guild_money(&mut self, money: u32) {
        self.info.guild_money = money;
    }

    pub fn set_guild_exp(&mut self, level: u8, exp: u32) {
        self.info.guild_level = level as u32;
        self.info.current_experience = exp;
    }

    pub fn set_grade_data(&mut self, grade_number: u8, data: GuildGradeData) {
        self.grades.insert(grade_number, data);
    }

    pub fn set_grade_name(&mut self, grade_number: u8, name: &str) {
        if let Some(grade) = self.grades.get_mut(&grade_number) {
            grade.name = name.to_string();
        }
    }

    pub fn set_grade_authority(&mut self, grade_number: u8, authority: u8) {
        if let Some(grade) = self.grades.get_mut(&grade_number) {
            grade.authority_flag = authority;
        }
    }

    pub fn clear_comment(&mut self) {
        self.board_comments.clear();
    }

    pub fn register_comment(&mut self, comment_id: u32, name: &str, comment: &str) {
        if comment.is_empty() {
            return;
        }

        self.board_comments.push(GuildBoardCommentData {
            comment_id,
            name: name.to_string(),
            comment: comment.to_string(),
        });
    }

    pub fn register_member(&mut self, mut member_data: GuildMemberData) {
        if let Some(existing_member) = self.members.iter_mut().find(|m| m.pid == member_data.pid) {
            existing_member.general_flag = member_data.general_flag;
            existing_member.grade = member_data.grade;
            existing_member.level = member_data.level;
            existing_member.offer = member_data.offer;
        } else {
            self.members.push(member_data);
        }

        self.calculate_level_average();
        self.sort_member();
    }

    pub fn change_guild_member_grade(&mut self, pid: u32, grade: u8) {
        if let Some(member) = self.members.iter_mut().find(|m| m.pid == pid) {
            member.grade = grade;
        }
    }

    pub fn change_guild_member_general_flag(&mut self, pid: u32, flag: u8) {
        if let Some(member) = self.members.iter_mut().find(|m| m.pid == pid) {
            member.general_flag = flag;
        }
    }

    pub fn remove_member(&mut self, pid: u32) {
        self.members.retain(|m| m.pid != pid);
        self.calculate_level_average();
    }

    pub fn register_guild_name(&mut self, id: u32, name: &str) {
        self.names.insert(id, name.to_string());
    }

    fn calculate_level_average(&mut self) {
        self.member_level_summary = 0;
        self.member_level_average = 0;
        self.member_experience_summary = 0;

        if self.members.is_empty() {
            return;
        }

        for member in &self.members {
            self.member_level_summary += member.level as u32;
            self.member_experience_summary += member.offer;
        }

        self.member_level_average = self.member_level_summary / self.members.len() as u32;
    }

    fn sort_member(&mut self) {
        self.members.sort_by(|a, b| {
            if a.grade != b.grade {
                a.grade.cmp(&b.grade)
            } else {
                a.name.cmp(&b.name)
            }
        });
    }
    
    pub fn get_guild_skill_data_mut(&mut self) -> &mut GuildSkillData {
        &mut self.skill_data
    }
}
