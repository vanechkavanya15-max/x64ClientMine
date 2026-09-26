#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleState {
    Wait,
    NormalAttack,
    ComboAttack,
    UseSkill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorCondition {
    Normal,
    Dead,
    Stunned,
    Frozen,
    Sleeping,
}

pub struct InstanceBattle {
    pub state: BattleState,
    pub condition: ActorCondition,
    
    // Configurable parameters
    pub attack_cooldown_ms: u32,
    pub combo_window_start_ms: u32,
    pub combo_window_end_ms: u32,
    
    // Runtime state
    pub last_attack_time_ms: u32,
    pub current_rotation_deg: f32,
    pub combo_index: u32,
    pub is_pre_input: bool,
}

impl InstanceBattle {
    pub fn new() -> Self {
        Self {
            state: BattleState::Wait,
            condition: ActorCondition::Normal,
            attack_cooldown_ms: 500, // Default 500ms for testing
            combo_window_start_ms: 300,
            combo_window_end_ms: 700,
            last_attack_time_ms: 0,
            current_rotation_deg: 0.0,
            combo_index: 0,
            is_pre_input: false,
        }
    }

    pub fn can_act(&self) -> bool {
        match self.condition {
            ActorCondition::Dead | ActorCondition::Stunned | ActorCondition::Frozen | ActorCondition::Sleeping => false,
            ActorCondition::Normal => true,
        }
    }

    pub fn can_attack(&self, cur_time_ms: u32) -> bool {
        if !self.can_act() {
            return false;
        }

        if self.state == BattleState::UseSkill {
            return false;
        }

        // Jeśli postać właśnie atakuje (lub nie minął cooldown z poprzedniego ataku)
        // W normalnym przypadku C++ CanAttack nie sprawdzało stricte cooldownu ataku w tym samym wywołaniu co blokada skilli
        // Ale upewniamy się, że od last_attack_time_ms upłynęło wystarczająco dużo czasu dla NOWEGO ataku bez combo,
        // Albo dla combo
        
        let elapsed = cur_time_ms.saturating_sub(self.last_attack_time_ms);
        
        // Zabezpieczenie przed atakiem przed upłynięciem połowy normalnego ataku,
        // jednak szczegółowa walidacja czasu ataku leży po stronie wywołania Process/Input
        if self.state == BattleState::NormalAttack || self.state == BattleState::ComboAttack {
            if elapsed < self.attack_cooldown_ms / 2 {
                return false;
            }
        }
        
        true
    }

    pub fn process_normal_attack(&mut self, cur_time_ms: u32, rot_deg: f32) -> bool {
        if !self.can_attack(cur_time_ms) {
            return false;
        }
        
        // Czas od ostatniego ataku
        let elapsed = cur_time_ms.saturating_sub(self.last_attack_time_ms);
        
        // Jeżeli nie atakujemy i minął pełny cooldown LUB jest to pierwszy atak
        if self.state == BattleState::Wait || elapsed >= self.attack_cooldown_ms {
            self.state = BattleState::NormalAttack;
            self.current_rotation_deg = rot_deg;
            self.last_attack_time_ms = cur_time_ms;
            self.combo_index = 0;
            self.is_pre_input = false;
            return true;
        }

        false
    }

    pub fn input_combo_attack_command(&mut self, cur_time_ms: u32, dir_rot: f32) -> bool {
        if !self.can_act() {
            return false;
        }

        if self.is_pre_input {
            return false;
        }

        self.current_rotation_deg = dir_rot;

        if self.combo_index == 0 && self.state != BattleState::NormalAttack {
            // Pierwszy cios traktujemy jak normalny atak
            let success = self.process_normal_attack(cur_time_ms, dir_rot);
            if success {
                self.combo_index = 1;
            }
            return success;
        }

        // Czas od ostatniego ataku
        let elapsed = cur_time_ms.saturating_sub(self.last_attack_time_ms);

        if elapsed > self.combo_window_end_ms {
            // Minęło okno combo, pre_input zostaje pominięty lub atak wygasł
            return false;
        }

        if elapsed >= self.combo_window_start_ms && elapsed <= self.combo_window_end_ms {
            // Jesteśmy w oknie combo, poprawny pre-input
            self.is_pre_input = true;
            self.combo_index += 1;
            
            // W zależności od implementacji, tutaj możemy od razu wyzwolić cios, ale pre-input
            // oznacza zbuforowanie. Skrótowo, symulujemy uruchomienie ciosu.
            self.state = BattleState::ComboAttack;
            // Aktualizujemy czas ataku
            self.last_attack_time_ms = cur_time_ms;
            self.is_pre_input = false; // "Zjedzone" przez maszynę
            
            return true;
        }

        false
    }
}
