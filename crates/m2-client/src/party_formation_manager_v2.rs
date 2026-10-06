use std::collections::HashMap;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;
pub const MAX_PARTY_MEMBERS: usize = 8;
pub const PARTY_MAX_DISTANCE: f32 = 5000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PartyRole {
    Normal = 0,
    Leader = 1,
    Attacker = 2,
    Tanker = 3,
    Buffer = 4,
    SkillMaster = 5,
    Berserker = 6,
    Defender = 7,
}

impl TryFrom<u8> for PartyRole {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PartyRole::Normal),
            1 => Ok(PartyRole::Leader),
            2 => Ok(PartyRole::Attacker),
            3 => Ok(PartyRole::Tanker),
            4 => Ok(PartyRole::Buffer),
            5 => Ok(PartyRole::SkillMaster),
            6 => Ok(PartyRole::Berserker),
            7 => Ok(PartyRole::Defender),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartyMemberInfo {
    pub pid: u32,
    pub vid: u32,
    pub name: String,
    pub hp_percent: u8,
    pub role: PartyRole,
    pub affects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
    pub is_near: bool, // Based on position updates vs player position
}

impl PartyMemberInfo {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            vid: 0,
            name: name.to_string(),
            hp_percent: 100,
            role: PartyRole::Normal,
            affects: [0; PARTY_AFFECT_SLOT_MAX_NUM],
            is_near: true, // Default to near for testing simplicity unless position is provided
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PartyBonuses {
    pub exp_bonus_percent: u32,
    pub attacker_bonus: u32,
    pub defender_bonus: u32,
    pub buffer_bonus: u32,
    pub skill_master_bonus: u32,
}

pub struct PartyFormationManager {
    members: HashMap<u32, PartyMemberInfo>,
}

impl PartyFormationManager {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
        }
    }

    pub fn append_member(&mut self, pid: u32, name: &str) -> Result<(), &'static str> {
        if self.members.len() >= MAX_PARTY_MEMBERS {
            return Err("Party is full");
        }
        if self.members.contains_key(&pid) {
            return Err("Member already exists");
        }
        self.members.insert(pid, PartyMemberInfo::new(pid, name));
        Ok(())
    }

    pub fn remove_member(&mut self, pid: u32) -> bool {
        self.members.remove(&pid).is_some()
    }

    pub fn link_member(&mut self, pid: u32, vid: u32) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.vid = vid;
            return true;
        }
        false
    }

    pub fn unlink_member(&mut self, pid: u32) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.vid = 0;
            return true;
        }
        false
    }

    pub fn update_member_info(&mut self, pid: u32, state: u8, hp_percent: u8) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            if let Ok(role) = PartyRole::try_from(state) {
                member.role = role;
            }
            member.hp_percent = std::cmp::min(hp_percent, 100);
            return true;
        }
        false
    }

    pub fn update_member_affect(&mut self, pid: u32, affect_index: usize, affect_value: i16) -> bool {
        if affect_index >= PARTY_AFFECT_SLOT_MAX_NUM {
            return false;
        }
        if let Some(member) = self.members.get_mut(&pid) {
            member.affects[affect_index] = affect_value;
            return true;
        }
        false
    }

    pub fn update_member_near(&mut self, pid: u32, is_near: bool) -> bool {
        if let Some(member) = self.members.get_mut(&pid) {
            member.is_near = is_near;
            return true;
        }
        false
    }

    pub fn get_member_by_pid(&self, pid: u32) -> Option<&PartyMemberInfo> {
        self.members.get(&pid)
    }

    pub fn get_member_by_vid(&self, vid: u32) -> Option<&PartyMemberInfo> {
        if vid == 0 {
            return None;
        }
        self.members.values().find(|m| m.vid == vid)
    }
    
    pub fn is_party_member_by_vid(&self, vid: u32) -> bool {
        self.get_member_by_vid(vid).is_some()
    }

    pub fn is_party_member_by_name(&self, name: &str) -> bool {
        self.members.values().any(|m| m.name == name)
    }

    pub fn clear(&mut self) {
        self.members.clear();
    }

    pub fn calculate_group_bonuses(&self) -> PartyBonuses {
        let mut bonuses = PartyBonuses::default();
        let near_members_count = self.members.values().filter(|m| m.is_near).count();
        
        // C++ parity EXP scaling based on near members
        if near_members_count >= 2 {
            // Simplified C++ EXP bonus rule from common instances
            bonuses.exp_bonus_percent = match near_members_count {
                2 => 12,
                3 => 15,
                4 => 20,
                5 => 30,
                6 => 45,
                7 => 65,
                8 => 100,
                _ => 0,
            };
        }

        // Apply role bonuses based on active roles near leader
        for member in self.members.values().filter(|m| m.is_near) {
            match member.role {
                PartyRole::Attacker => bonuses.attacker_bonus += 1,
                PartyRole::Defender => bonuses.defender_bonus += 1,
                PartyRole::Buffer => bonuses.buffer_bonus += 1,
                PartyRole::SkillMaster => bonuses.skill_master_bonus += 1,
                _ => {}
            }
        }

        bonuses
    }
    
    pub fn get_member_count(&self) -> usize {
        self.members.len()
    }
}
