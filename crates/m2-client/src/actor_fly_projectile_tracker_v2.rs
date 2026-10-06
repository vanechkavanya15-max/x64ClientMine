use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

/// C++ CFlyTarget::EType and corresponding data
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlyTarget {
    None,
    Object(u32), // In Rust, we use VID instead of pointer to IFlyTargetableObject
    Position(f32, f32, f32),
}

impl Default for FlyTarget {
    fn default() -> Self {
        Self::None
    }
}

/// Equivalent of C++ CFlyingData
#[derive(Debug, Clone, PartialEq)]
pub struct FlyingData {
    pub init_vel: f32,
    pub gravity: f32,
    pub is_homing: bool,
    pub homing_max_angle: f32,
    pub homing_start_time: f32,
    pub accel: (f32, f32, f32),
    pub range: f32,
    pub bomb_range: f32,
    pub collision_sphere_radius: f32,
    pub pierce_count: i32,
    pub hit_on_background: bool,
    pub hit_on_another_monster: bool,
    pub maintain_parallel: bool,
}

impl Default for FlyingData {
    fn default() -> Self {
        Self {
            init_vel: 200.0,
            gravity: 0.0,
            is_homing: false,
            homing_max_angle: 3.0,
            homing_start_time: 0.0,
            accel: (0.0, 0.0, 0.0),
            range: 500.0,
            bomb_range: 10.0,
            collision_sphere_radius: 0.0,
            pierce_count: 0,
            hit_on_background: false,
            hit_on_another_monster: false,
            maintain_parallel: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlyingStatus {
    Flying,
    HitTarget,
    HitBackground,
    HitAnotherMonster,
    OutOfRange,
}

/// Equivalent of CFlyingInstance
#[derive(Debug, Clone)]
pub struct FlyingInstance {
    pub id: u32,
    pub data: FlyingData,
    pub pos: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub accel: (f32, f32, f32),
    pub target: FlyTarget,
    pub is_alive: bool,
    pub can_attack: bool,
    pub owner_vid: Option<u32>,
    pub skill_index: u32,

    pub remain_range: f32,
    pub start_time: f32,
    pub current_time: f32,
    
    pub pierce_count: i32,
    pub target_hitted: bool,
    pub hitted_object_set: HashSet<u32>,
}

impl FlyingInstance {
    pub fn new(id: u32, data: FlyingData, start_pos: (f32, f32, f32), target: FlyTarget, can_attack: bool) -> Self {
        let accel = data.accel;
        let mut instance = Self {
            id,
            data: data.clone(),
            pos: start_pos,
            velocity: (0.0, 0.0, 0.0), // Will be initialized by target direction
            accel,
            target,
            is_alive: true,
            can_attack,
            owner_vid: None,
            skill_index: 0,
            remain_range: data.range,
            start_time: 0.0,
            current_time: 0.0,
            pierce_count: data.pierce_count,
            target_hitted: false,
            hitted_object_set: HashSet::new(),
        };

        instance.set_target_direction();
        instance
    }

    pub fn set_owner(&mut self, owner_vid: u32) {
        self.owner_vid = Some(owner_vid);
    }

    pub fn set_skill_index(&mut self, skill_index: u32) {
        self.skill_index = skill_index;
    }

    fn set_target_direction(&mut self) {
        let target_pos = match self.target {
            FlyTarget::Object(_) => (0.0, 0.0, 0.0), // Need external position fetcher in real update
            FlyTarget::Position(x, y, z) => (x, y, z),
            FlyTarget::None => return,
        };

        let mut t_pos = target_pos;
        if self.data.maintain_parallel {
            t_pos.2 += 50.0;
        }

        let dir = (t_pos.0 - self.pos.0, t_pos.1 - self.pos.1, t_pos.2 - self.pos.2);
        let len = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt();
        
        if len > 0.0 {
            let norm = (dir.0 / len, dir.1 / len, dir.2 / len);
            // In C++ it calculates Q and transforms velocity based on init_vel.
            // Simplified initialization:
            self.velocity = (norm.0 * self.data.init_vel, norm.1 * self.data.init_vel, norm.2 * self.data.init_vel);
        } else {
            self.velocity = (0.0, 0.0, self.data.init_vel);
        }
    }

    pub fn set_time(&mut self, current_time: f32) {
        if self.start_time == 0.0 {
            self.start_time = current_time;
        }
        self.current_time = current_time;
    }

    /// Replicates CFlyingInstance::AdjustDirectionForHoming
    pub fn adjust_direction_for_homing(&mut self, target_pos: (f32, f32, f32)) {
        let dir_x = target_pos.0 - self.pos.0;
        let dir_y = target_pos.1 - self.pos.1;
        let dir_z = target_pos.2 - self.pos.2;

        let mut target_dir = (dir_x, dir_y, dir_z);
        let target_len = (target_dir.0 * target_dir.0 + target_dir.1 * target_dir.1 + target_dir.2 * target_dir.2).sqrt();
        if target_len > 0.0 {
            target_dir.0 /= target_len;
            target_dir.1 /= target_len;
            target_dir.2 /= target_len;
        } else {
            return;
        }

        let mut vel_dir = self.velocity;
        let vel_len = (vel_dir.0 * vel_dir.0 + vel_dir.1 * vel_dir.1 + vel_dir.2 * vel_dir.2).sqrt();
        if vel_len > 0.0 {
            vel_dir.0 /= vel_len;
            vel_dir.1 /= vel_len;
            vel_dir.2 /= vel_len;
        } else {
            return;
        }

        let diff = (
            vel_dir.0 - target_dir.0,
            vel_dir.1 - target_dir.1,
            vel_dir.2 - target_dir.2,
        );
        
        let diff_len_sq = diff.0 * diff.0 + diff.1 * diff.1 + diff.2 * diff.2;
        if diff_len_sq < 0.001 {
            return;
        }

        let q = safe_rotation_normalized_arc(vel_dir, target_dir);

        if self.data.homing_max_angle > 180.0 {
            self.velocity = vec3_transform_quaternion_safe(self.velocity, q);
            self.accel = vec3_transform_quaternion_safe(self.accel, q);
            return;
        }

        let angle_rad = self.data.homing_max_angle * PI / 180.0;
        let c = angle_rad.cos();
        let s = angle_rad.sin();

        let mut q_adjusted = q;

        if q.3 <= -1.0 + 0.0001 {
            q_adjusted = (0.0, 0.0, s, c);
        } else if q.3 <= c && q.3 <= 1.0 - 0.0001 {
            let factor = s / (1.0 - q.3 * q.3).sqrt();
            q_adjusted = (q.0 * factor, q.1 * factor, q.2 * factor, c);
        }

        self.velocity = vec3_transform_quaternion_safe(self.velocity, q_adjusted);
        self.accel = vec3_transform_quaternion_safe(self.accel, q_adjusted);
    }

    /// Replicates m2_flying_calculate_step
    fn calculate_step(&mut self, dt: f32) -> f32 {
        self.velocity.0 += self.accel.0 * dt;
        self.velocity.1 += self.accel.1 * dt;
        self.velocity.2 += (self.accel.2 - self.data.gravity) * dt;

        let move_x = self.velocity.0 * dt;
        let move_y = self.velocity.1 * dt;
        let move_z = self.velocity.2 * dt;

        let move_dist = (move_x * move_x + move_y * move_y + move_z * move_z).sqrt();

        self.pos.0 += move_x;
        self.pos.1 += move_y;
        self.pos.2 += move_z;

        move_dist
    }

    /// Replicates CFlyingInstance::Update
    pub fn update(
        &mut self, 
        dt: f32, 
        get_target_pos: &impl Fn(u32) -> Option<(f32, f32, f32)>,
        get_objects_in_range: &impl Fn((f32, f32, f32), f32) -> Vec<u32>, // Returns VIDs
    ) -> FlyingStatus {
        if !self.is_alive {
            return FlyingStatus::OutOfRange;
        }

        // Homing logic
        if self.data.is_homing && (self.data.homing_start_time + self.start_time < self.current_time) {
            if let FlyTarget::Object(vid) = self.target {
                if let Some(pos) = get_target_pos(vid) {
                    self.adjust_direction_for_homing(pos);
                }
            }
        }

        let last_pos = self.pos;
        let move_distance = self.calculate_step(dt);
        let collision_sphere_radius = (move_distance * 2.0).max(self.data.collision_sphere_radius);
        
        self.remain_range -= move_distance;

        if self.remain_range < 0.0 {
            self.is_alive = false;
            return FlyingStatus::OutOfRange;
        }

        let target_pos = match self.target {
            FlyTarget::Object(vid) => get_target_pos(vid),
            FlyTarget::Position(x, y, z) => Some((x, y, z)),
            FlyTarget::None => None,
        };

        // Target hit detection
        if let Some(t_pos) = target_pos {
            if !self.target_hitted {
                let dist_sq = square_distance_between_linesegment_and_point(self.pos, last_pos, t_pos);
                if dist_sq < self.data.bomb_range * self.data.bomb_range {
                    self.target_hitted = true;
                    
                    if self.pierce_count > 0 {
                        self.pierce_count -= 1;
                        // __Bomb() logic would go here
                    } else {
                        self.is_alive = false;
                        return FlyingStatus::HitTarget;
                    }
                    return FlyingStatus::HitTarget; // Still flying if pierced
                }
            }
        }

        // Hit on Another Monster detection
        if self.data.hit_on_another_monster {
            let hit_vids = get_objects_in_range(self.pos, collision_sphere_radius);
            for vid in hit_vids {
                if Some(vid) != self.owner_vid {
                    if !self.hitted_object_set.contains(&vid) {
                        self.hitted_object_set.insert(vid);

                        if self.pierce_count > 0 {
                            self.pierce_count -= 1;
                        } else {
                            self.is_alive = false;
                            return FlyingStatus::HitAnotherMonster;
                        }
                        return FlyingStatus::HitAnotherMonster;
                    }
                }
            }
        }

        FlyingStatus::Flying
    }
}

pub struct FlyProjectileTracker {
    instances: HashMap<u32, FlyingInstance>,
    id_counter: u32,
}

impl FlyProjectileTracker {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            id_counter: 1,
        }
    }

    pub fn create_flying_instance(
        &mut self,
        data: FlyingData,
        start_pos: (f32, f32, f32),
        target: FlyTarget,
        can_attack: bool,
    ) -> u32 {
        let id = self.id_counter;
        self.id_counter += 1;

        let instance = FlyingInstance::new(id, data, start_pos, target, can_attack);
        self.instances.insert(id, instance);
        id
    }

    pub fn get_instance_mut(&mut self, id: u32) -> Option<&mut FlyingInstance> {
        self.instances.get_mut(&id)
    }

    pub fn remove_instance(&mut self, id: u32) {
        self.instances.remove(&id);
    }
}

// --- Math Functions from GrpMath.h ---

/// SafeRotationNormalizedArc
fn safe_rotation_normalized_arc(v_from: (f32, f32, f32), v_to: (f32, f32, f32)) -> (f32, f32, f32, f32) {
    let diff_sq = (v_from.0 - v_to.0).powi(2) + (v_from.1 - v_to.1).powi(2) + (v_from.2 - v_to.2).powi(2);
    if diff_sq < 0.0001 {
        return (0.0, 0.0, 0.0, 1.0);
    }
    let sum_sq = (v_from.0 + v_to.0).powi(2) + (v_from.1 + v_to.1).powi(2) + (v_from.2 + v_to.2).powi(2);
    if sum_sq < 0.0001 {
        return (0.0, 0.0, 1.0, 0.0);
    }

    let c = vec3_cross(v_from, v_to);
    let d = vec3_dot(v_from, v_to);
    let s = ((1.0 + d) * 2.0).sqrt();

    if s == 0.0 {
        (0.0, 0.0, 1.0, 0.0)
    } else {
        (c.0 / s, c.1 / s, c.2 / s, s * 0.5)
    }
}

fn vec3_cross(a: (f32, f32, f32), b: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

fn vec3_dot(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn vec3_transform_quaternion_safe(v: (f32, f32, f32), q: (f32, f32, f32, f32)) -> (f32, f32, f32) {
    let cross_qv = vec3_cross((q.0, q.1, q.2), v);
    let cross_q_qv = vec3_cross((q.0, q.1, q.2), cross_qv);

    (
        v.0 + 2.0 * (q.3 * cross_qv.0 + cross_q_qv.0),
        v.1 + 2.0 * (q.3 * cross_qv.1 + cross_q_qv.1),
        v.2 + 2.0 * (q.3 * cross_qv.2 + cross_q_qv.2),
    )
}

/// square_distance_between_linesegment_and_point
pub fn square_distance_between_linesegment_and_point(p1: (f32, f32, f32), p2: (f32, f32, f32), x: (f32, f32, f32)) -> f32 {
    let v1 = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
    let l = vec3_dot(v1, v1);

    let v2 = (x.0 - p1.0, x.1 - p1.1, x.2 - p1.2);
    let v3 = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);

    let d = vec3_dot(v2, v3);
    
    if d <= 0.0 {
        vec3_dot(v2, v2)
    } else if d >= l {
        let v4 = (x.0 - p2.0, x.1 - p2.1, x.2 - p2.2);
        vec3_dot(v4, v4)
    } else {
        let c = vec3_cross(v2, v3);
        if l > 0.0 {
            vec3_dot(c, c) / l
        } else {
            vec3_dot(v2, v2)
        }
    }
}
