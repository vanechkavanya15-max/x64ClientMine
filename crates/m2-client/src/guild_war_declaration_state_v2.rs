#![allow(dead_code)]

use std::time::{Duration, SystemTime};

pub const ENEMY_GUILD_SLOT_MAX_COUNT: usize = 6;
pub const GUILD_WAR_DURATION_SECONDS: u32 = 2 * 60 * 60; // 2 hours

#[repr(u8)]
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
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildWarType {
    Field = 0,
    Arena = 1,
    Flag = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildRole {
    Leader = 0,
    General = 1,
    Soldier = 2,
    Member = 3,
}

impl GuildRole {
    pub fn can_declare_war(&self) -> bool {
        matches!(self, GuildRole::Leader | GuildRole::General)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GuildWar {
    pub opponent_guild_id: u32,
    pub war_type: GuildWarType,
    pub state: GuildWarState,
    pub score_self: i32,
    pub score_opp: i32,
    pub countdown_end: Option<SystemTime>,
    pub war_end_time: Option<SystemTime>,
}

impl GuildWar {
    pub fn new(opponent_guild_id: u32, war_type: GuildWarType, state: GuildWarState) -> Self {
        Self {
            opponent_guild_id,
            war_type,
            state,
            score_self: 0,
            score_opp: 0,
            countdown_end: None,
            war_end_time: None,
        }
    }

    pub fn set_score(&mut self, score_self: i32, score_opp: i32) {
        self.score_self = score_self;
        self.score_opp = score_opp;
    }
    
    pub fn start_countdown(&mut self, seconds: u64) {
        self.countdown_end = Some(SystemTime::now() + Duration::from_secs(seconds));
    }
    
    pub fn start_war(&mut self) {
        self.state = GuildWarState::OnWar;
        self.war_end_time = Some(SystemTime::now() + Duration::from_secs(GUILD_WAR_DURATION_SECONDS as u64));
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GuildWarDeclarationStateV2 {
    active_wars: [Option<GuildWar>; ENEMY_GUILD_SLOT_MAX_COUNT],
}

impl GuildWarDeclarationStateV2 {
    pub fn new() -> Self {
        const INIT: Option<GuildWar> = None;
        Self {
            active_wars: [INIT; ENEMY_GUILD_SLOT_MAX_COUNT],
        }
    }

    pub fn start_guild_war(&mut self, enemy_guild_id: u32, war_type: GuildWarType) -> Result<(), &'static str> {
        // Check if already in slot
        for i in 0..ENEMY_GUILD_SLOT_MAX_COUNT {
            if let Some(war) = &self.active_wars[i] {
                if war.opponent_guild_id == enemy_guild_id {
                    return Err("Guild is already in war list");
                }
            }
        }

        // Find empty slot
        for i in 0..ENEMY_GUILD_SLOT_MAX_COUNT {
            if self.active_wars[i].is_none() {
                self.active_wars[i] = Some(GuildWar::new(enemy_guild_id, war_type, GuildWarState::OnWar));
                return Ok(());
            }
        }

        Err("Enemy guild slot is full")
    }

    pub fn end_guild_war(&mut self, enemy_guild_id: u32) {
        for i in 0..ENEMY_GUILD_SLOT_MAX_COUNT {
            if let Some(war) = &self.active_wars[i] {
                if war.opponent_guild_id == enemy_guild_id {
                    self.active_wars[i] = None;
                }
            }
        }
    }

    pub fn get_enemy_guild_id(&self, index: usize) -> Option<u32> {
        if index < ENEMY_GUILD_SLOT_MAX_COUNT {
            if let Some(war) = &self.active_wars[index] {
                return Some(war.opponent_guild_id);
            }
        }
        None
    }

    pub fn is_doing_guild_war(&self) -> bool {
        for i in 0..ENEMY_GUILD_SLOT_MAX_COUNT {
            if self.active_wars[i].is_some() {
                return true;
            }
        }
        false
    }

    pub fn get_war_mut(&mut self, enemy_guild_id: u32) -> Option<&mut GuildWar> {
        for war_opt in self.active_wars.iter_mut() {
            if let Some(war) = war_opt {
                if war.opponent_guild_id == enemy_guild_id {
                    return Some(war);
                }
            }
        }
        None
    }
    
    pub fn update_score(&mut self, enemy_guild_id: u32, score_self: i32, score_opp: i32) -> Result<(), &'static str> {
        if let Some(war) = self.get_war_mut(enemy_guild_id) {
            war.set_score(score_self, score_opp);
            Ok(())
        } else {
            Err("War not found")
        }
    }
}
