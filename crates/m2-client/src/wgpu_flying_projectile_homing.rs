use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlyTarget {
    None,
    Object(u32), // VID
    Position(f32, f32, f32),
}

impl Default for FlyTarget {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlyingStatus {
    Flying,
    Hit,
    MaxRangeReached,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlyingProjectileData {
    pub gravity: f32,
    pub bomb_range: f32,
    pub is_homing: bool,
    pub homing_max_angle: f32,
    pub homing_start_time: f32,
    pub collision_sphere_radius: f32,
    pub accel: (f32, f32, f32),
    pub max_range: f32,
}

impl Default for FlyingProjectileData {
    fn default() -> Self {
        Self {
            gravity: 0.0,
            bomb_range: 0.0,
            is_homing: false,
            homing_max_angle: 0.0,
            homing_start_time: 0.0,
            collision_sphere_radius: 0.0,
            accel: (0.0, 0.0, 0.0),
            max_range: 10000.0,
        }
    }
}

pub struct FlyingProjectileHoming {
    pub pos: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub accel: (f32, f32, f32),
    pub target: FlyTarget,
    pub data: FlyingProjectileData,
    pub flown_distance: f32,
    pub start_time: f32,
    pub current_time: f32,
}

impl FlyingProjectileHoming {
    pub fn new(
        pos: (f32, f32, f32),
        velocity: (f32, f32, f32),
        target: FlyTarget,
        data: FlyingProjectileData,
    ) -> Self {
        let accel = data.accel;
        Self {
            pos,
            velocity,
            accel,
            target,
            data,
            flown_distance: 0.0,
            start_time: 0.0,
            current_time: 0.0,
        }
    }

    pub fn set_time(&mut self, current_time: f32) {
        self.current_time = current_time;
    }

    /// Oblicza przemieszczenie dla danej klatki zgodnie z m2_flying_calculate_step
    pub fn calculate_step(&mut self, dt: f32) -> f32 {
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

    /// Implementacja CFlyingInstance::AdjustDirectionForHoming
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

    pub fn update(&mut self, dt: f32, get_target_pos: impl Fn(u32) -> Option<(f32, f32, f32)>) -> FlyingStatus {
        if self.data.is_homing && (self.data.homing_start_time + self.start_time < self.current_time) {
            let t_pos = match self.target {
                FlyTarget::Object(vid) => get_target_pos(vid),
                FlyTarget::Position(x, y, z) => Some((x, y, z)),
                FlyTarget::None => None,
            };
            if let Some(pos) = t_pos {
                self.adjust_direction_for_homing(pos);
            }
        }

        let last_pos = self.pos;
        let move_dist = self.calculate_step(dt);
        self.flown_distance += move_dist;

        if self.flown_distance >= self.data.max_range {
            return FlyingStatus::MaxRangeReached;
        }

        let t_pos = match self.target {
            FlyTarget::Object(vid) => get_target_pos(vid),
            FlyTarget::Position(x, y, z) => Some((x, y, z)),
            FlyTarget::None => None,
        };

        if let Some(target_position) = t_pos {
            let dist_sq = square_distance_between_linesegment_and_point(self.pos, last_pos, target_position);
            if dist_sq < self.data.bomb_range * self.data.bomb_range {
                return FlyingStatus::Hit;
            }
        }

        FlyingStatus::Flying
    }
}

/// Oblicza punkt na krzywej Beziera dla 3 punktów kontrolnych: p0, p1, p2 w czasie t [0, 1]
pub fn calculate_bezier_trajectory(p0: (f32, f32, f32), p1: (f32, f32, f32), p2: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
    let t_clamp = t.clamp(0.0, 1.0);
    let u = 1.0 - t_clamp;
    let tt = t_clamp * t_clamp;
    let uu = u * u;

    let x = uu * p0.0 + 2.0 * u * t_clamp * p1.0 + tt * p2.0;
    let y = uu * p0.1 + 2.0 * u * t_clamp * p1.1 + tt * p2.1;
    let z = uu * p0.2 + 2.0 * u * t_clamp * p1.2 + tt * p2.2;

    (x, y, z)
}

/// Wzorowane na C++ `SafeRotationNormalizedArc` z `GrpMath.h`
fn safe_rotation_normalized_arc(v_from: (f32, f32, f32), v_to: (f32, f32, f32)) -> (f32, f32, f32, f32) {
    // Tolerancja bycia identycznym
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
    // q = (x, y, z, w)
    let cross_qv = vec3_cross((q.0, q.1, q.2), v);
    let cross_q_qv = vec3_cross((q.0, q.1, q.2), cross_qv);

    (
        v.0 + 2.0 * (q.3 * cross_qv.0 + cross_q_qv.0),
        v.1 + 2.0 * (q.3 * cross_qv.1 + cross_q_qv.1),
        v.2 + 2.0 * (q.3 * cross_qv.2 + cross_q_qv.2),
    )
}

/// Wzorowane na C++ `square_distance_between_linesegment_and_point` z `GrpMath.h`
fn square_distance_between_linesegment_and_point(p1: (f32, f32, f32), p2: (f32, f32, f32), x: (f32, f32, f32)) -> f32 {
    let v1 = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
    let l = v1.0 * v1.0 + v1.1 * v1.1 + v1.2 * v1.2;

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
