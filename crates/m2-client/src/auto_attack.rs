#[repr(C)]
#[derive(Debug, Clone)]
pub struct Actor {
    pub vid: u32,
    pub name: String,
    pub race: u16,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub is_pc: bool,
    pub is_enemy: bool,
}

impl Actor {
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

/// Oblicza przyblizony dystans metoda uzywana w kliencie Metin2.
/// Zoptymalizowane uzycie przesuniec bitowych zamiast sqrt.
pub fn distance_approx(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let max = dx.max(dy);
    let min = dx.min(dy);
    ((max << 8) + (max << 3) - (max << 4) - (max << 1) + (min << 7) - (min << 5) + (min << 3) - (min << 1)) >> 8
}

#[derive(Debug, PartialEq, Eq)]
pub struct AutoAttackResult {
    pub need_approach: bool,
    pub trigger_strike: bool,
    pub clear_target: bool,
}

pub struct AutoAttackManager {
    pub is_auto_attack_enabled: bool,
    pub target_vid: Option<u32>,
    pub weapon_range: i32,
    pub attack_cooldown: f32,
    pub time_since_last_attack: f32,
}

impl AutoAttackManager {
    pub fn new(weapon_range: i32, attack_cooldown: f32) -> Self {
        Self {
            is_auto_attack_enabled: false,
            target_vid: None,
            weapon_range,
            attack_cooldown,
            time_since_last_attack: 0.0,
        }
    }

    pub fn set_target(&mut self, vid: u32) {
        self.target_vid = Some(vid);
        self.is_auto_attack_enabled = true;
    }

    pub fn clear_target(&mut self) {
        self.target_vid = None;
        self.is_auto_attack_enabled = false;
        self.time_since_last_attack = 0.0;
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        main_actor: &Actor,
        target_actor: Option<&Actor>,
    ) -> AutoAttackResult {
        let mut result = AutoAttackResult {
            need_approach: false,
            trigger_strike: false,
            clear_target: false,
        };

        if !self.is_auto_attack_enabled {
            return result;
        }

        let Some(target) = target_actor else {
            self.clear_target();
            result.clear_target = true;
            return result;
        };

        if Some(target.vid) != self.target_vid {
            self.clear_target();
            result.clear_target = true;
            return result;
        }

        if target.is_dead() {
            self.clear_target();
            result.clear_target = true;
            return result;
        }

        self.time_since_last_attack += delta_time;

        let distance = distance_approx(main_actor.x, main_actor.y, target.x, target.y);

        if distance > self.weapon_range {
            result.need_approach = true;
            return result;
        }

        if self.time_since_last_attack >= self.attack_cooldown {
            result.trigger_strike = true;
            self.time_since_last_attack = 0.0;
        }

        result
    }
}
