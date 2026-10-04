use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoHuntState {
    Idle,
    Search,
    Approach,
    Attack,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoHuntAction {
    None,
    MoveTo(Vec2),
    Attack(u32), // VID of the target
    UseSkill(u32, u32), // skill_id, target_vid
}

#[derive(Debug, Clone)]
pub struct AutoHuntActorInfo {
    pub vid: u32,
    pub position: Vec2,
    pub is_monster: bool,
    pub is_dead: bool,
    pub hp_percent: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct AutoHuntConfig {
    pub scan_radius: f32,
    pub attack_range: f32,
    pub use_skills: bool,
    pub skill_cooldown_ms: u64, // simplified skill cooldown for AI logic
}

impl Default for AutoHuntConfig {
    fn default() -> Self {
        Self {
            scan_radius: 1500.0,
            attack_range: 200.0, // typical melee range
            use_skills: false,
            skill_cooldown_ms: 3000,
        }
    }
}

pub struct AutoHuntAi {
    state: AutoHuntState,
    config: AutoHuntConfig,
    target_vid: Option<u32>,
    last_skill_use_time_ms: u64,
}

impl AutoHuntAi {
    pub fn new(config: AutoHuntConfig) -> Self {
        Self {
            state: AutoHuntState::Idle,
            config,
            target_vid: None,
            last_skill_use_time_ms: 0,
        }
    }

    pub fn get_state(&self) -> AutoHuntState {
        self.state
    }

    pub fn set_state(&mut self, state: AutoHuntState) {
        self.state = state;
    }

    pub fn get_target_vid(&self) -> Option<u32> {
        self.target_vid
    }

    pub fn clear_target(&mut self) {
        self.target_vid = None;
    }

    pub fn update(
        &mut self,
        current_time_ms: u64,
        my_pos: Vec2,
        surrounding_actors: &[AutoHuntActorInfo],
    ) -> AutoHuntAction {
        match self.state {
            AutoHuntState::Idle => {
                // If in idle and activated, move to Search
                self.state = AutoHuntState::Search;
                AutoHuntAction::None
            }
            AutoHuntState::Search => {
                // Find best target
                let best_target = self.find_best_target(my_pos, surrounding_actors);
                
                if let Some(target) = best_target {
                    self.target_vid = Some(target.vid);
                    self.state = AutoHuntState::Approach;
                    // We don't immediately return MoveTo here, we let the Approach state handle it next frame
                    // Or we can return it now. For simplicity, we just change state and wait next tick.
                    // But returning action immediately makes AI more responsive.
                    // Let's just return None and let it evaluate in the next update.
                    AutoHuntAction::None
                } else {
                    AutoHuntAction::None
                }
            }
            AutoHuntState::Approach => {
                if let Some(target_vid) = self.target_vid {
                    if let Some(target) = surrounding_actors.iter().find(|a| a.vid == target_vid) {
                        if target.is_dead {
                            self.clear_target();
                            self.state = AutoHuntState::Search;
                            return AutoHuntAction::None;
                        }

                        let dist_sq = my_pos.distance_squared(target.position);
                        if dist_sq <= self.config.attack_range * self.config.attack_range {
                            self.state = AutoHuntState::Attack;
                            AutoHuntAction::None // Evaluated next tick
                        } else {
                            AutoHuntAction::MoveTo(target.position)
                        }
                    } else {
                        // Target lost
                        self.clear_target();
                        self.state = AutoHuntState::Search;
                        AutoHuntAction::None
                    }
                } else {
                    self.state = AutoHuntState::Search;
                    AutoHuntAction::None
                }
            }
            AutoHuntState::Attack => {
                if let Some(target_vid) = self.target_vid {
                    if let Some(target) = surrounding_actors.iter().find(|a| a.vid == target_vid) {
                        if target.is_dead {
                            self.clear_target();
                            self.state = AutoHuntState::Search;
                            return AutoHuntAction::None;
                        }

                        let dist_sq = my_pos.distance_squared(target.position);
                        if dist_sq > self.config.attack_range * self.config.attack_range {
                            // Target moved away, approach again
                            self.state = AutoHuntState::Approach;
                            return AutoHuntAction::None;
                        }

                        // We are in range. Try to use skill first, otherwise normal attack
                        if self.config.use_skills && current_time_ms >= self.last_skill_use_time_ms + self.config.skill_cooldown_ms {
                            self.last_skill_use_time_ms = current_time_ms;
                            AutoHuntAction::UseSkill(1, target.vid) // Using 1 as a dummy skill ID for now
                        } else {
                            AutoHuntAction::Attack(target.vid)
                        }
                    } else {
                        // Target lost
                        self.clear_target();
                        self.state = AutoHuntState::Search;
                        AutoHuntAction::None
                    }
                } else {
                    self.state = AutoHuntState::Search;
                    AutoHuntAction::None
                }
            }
        }
    }

    fn find_best_target<'a>(
        &self,
        my_pos: Vec2,
        surrounding_actors: &'a [AutoHuntActorInfo],
    ) -> Option<&'a AutoHuntActorInfo> {
        let mut best_target: Option<&AutoHuntActorInfo> = None;
        let mut best_score = f32::MAX;
        
        let scan_rad_sq = self.config.scan_radius * self.config.scan_radius;

        for actor in surrounding_actors {
            if !actor.is_monster || actor.is_dead {
                continue;
            }

            let dist_sq = my_pos.distance_squared(actor.position);
            if dist_sq <= scan_rad_sq {
                // Score metric: Lower is better. Distance + HP factor.
                // Prioritize closer enemies, and enemies with lower HP.
                // For instance, distance dominates, but if HP is low, it might be more attractive.
                let dist = dist_sq.sqrt();
                let hp_penalty = actor.hp_percent * 500.0; // 0.0 to 500.0 points
                
                let score = dist + hp_penalty;

                if score < best_score {
                    best_score = score;
                    best_target = Some(actor);
                }
            }
        }

        best_target
    }
}
