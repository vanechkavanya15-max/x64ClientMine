use glam::Vec3;

const FRAME_TIME: f32 = 0.02;
const EPSILON: f32 = 0.001;
const FLT_EPSILON: f32 = 1.19209290e-07_f32;

#[derive(Debug, Clone)]
pub struct EaseOutInterpolation {
    pub remaining_time: f32,
    pub value: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub start_value: f32,
    pub last_value: f32,
}

impl EaseOutInterpolation {
    pub fn new() -> Self {
        let mut slf = Self {
            remaining_time: 0.0,
            value: 0.0,
            speed: 0.0,
            acceleration: 0.0,
            start_value: 0.0,
            last_value: 0.0,
        };
        slf.initialize();
        slf
    }

    pub fn initialize(&mut self) {
        self.remaining_time = 0.0;
        self.value = 0.0;
        self.speed = 0.0;
        self.acceleration = 0.0;
        self.start_value = 0.0;
        self.last_value = 0.0;
    }

    pub fn setup(&mut self, start: f32, end: f32, mut time: f32) -> bool {
        // for safety
        if time.abs() < FLT_EPSILON {
            time = 0.01;
        }

        self.value = start;
        self.start_value = start;
        self.last_value = start;

        self.speed = (2.0 * (end - start)) / time;
        self.acceleration = 2.0 * (end - start) / (time * time) - 2.0 * self.speed / time;
        self.remaining_time = time;

        true
    }

    pub fn interpolate(&mut self, elapsed_time: f32) {
        self.last_value = self.value;

        self.remaining_time -= elapsed_time;
        self.speed += self.acceleration * elapsed_time;
        self.value += self.speed * elapsed_time;

        if !self.is_playing() {
            self.value = 0.0;
            self.last_value = 0.0;
        }
    }

    pub fn is_playing(&self) -> bool {
        self.remaining_time > 0.0
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_changing_value(&self) -> f32 {
        self.value - self.last_value
    }
}

impl Default for EaseOutInterpolation {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PhysicsWorldCollider {
    fn is_physical_collision(&self, position: Vec3) -> bool;
}

pub struct PhysicsObject {
    pub mass: f32,
    pub friction: f32,
    pub direction: Vec3,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub last_position: Vec3,

    pub x_pushing_position: EaseOutInterpolation,
    pub y_pushing_position: EaseOutInterpolation,
}

impl PhysicsObject {
    pub fn new() -> Self {
        let mut slf = Self {
            mass: 1.0,
            friction: 0.3,
            direction: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            velocity: Vec3::ZERO,
            last_position: Vec3::ZERO,
            x_pushing_position: EaseOutInterpolation::new(),
            y_pushing_position: EaseOutInterpolation::new(),
        };
        slf.initialize();
        slf
    }

    pub fn initialize(&mut self) {
        self.mass = 1.0;
        self.friction = 0.3;
        self.direction = Vec3::ZERO;
        self.acceleration = Vec3::ZERO;
        self.velocity = Vec3::ZERO;
        self.last_position = Vec3::ZERO;

        self.x_pushing_position.initialize();
        self.y_pushing_position.initialize();
    }

    pub fn update(&mut self, elapsed_time: f32) {
        if self.x_pushing_position.is_playing() {
            self.x_pushing_position.interpolate(elapsed_time);
        }
        if self.y_pushing_position.is_playing() {
            self.y_pushing_position.interpolate(elapsed_time);
        }
    }

    pub fn is_blending(&self) -> bool {
        // NOTE : IncreaseExternalForce() 에 의해 밀리는 처리중인가?
        if self.velocity.length() != 0.0 {
            return true;
        }

        // NOTE : SetLastPosition() 에 의해 밀리는 처리중인가?
        if self.x_pushing_position.is_playing() || self.y_pushing_position.is_playing() {
            return true;
        }

        false
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction;
    }

    pub fn increase_external_force_with_collider<C: PhysicsWorldCollider>(
        &mut self,
        base_position: Vec3,
        force: f32,
        collider: Option<&C>,
    ) {
        // Accumulate Acceleration by External Force
        self.acceleration = self.direction * (force / self.mass);
        self.velocity = self.acceleration;

        // NOTE : 최종 위치를 구해둔다. 근데 100보다 크다면? ;
        let loop_value = 100;
        let mut movement = Vec3::ZERO;

        for _ in 0..loop_value {
            self.accumulate(&mut movement);

            // VICTIM_COLLISION_TEST
            if let Some(col) = collider {
                if col.is_physical_collision(base_position + movement) {
                    self.initialize();
                    return;
                }
            }

            if self.velocity.x.abs() < EPSILON
                && self.velocity.y.abs() < EPSILON
                && self.velocity.z.abs() < EPSILON
            {
                break;
            }
        }

        self.set_last_position(movement, (loop_value as f32) * FRAME_TIME);

        // if m_pActorInstance AdjustCollisionWithOtherObjects
        // omitted here as it's tightly coupled to the full object manager system
    }

    pub fn increase_external_force(&mut self, base_position: Vec3, force: f32) {
        // Dummy implementation since Rust requires type for Option<&C> if not provided explicitly
        // We will pass a dummy collider type to the underlying method.
        struct DummyCollider;
        impl PhysicsWorldCollider for DummyCollider {
            fn is_physical_collision(&self, _position: Vec3) -> bool { false }
        }
        let dummy: Option<&DummyCollider> = None;
        self.increase_external_force_with_collider(base_position, force, dummy);
    }

    pub fn set_last_position(&mut self, position: Vec3, blending_time: f32) {
        self.last_position = position;
        self.x_pushing_position.setup(0.0, position.x, blending_time);
        self.y_pushing_position.setup(0.0, position.y, blending_time);
    }

    pub fn get_last_position(&self) -> Vec3 {
        self.last_position
    }

    pub fn get_x_movement(&self) -> f32 {
        self.x_pushing_position.get_changing_value()
    }

    pub fn get_y_movement(&self) -> f32 {
        self.y_pushing_position.get_changing_value()
    }

    fn accumulate(&mut self, position: &mut Vec3) {
        // If object is moving, give minor power to object.
        let mut force = 0.0;

        if self.velocity.x.abs() > EPSILON
            || self.velocity.y.abs() > EPSILON
            || self.velocity.z.abs() > EPSILON
        {
            force -= self.mass * self.friction;
        }

        self.acceleration = self.direction * (force / self.mass);
        self.velocity += self.acceleration;

        if self.velocity.x * self.direction.x < EPSILON {
            self.velocity.x = 0.0;
            self.direction.x = 0.0;
        }
        if self.velocity.y * self.direction.y < EPSILON {
            self.velocity.y = 0.0;
            self.direction.y = 0.0;
        }
        if self.velocity.z * self.direction.z < EPSILON {
            self.velocity.z = 0.0;
            self.direction.z = 0.0;
        }

        position.x += self.velocity.x;
        position.y += self.velocity.y;
        position.z += self.velocity.z;
    }
}

impl Default for PhysicsObject {
    fn default() -> Self {
        Self::new()
    }
}
