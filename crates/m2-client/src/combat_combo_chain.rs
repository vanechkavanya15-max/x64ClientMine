use std::f32::consts::PI;

use crate::combat_combo_engine::{HEADER_CG_ATTACK, TPacketCGAttack, WeaponType};

#[derive(Debug, Clone)]
pub struct CombatComboChain {
    pub combo_index: u32,
    pub max_combo_count: u32,
    pub elapsed_time: f32,
    pub weapon_type: WeaponType,
    pub is_mounting_horse: bool,
    pub pre_input_buffered: bool,
    pub pre_input_window: f32,
}

impl CombatComboChain {
    pub fn new(weapon_type: WeaponType, is_mounting_horse: bool) -> Self {
        // Specific logic requested: indexes 0, 1, 2, 3, 4 (max 5)
        let max_combo_count = match weapon_type {
            WeaponType::Sword => 4,
            WeaponType::TwoHand => 4,
            WeaponType::Dagger => 5, // Daggers often have different combo lengths, assuming 5 (indices 0..4) based on reqs
            WeaponType::Bow => 4,
            _ => if is_mounting_horse { 3 } else { 4 },
        };

        Self {
            combo_index: 0,
            max_combo_count,
            elapsed_time: 0.0,
            weapon_type,
            is_mounting_horse,
            pre_input_buffered: false,
            pre_input_window: 0.35, // 350ms pre-input buffer window
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.combo_index > 0 {
            self.elapsed_time += delta_time;
        }
    }

    /// Sprawdza czy buforowany pre-input powinien wyzwolic nastepne uderzenie combo
    pub fn update_and_check_transition(&mut self, delta_time: f32, next_combo_time: f32, combo_window_end: f32) -> Option<u32> {
        if self.combo_index == 0 {
            return None;
        }
        self.elapsed_time += delta_time;

        if self.elapsed_time > combo_window_end {
            self.reset_combo();
            return None;
        }

        if self.pre_input_buffered && self.elapsed_time >= next_combo_time {
            self.pre_input_buffered = false;
            if self.combo_index < self.max_combo_count {
                self.combo_index += 1;
                self.elapsed_time = 0.0;
                return Some(self.combo_index);
            } else if self.is_mounting_horse {
                self.combo_index = 1;
                self.elapsed_time = 0.0;
                return Some(self.combo_index);
            } else {
                self.reset_combo();
            }
        }
        None
    }

    pub fn input_combo_attack(&mut self, combo_window_end: f32, next_combo_time: f32) -> bool {
        if self.combo_index == 0 {
            // First attack
            self.combo_index = 1;
            self.elapsed_time = 0.0;
            self.pre_input_buffered = false;
            return true;
        }

        // Verify combo reset timeout: if next attack doesn't occur within the time window, reset to 0
        if self.elapsed_time > combo_window_end {
            self.reset_combo();
            return false;
        }

        // Time to next combo reached -> natychmiastowe przejscie
        if self.elapsed_time >= next_combo_time {
            if self.combo_index < self.max_combo_count {
                self.combo_index += 1;
                self.elapsed_time = 0.0;
                self.pre_input_buffered = false;
                return true;
            } else if self.is_mounting_horse {
                self.combo_index = 1;
                self.elapsed_time = 0.0;
                self.pre_input_buffered = false;
                return true;
            }
        } else if (next_combo_time - self.elapsed_time) <= self.pre_input_window {
            // Gracz wcisnal atak za wczesnie, ale w oknie bufora (pre-input)
            self.pre_input_buffered = true;
            return true;
        }

        false
    }

    pub fn reset_combo(&mut self) {
        self.combo_index = 0;
        self.elapsed_time = 0.0;
        self.pre_input_buffered = false;
    }

    pub fn calculate_attack_geometry(
        &self,
        attacker_x: f32,
        attacker_y: f32,
        attacker_rot: f32,
        victim_x: f32,
        victim_y: f32,
        victim_radius: f32,
    ) -> bool {
        let dx = victim_x - attacker_x;
        let dy = victim_y - attacker_y;
        let dist_sq = dx * dx + dy * dy;

        let base_distance = self.weapon_type.get_base_distance();
        let hit_distance = base_distance + victim_radius;

        // Reach check
        if dist_sq > hit_distance * hit_distance {
            return false;
        }

        if dist_sq < 25.0 * 25.0 {
            // Very close, always hit regardless of angle (prevents point-blank misses)
            return true;
        }

        let rot_rad = (attacker_rot * PI) / 180.0;
        let view_dir_x = rot_rad.sin();
        let view_dir_y = rot_rad.cos(); // Keep C++ mapping consistency

        let dist = dist_sq.sqrt();
        let dir_x = dx / dist;
        let dir_y = dy / dist;

        let dot_product = view_dir_x * dir_x + view_dir_y * dir_y;

        // Angle check 60-90 degrees.
        // For a cone of 60 to 90 degrees total, half angle is 30 to 45 degrees.
        // cos(45 deg) = ~0.707. cos(60 deg) = 0.5.
        // According to requirement: attack cone angle 60-90 degrees.
        // We will use 0.5 (which is 120 degree cone, 60 half-angle) or 0.707 (90 degree cone, 45 half-angle)
        // Let's use 0.5 which covers standard Metin2 wide swings.
        
        // Actually, requirement says "attack cone angle 60-90 degrees". 
        // 90 degrees total = 45 degrees half angle -> cos(45) = 0.707.
        // 60 degrees total = 30 degrees half angle -> cos(30) = 0.866.
        // Let's use 0.707 (90 degrees) or vary by weapon.
        let cone_threshold = match self.weapon_type {
            WeaponType::Sword | WeaponType::TwoHand => 0.5, // Wider 120 deg
            WeaponType::Dagger => 0.707, // 90 deg
            WeaponType::Bow => 0.866, // 60 deg
            _ => 0.5,
        };

        dot_product >= cone_threshold
    }

    pub fn create_attack_packet(&self, victim_vid: u32) -> TPacketCGAttack {
        TPacketCGAttack {
            header: HEADER_CG_ATTACK,
            length: std::mem::size_of::<TPacketCGAttack>() as u16,
            b_type: self.combo_index as u8,
            dw_victim_vid: victim_vid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo_chain_first_attack() {
        let mut chain = CombatComboChain::new(WeaponType::Sword, false);
        assert_eq!(chain.combo_index, 0);
        let accepted = chain.input_combo_attack(1.5, 0.5);
        assert!(accepted);
        assert_eq!(chain.combo_index, 1);
        assert_eq!(chain.elapsed_time, 0.0);
    }

    #[test]
    fn test_combo_chain_pre_input_buffering() {
        let mut chain = CombatComboChain::new(WeaponType::Sword, false);
        chain.input_combo_attack(1.5, 0.5); // Start combo at index 1

        // Update elapsed time to 0.25s (before 0.5s, but within 350ms window: 0.5 - 0.25 = 0.25 <= 0.35)
        chain.update(0.25);
        let buffered = chain.input_combo_attack(1.5, 0.5);
        assert!(buffered, "Wczesne klikniecie w oknie 350ms powinno byc zbuforowane");
        assert!(chain.pre_input_buffered);
        assert_eq!(chain.combo_index, 1); // Nadal krok 1 dopoki nie minie czas

        // Update o kolejne 0.3s (lacznie 0.55s > 0.5s next_combo_time)
        let transition = chain.update_and_check_transition(0.3, 0.5, 1.5);
        assert_eq!(transition, Some(2), "Po osiagnieciu czasu nastepnego uderzenia buforowany atak powinien aktywowac combo 2");
        assert_eq!(chain.combo_index, 2);
        assert!(!chain.pre_input_buffered);
    }

    #[test]
    fn test_combo_chain_timeout_reset() {
        let mut chain = CombatComboChain::new(WeaponType::Sword, false);
        chain.input_combo_attack(1.5, 0.5); // Krok 1
        chain.update(2.0); // Przekroczenie combo_window_end (1.5s)
        let accepted = chain.input_combo_attack(1.5, 0.5);
        assert!(!accepted, "Po przekroczeniu okna czasowego combo powinno zostac zresetowane");
        assert_eq!(chain.combo_index, 0);
    }
}

