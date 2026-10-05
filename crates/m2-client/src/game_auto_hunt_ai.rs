use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoHuntState {
    Idle,
    Search,
    Approach,
    Attack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoHuntAction {
    None,
    SetTarget(u32), // target_vid
    MoveTo(f32, f32), // x, y
    StartAttack(u32), // target_vid
    ComboAttack(u32), // target_vid
    UseSkill(u32), // skill_id
}

#[derive(Debug, Clone)]
pub struct MonsterTarget {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub hp: u32,
    pub max_hp: u32,
    pub is_dead: bool,
}

#[derive(Debug, Clone)]
pub struct ConfiguredSkill {
    pub skill_id: u32,
    pub cooldown: Duration,
    pub last_used: Option<Instant>,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub x: f32,
    pub y: f32,
    pub is_dead: bool,
    pub is_sleeping: bool,
    pub is_charging_skill: bool,
    pub is_mounting_horse: bool,
    pub can_attack_horse_level: bool,
}

pub struct AutoHuntAiManager {
    pub state: AutoHuntState,
    pub target_vid: Option<u32>,
    pub combat_radius: f32,
    pub weapon_range: f32,
    pub configured_skills: Vec<ConfiguredSkill>,
}

impl Default for AutoHuntAiManager {
    fn default() -> Self {
        Self {
            state: AutoHuntState::Idle,
            target_vid: None,
            combat_radius: 1500.0,
            weapon_range: 200.0,
            configured_skills: Vec::new(),
        }
    }
}

impl AutoHuntAiManager {
    pub fn new(combat_radius: f32, weapon_range: f32) -> Self {
        Self {
            state: AutoHuntState::Idle,
            target_vid: None,
            combat_radius,
            weapon_range,
            configured_skills: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        if self.state == AutoHuntState::Idle {
            self.state = AutoHuntState::Search;
            self.target_vid = None;
        }
    }

    pub fn stop(&mut self) {
        self.state = AutoHuntState::Idle;
        self.target_vid = None;
    }

    pub fn add_skill(&mut self, skill_id: u32, cooldown: Duration) {
        self.configured_skills.push(ConfiguredSkill {
            skill_id,
            cooldown,
            last_used: None,
        });
    }

    fn calculate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    fn find_optimal_target<'a>(
        &self,
        player_x: f32,
        player_y: f32,
        monsters: &'a [MonsterTarget],
    ) -> Option<&'a MonsterTarget> {
        let mut best_target: Option<&MonsterTarget> = None;
        let mut best_score = f32::MAX;

        for monster in monsters {
            if monster.is_dead || monster.hp == 0 {
                continue;
            }

            let dist = Self::calculate_distance(player_x, player_y, monster.x, monster.y);
            if dist > self.combat_radius {
                continue;
            }

            // Score: primarily distance, but also prioritize lower HP to finish them off
            let hp_factor = monster.hp as f32 / monster.max_hp.max(1) as f32; // 0.0 to 1.0
            let score = dist + (hp_factor * 200.0); // Arbitrary weight for HP

            if score < best_score {
                best_score = score;
                best_target = Some(monster);
            }
        }

        best_target
    }

    pub fn update(
        &mut self,
        player: &PlayerState,
        monsters: &[MonsterTarget],
        current_time: Instant,
    ) -> AutoHuntAction {
        if player.is_dead {
            self.target_vid = None;
            self.state = AutoHuntState::Idle;
            return AutoHuntAction::None;
        }

        if player.is_charging_skill {
            return AutoHuntAction::None;
        }

        if player.is_sleeping {
            return AutoHuntAction::None;
        }

        if player.is_mounting_horse && !player.can_attack_horse_level {
            self.target_vid = None;
            self.state = AutoHuntState::Search;
            return AutoHuntAction::None;
        }

        match self.state {
            AutoHuntState::Idle => AutoHuntAction::None,
            AutoHuntState::Search => {
                if let Some(target) = self.find_optimal_target(player.x, player.y, monsters) {
                    self.target_vid = Some(target.vid);
                    self.state = AutoHuntState::Approach;
                    return AutoHuntAction::SetTarget(target.vid);
                }
                AutoHuntAction::None
            }
            AutoHuntState::Approach => {
                let target_vid = if let Some(vid) = self.target_vid {
                    vid
                } else {
                    self.state = AutoHuntState::Search;
                    return AutoHuntAction::None;
                };

                let target_info = monsters.iter().find(|m| m.vid == target_vid);

                match target_info {
                    Some(monster) => {
                        if monster.is_dead || monster.hp == 0 {
                            self.target_vid = None;
                            self.state = AutoHuntState::Search;
                            return AutoHuntAction::None;
                        }

                        let dist = Self::calculate_distance(player.x, player.y, monster.x, monster.y);
                        
                        if dist <= self.weapon_range {
                            self.state = AutoHuntState::Attack;
                            return AutoHuntAction::StartAttack(monster.vid);
                        } else {
                            return AutoHuntAction::MoveTo(monster.x, monster.y);
                        }
                    }
                    None => {
                        // Target lost
                        self.target_vid = None;
                        self.state = AutoHuntState::Search;
                        return AutoHuntAction::None;
                    }
                }
            }
            AutoHuntState::Attack => {
                let target_vid = if let Some(vid) = self.target_vid {
                    vid
                } else {
                    self.state = AutoHuntState::Search;
                    return AutoHuntAction::None;
                };

                let target_info = monsters.iter().find(|m| m.vid == target_vid);

                match target_info {
                    Some(monster) => {
                        if monster.is_dead || monster.hp == 0 {
                            self.target_vid = None;
                            self.state = AutoHuntState::Search;
                            return AutoHuntAction::None;
                        }

                        let dist = Self::calculate_distance(player.x, player.y, monster.x, monster.y);
                        
                        if dist > self.weapon_range {
                            self.state = AutoHuntState::Approach;
                            return AutoHuntAction::MoveTo(monster.x, monster.y);
                        }

                        // Check if we can use a skill
                        for skill in &mut self.configured_skills {
                            let can_use = match skill.last_used {
                                Some(last) => current_time.duration_since(last) >= skill.cooldown,
                                None => true,
                            };

                            if can_use {
                                skill.last_used = Some(current_time);
                                return AutoHuntAction::UseSkill(skill.skill_id);
                            }
                        }

                        // Otherwise just combo attack
                        AutoHuntAction::ComboAttack(monster.vid)
                    }
                    None => {
                         // Target lost
                         self.target_vid = None;
                         self.state = AutoHuntState::Search;
                         return AutoHuntAction::None;
                    }
                }
            }
        }
    }
}
