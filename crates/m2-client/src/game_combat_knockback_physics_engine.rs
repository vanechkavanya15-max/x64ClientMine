use glam::{Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnockbackMotionState {
    DamageFlying,
    DamageFlyingBack,
    StandUp,
    StandUpBack,
    Wait,
}

pub struct KnockbackPhysicsEngine {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub direction: Vec3,
    pub mass: f32,
    pub friction: f32,
    pub motion_state: KnockbackMotionState,
    pub stand_up_timer: f32,
    pub target_position: Vec3,
    pub original_position: Vec3,
    pub blending_time: f32,
    pub remaining_time: f32,
    pub knockdown_rotation: f32,
}

impl KnockbackPhysicsEngine {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            direction: Vec3::ZERO,
            mass: 1.0,
            friction: 0.3,
            motion_state: KnockbackMotionState::Wait,
            stand_up_timer: 0.0,
            target_position: position,
            original_position: position,
            blending_time: 0.0,
            remaining_time: 0.0,
            knockdown_rotation: 0.0,
        }
    }

    pub fn calculate_push_direction(&mut self, attacker_pos: Vec3, victim_pos: Vec3) -> Vec3 {
        let mut dir = victim_pos - attacker_pos;
        dir.z = 0.0;
        let length = dir.length();
        if length > 0.001 {
            dir = dir / length;
        } else {
            dir = Vec3::ZERO;
        }
        self.direction = dir;
        dir
    }

    pub fn apply_external_force(
        &mut self,
        force: f32,
        attacker_rotation_deg: f32,
        victim_rotation_deg: f32,
        base_position: Vec3,
    ) {
        // Accumulate Acceleration by External Force
        let eps = 0.001;
        self.acceleration = self.direction * (force / self.mass);
        self.velocity = self.acceleration;

        // Loop ticks as seen in CPhysicsObject::IncreaseExternalForce
        let loop_value = 100;
        let frame_time = 0.02;
        let mut movement = Vec3::ZERO;
        let mut actual_ticks = 0;

        for i in 0..loop_value {
            actual_ticks = i + 1;
            let mut f_force = 0.0;
            if self.velocity.x.abs() >= eps || self.velocity.y.abs() >= eps || self.velocity.z.abs() >= eps {
                f_force -= self.mass * self.friction;
            }

            self.acceleration = self.direction * (f_force / self.mass);
            self.velocity += self.acceleration;

            if self.velocity.x * self.direction.x <= 0.0 {
                self.velocity.x = 0.0;
                self.direction.x = 0.0;
            }
            if self.velocity.y * self.direction.y <= 0.0 {
                self.velocity.y = 0.0;
                self.direction.y = 0.0;
            }
            if self.velocity.z * self.direction.z <= 0.0 {
                self.velocity.z = 0.0;
                self.direction.z = 0.0;
            }

            movement += self.velocity;

            if self.velocity.x.abs() < eps && self.velocity.y.abs() < eps && self.velocity.z.abs() < eps {
                break;
            }
        }

        self.original_position = base_position;
        self.target_position = base_position + movement;
        self.blending_time = (actual_ticks as f32) * frame_time;
        self.remaining_time = self.blending_time;

        let attacker_rad = attacker_rotation_deg.to_radians();
        let victim_rad = victim_rotation_deg.to_radians();

        let attacker_normal = Vec2::new(attacker_rad.sin(), attacker_rad.cos()).normalize_or_zero();
        let victim_normal = Vec2::new(victim_rad.sin(), victim_rad.cos()).normalize_or_zero();

        let dot = attacker_normal.dot(victim_normal);

        if dot < 0.0 {
            self.motion_state = KnockbackMotionState::DamageFlying;
            // Face the attacker
            self.knockdown_rotation = attacker_rotation_deg + 180.0;
        } else {
            self.motion_state = KnockbackMotionState::DamageFlyingBack;
            // Face the same direction as the attack is pushing
            self.knockdown_rotation = attacker_rotation_deg;
        }
        
        // Normalize rotation to 0..360 safely using rem_euclid
        self.knockdown_rotation = self.knockdown_rotation.rem_euclid(360.0);

        // typical knockback sequence delay for standing up
        self.stand_up_timer = self.blending_time + 1.0; 
    }

    pub fn update(&mut self, elapsed_time: f32) {
        if self.remaining_time > 0.0 {
            self.remaining_time -= elapsed_time;
            if self.remaining_time <= 0.0 {
                self.remaining_time = 0.0;
                self.position = self.target_position;
            } else {
                let progress = 1.0 - (self.remaining_time / self.blending_time);
                // Simple linear interpolation for blending, actual gamelib uses ease-out but linear is sufficient for testing functionality if ease-out formula isn't strictly requested.
                // Let's implement ease-out: Speed starts max and decelerates.
                // Value = start + (end - start) * (1 - (1 - progress)^2)
                let ease_out_progress = 1.0 - (1.0 - progress) * (1.0 - progress);
                self.position = self.original_position + (self.target_position - self.original_position) * ease_out_progress;

                // Add gravity arc (parabola) over the z axis for knockdown effect
                if matches!(self.motion_state, KnockbackMotionState::DamageFlying | KnockbackMotionState::DamageFlyingBack) {
                    let arc_height = 150.0; // max height of jump in units
                    let z_offset = arc_height * (1.0 - (2.0 * progress - 1.0).powi(2));
                    self.position.z = self.original_position.z + z_offset.max(0.0);
                }
            }
        }

        if self.stand_up_timer > 0.0 {
            self.stand_up_timer -= elapsed_time;
            if self.stand_up_timer <= 0.0 {
                self.stand_up_timer = 0.0;
                match self.motion_state {
                    KnockbackMotionState::DamageFlying => {
                        self.motion_state = KnockbackMotionState::StandUp;
                        self.stand_up_timer = 1.0; // Stand up duration
                    }
                    KnockbackMotionState::DamageFlyingBack => {
                        self.motion_state = KnockbackMotionState::StandUpBack;
                        self.stand_up_timer = 1.0;
                    }
                    KnockbackMotionState::StandUp | KnockbackMotionState::StandUpBack => {
                        self.motion_state = KnockbackMotionState::Wait;
                    }
                    _ => {}
                }
            }
        }
    }
}
