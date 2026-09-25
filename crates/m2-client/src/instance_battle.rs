//! Modul `instance_battle` — System walki, sekwencji combo i detekcji trafien
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseBattle.cpp oraz source/GameLib/ActorInstanceBattle.cpp`
//! Opis: Odwzorowanie ProcessNormalAttack i InputComboAttackCommand: sekwencja 4 uderzen combo, promien zasiegu broni, kat trafienia stożkowego i wysylanie ciosow.

use std::f32::consts::PI;

#[derive(Debug, Clone, Default)]
pub struct InstanceBattle {
    pub initialized: bool,
    pub cur_combo_index: u32,
    pub is_pre_input: bool,
    pub is_next_pre_input: bool,
    pub atk_dir_rot: f32,
    pub combo_type: u16,
}

impl InstanceBattle {
    pub fn new() -> Self {
        Self {
            initialized: true,
            cur_combo_index: 0,
            is_pre_input: false,
            is_next_pre_input: false,
            atk_dir_rot: 0.0,
            combo_type: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn input_normal_attack_command(&mut self, dir_rot: f32) {
        if !self.can_input_normal_attack_command() {
            return;
        }
        self.atk_dir_rot = dir_rot;
        self.normal_attack(self.atk_dir_rot);
    }

    pub fn input_combo_attack_command(&mut self, dir_rot: f32, is_combo_input_time_data: bool) -> bool {
        self.atk_dir_rot = dir_rot;

        if self.is_pre_input {
            return false;
        }

        if self.cur_combo_index == 0 {
            self.run_next_combo();
            return true;
        } else if is_combo_input_time_data {
            self.is_pre_input = true;
            return true;
        }

        false
    }

    pub fn combo_process(&mut self, elapsed_time: f32, next_combo_time: f32, has_motion_data: bool, is_using_skill: bool) {
        if self.cur_combo_index != 0 {
            if !has_motion_data {
                self.clear_combo();
                return;
            }

            if self.is_pre_input {
                if elapsed_time > next_combo_time {
                    self.run_next_combo();
                    self.is_pre_input = false;
                    return;
                }
            }
        } else {
            self.is_pre_input = false;

            if !is_using_skill {
                if self.is_next_pre_input {
                    self.run_next_combo();
                    self.is_next_pre_input = false;
                }
            }
        }
    }

    pub fn run_next_combo(&mut self) {
        self.cur_combo_index += 1;
        
        // Zaleznie od poziomu combo (combo_type), gracz uderza 4, 5 lub 6 razy
        let max_combo = match self.combo_type {
            0 => 4,
            1 => 5,
            2 => 6,
            _ => 4,
        };

        if self.cur_combo_index > max_combo {
            self.on_end_combo();
        }
    }

    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.is_next_pre_input = false;
    }

    pub fn on_end_combo(&mut self) {
        self.clear_combo();
    }

    pub fn can_input_normal_attack_command(&self) -> bool {
        true
    }

    pub fn normal_attack(&mut self, _dir_rot: f32) -> bool {
        true
    }

    pub fn process_normal_attack(
        &self,
        attacker_pos: (f32, f32, f32),
        attacker_rot_deg: f32,
        victim_pos: (f32, f32, f32),
        victim_is_huge: bool,
        reach_scale: f32,
        attack_radius_base: f32,
        hit_angle_limit_deg: f32,
    ) -> bool {
        let (ax, ay, az) = attacker_pos;
        let (vx, vy, vz) = victim_pos;

        let dx = vx - ax;
        let dy = vy - ay;
        let dz = vz - az;
        let distance_sq = dx * dx + dy * dy + dz * dz;

        // Odrzucenie jesli dystans przekracza limit zalezny od rozmiaru ofiary
        let max_dist_sq = if victim_is_huge {
            500.0 * 500.0
        } else {
            300.0 * 300.0
        };

        if distance_sq >= max_dist_sq {
            return false;
        }

        let attacker_rot_rad = attacker_rot_deg * (PI / 180.0);
        
        let dir_x = attacker_rot_rad.sin();
        let dir_y = attacker_rot_rad.cos();
        
        let dist_2d = (dx * dx + dy * dy).sqrt();
        if dist_2d == 0.0 {
            return true;
        }
        
        let to_victim_x = dx / dist_2d;
        let to_victim_y = dy / dist_2d;
        
        let dot_product = dir_x * to_victim_x + dir_y * to_victim_y;
        
        let angle_limit_rad = hit_angle_limit_deg * (PI / 180.0);
        let dot_limit = angle_limit_rad.cos();
        
        // Sprawdzamy czy cel znajduje sie w stozku trafienia
        if dot_product < dot_limit {
            return false;
        }
        
        // Sprawdzamy promien zasiegu broni
        let attack_radius = attack_radius_base * reach_scale;
        if dist_2d > attack_radius {
            return false;
        }

        true
    }
}
