#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlyingTarget {
    Position(f32, f32, f32),
    Vid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlyingStatus {
    Flying,
    Hit,
    MaxRangeReached,
}

#[derive(Debug, Clone)]
pub struct FlyingInstance {
    pub pos: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub target: Option<FlyingTarget>,
    pub speed: f32,
    pub turn_rate: f32,
    pub hit_radius: f32,
    pub max_range: f32,
    pub flown_distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl FlyingInstance {
    pub fn new(
        pos: (f32, f32, f32),
        velocity: (f32, f32, f32),
        target: Option<FlyingTarget>,
        speed: f32,
        turn_rate: f32,
        hit_radius: f32,
        max_range: f32,
    ) -> Self {
        let mut instance = Self {
            pos,
            velocity,
            target,
            speed,
            turn_rate,
            hit_radius,
            max_range,
            flown_distance: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        };
        instance.update_orientation();
        instance
    }

    pub fn update(
        &mut self,
        dt: f32,
        get_target_pos: impl Fn(u32) -> Option<(f32, f32, f32)>,
    ) -> FlyingStatus {
        let target_pos = match self.target {
            Some(FlyingTarget::Position(x, y, z)) => Some((x, y, z)),
            Some(FlyingTarget::Vid(vid)) => get_target_pos(vid),
            None => None,
        };

        if let Some((tx, ty, tz)) = target_pos {
            let dir_x = tx - self.pos.0;
            let dir_y = ty - self.pos.1;
            let dir_z = tz - self.pos.2;

            let dist = (dir_x * dir_x + dir_y * dir_y + dir_z * dir_z).sqrt();

            if dist < self.hit_radius {
                return FlyingStatus::Hit;
            }

            if dist > 0.0 {
                let dir_norm_x = dir_x / dist;
                let dir_norm_y = dir_y / dist;
                let dir_norm_z = dir_z / dist;

                let vel_len = (self.velocity.0 * self.velocity.0
                    + self.velocity.1 * self.velocity.1
                    + self.velocity.2 * self.velocity.2)
                    .sqrt();

                let (v_norm_x, v_norm_y, v_norm_z) = if vel_len > 0.0 {
                    (
                        self.velocity.0 / vel_len,
                        self.velocity.1 / vel_len,
                        self.velocity.2 / vel_len,
                    )
                } else {
                    // Fallback to straight towards target if velocity was 0
                    (dir_norm_x, dir_norm_y, dir_norm_z)
                };

                let t = (self.turn_rate * dt).clamp(0.0, 1.0);

                let mut new_v_x = v_norm_x + (dir_norm_x - v_norm_x) * t;
                let mut new_v_y = v_norm_y + (dir_norm_y - v_norm_y) * t;
                let mut new_v_z = v_norm_z + (dir_norm_z - v_norm_z) * t;

                let new_v_len =
                    (new_v_x * new_v_x + new_v_y * new_v_y + new_v_z * new_v_z).sqrt();

                if new_v_len > 0.0 {
                    new_v_x /= new_v_len;
                    new_v_y /= new_v_len;
                    new_v_z /= new_v_len;
                } else {
                    new_v_x = dir_norm_x;
                    new_v_y = dir_norm_y;
                    new_v_z = dir_norm_z;
                }

                self.velocity = (
                    new_v_x * self.speed,
                    new_v_y * self.speed,
                    new_v_z * self.speed,
                );
            }
        }

        let move_x = self.velocity.0 * dt;
        let move_y = self.velocity.1 * dt;
        let move_z = self.velocity.2 * dt;

        let move_dist = (move_x * move_x + move_y * move_y + move_z * move_z).sqrt();

        self.pos.0 += move_x;
        self.pos.1 += move_y;
        self.pos.2 += move_z;

        self.flown_distance += move_dist;

        self.update_orientation();

        if self.flown_distance >= self.max_range {
            FlyingStatus::MaxRangeReached
        } else {
            FlyingStatus::Flying
        }
    }

    fn update_orientation(&mut self) {
        let (vx, vy, vz) = self.velocity;
        let len_xy = (vx * vx + vy * vy).sqrt();
        
        if len_xy > 0.0 {
            self.yaw = vy.atan2(vx);
        }
        
        let len_xyz = (vx * vx + vy * vy + vz * vz).sqrt();
        if len_xyz > 0.0 {
            self.pitch = vz.atan2(len_xy);
        }
    }
}
