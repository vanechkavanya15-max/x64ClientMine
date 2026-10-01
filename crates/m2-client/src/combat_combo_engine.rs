use std::f32::consts::PI;
use std::mem::size_of;
use std::ptr;

pub const HEADER_CG_ATTACK: u16 = 0x0401; // As per the reference C++ source code CG::ATTACK

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGAttack {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub dw_victim_vid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Sword,
    TwoHand,
    Dagger,
    Bow,
    Bell,
    Fan,
    Unarmed,
}

impl WeaponType {
    pub fn get_base_distance(&self) -> f32 {
        match self {
            WeaponType::Sword => 150.0,
            WeaponType::TwoHand => 200.0,
            WeaponType::Dagger => 100.0,
            WeaponType::Bow => 2000.0,
            WeaponType::Bell => 150.0,
            WeaponType::Fan => 150.0,
            WeaponType::Unarmed => 100.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComboTimeData {
    pub combo_window_start: f32,
    pub next_combo_time: f32,
    pub combo_window_end: f32,
    pub motion_duration: f32,
    pub has_time_data: bool,
}

#[derive(Debug, Clone)]
pub struct CombatComboEngine {
    pub cur_combo_index: u32,
    pub max_combo_count: u32,
    pub is_pre_input: bool,
    pub is_mounting_horse: bool,
    pub elapsed_time: f32,
    pub time_data: Option<ComboTimeData>,
    pub attack_dir_rot: f32,
    pub weapon_type: WeaponType,
}

pub struct EnemyTarget {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct AttackResult {
    pub packets: Vec<Vec<u8>>,
    pub pushback_targets: Vec<u32>,
}

impl CombatComboEngine {
    pub fn new(is_mounting_horse: bool, weapon_type: WeaponType) -> Self {
        let max_combo_count = if is_mounting_horse { 3 } else { 4 };
        Self {
            cur_combo_index: 0,
            max_combo_count,
            is_pre_input: false,
            is_mounting_horse,
            elapsed_time: 0.0,
            time_data: None,
            attack_dir_rot: 0.0,
            weapon_type,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.cur_combo_index == 0 {
            return;
        }

        self.elapsed_time += delta_time;

        if let Some(ref data) = self.time_data {
            if data.has_time_data {
                if self.elapsed_time > data.combo_window_end {
                    self.clear_combo();
                } else if self.elapsed_time > data.next_combo_time {
                    if self.is_pre_input {
                        self.run_next_combo();
                    }
                }
            } else if self.elapsed_time > data.motion_duration * 0.9 {
                if self.is_pre_input {
                    self.run_next_combo();
                }
            }
        }
    }

    pub fn input_attack(&mut self, dir_rot: f32) -> bool {
        self.attack_dir_rot = dir_rot;

        if self.is_pre_input {
            return false;
        }

        if !self.is_mounting_horse && self.cur_combo_index >= self.max_combo_count {
            return false;
        }

        if self.cur_combo_index == 0 {
            self.run_next_combo();
            return true;
        }

        if let Some(ref data) = self.time_data {
            if data.has_time_data {
                if self.elapsed_time > data.combo_window_end {
                    return false;
                }

                if self.elapsed_time > data.next_combo_time {
                    self.run_next_combo();
                    return true;
                } else if self.elapsed_time > data.combo_window_start {
                    self.is_pre_input = true;
                    return false;
                }
            } else if self.elapsed_time > data.motion_duration * 0.9 {
                self.run_next_combo();
                return true;
            }
        }

        false
    }

    fn run_next_combo(&mut self) {
        if self.is_mounting_horse && self.cur_combo_index >= self.max_combo_count {
            self.cur_combo_index = 1;
        } else {
            self.cur_combo_index += 1;
        }

        self.elapsed_time = 0.0;
        self.is_pre_input = false;
    }

    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.elapsed_time = 0.0;
    }

    pub fn process_attack_geometry(
        &self,
        attacker_x: f32,
        attacker_y: f32,
        enemies: &[EnemyTarget],
    ) -> AttackResult {
        let mut packets = Vec::new();
        let mut pushback_targets = Vec::new();

        let weapon_dist = self.weapon_type.get_base_distance();

        // Calculate view direction vector (Metin2 standard rotations)
        // Adjust standard rotations depending on axis mapping, typically angle in degrees mapped to radians
        let rot_rad = (self.attack_dir_rot * PI) / 180.0;
        
        // C++ Uses sin and cos for normal directions
        // D3DXVECTOR2 v2Normal(sin(fRotRad), cos(fRotRad));
        let view_dir_x = rot_rad.sin();
        let view_dir_y = rot_rad.cos(); // C++ does cos for Y/Z logic in ActorInstanceBattle, we stick to 2D math

        // 120 degree cone -> half angle is 60 degrees. cos(60) = 0.5
        let cone_threshold = 0.5;

        let is_finisher = !self.is_mounting_horse && self.cur_combo_index == self.max_combo_count;

        for enemy in enemies {
            let dx = enemy.x - attacker_x;
            let dy = enemy.y - attacker_y;
            let dist_sq = dx * dx + dy * dy;

            let hit_distance = weapon_dist + enemy.radius;

            if dist_sq <= hit_distance * hit_distance {
                // If it's a very short distance, it's a hit regardless of cone to avoid misses at point blank
                if dist_sq < 25.0 * 25.0 {
                    self.add_hit(enemy, is_finisher, &mut packets, &mut pushback_targets);
                    continue;
                }

                let dist = dist_sq.sqrt();
                let dir_x = dx / dist;
                let dir_y = dy / dist;

                let dot_product = view_dir_x * dir_x + view_dir_y * dir_y;

                if dot_product >= cone_threshold {
                    self.add_hit(enemy, is_finisher, &mut packets, &mut pushback_targets);
                }
            }
        }

        AttackResult {
            packets,
            pushback_targets,
        }
    }

    fn add_hit(
        &self,
        enemy: &EnemyTarget,
        is_finisher: bool,
        packets: &mut Vec<Vec<u8>>,
        pushback_targets: &mut Vec<u32>,
    ) {
        let packet = TPacketCGAttack {
            header: HEADER_CG_ATTACK,
            length: size_of::<TPacketCGAttack>() as u16,
            b_type: self.cur_combo_index as u8,
            dw_victim_vid: enemy.vid,
        };

        let mut buf = vec![0u8; size_of::<TPacketCGAttack>()];
        unsafe {
            ptr::copy_nonoverlapping(
                &packet as *const TPacketCGAttack as *const u8,
                buf.as_mut_ptr(),
                size_of::<TPacketCGAttack>(),
            );
        }

        packets.push(buf);

        if is_finisher {
            pushback_targets.push(enemy.vid);
        }
    }
}