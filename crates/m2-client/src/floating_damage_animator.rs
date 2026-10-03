#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Normal,
    Critical,
    Poison,
    Miss,
    Block,
}

#[derive(Debug, Clone)]
pub struct FloatingDamage {
    pub id: u64,
    pub amount: u32,
    pub damage_type: DamageType,
    pub start_time: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub scale: f32,
    pub alpha: f32,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct AnimatorConfig {
    pub lifetime: f32,
    pub bounce_duration: f32,
    pub fade_duration: f32,
    pub float_speed_z: f32,
    pub base_scale: f32,
    pub max_bounce_scale: f32,
}

impl Default for AnimatorConfig {
    fn default() -> Self {
        Self {
            lifetime: 2.0,
            bounce_duration: 0.3,
            fade_duration: 0.5,
            float_speed_z: 100.0,
            base_scale: 1.0,
            max_bounce_scale: 1.5,
        }
    }
}

pub struct FloatingDamageAnimator {
    pub config: AnimatorConfig,
    instances: Vec<FloatingDamage>,
    next_id: u64,
}

fn smooth_interp(start: f32, end: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        0.0
    } else if t > 1.0 {
        1.0
    } else {
        t
    };
    let t_smooth = t * t * (3.0 - 2.0 * t);
    start + (end - start) * t_smooth
}

impl FloatingDamageAnimator {
    pub fn new(config: AnimatorConfig) -> Self {
        Self {
            config,
            instances: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_damage(&mut self, amount: u32, damage_type: DamageType, current_time: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(FloatingDamage {
            id,
            amount,
            damage_type,
            start_time: current_time,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            scale: 0.0,
            alpha: 1.0,
            is_active: true,
        });
        id
    }

    pub fn update(&mut self, current_time: f32) {
        for instance in self.instances.iter_mut() {
            let elapsed = current_time - instance.start_time;

            if elapsed < 0.0 {
                continue;
            }

            if elapsed >= self.config.lifetime {
                instance.is_active = false;
                instance.alpha = 0.0;
                continue;
            }

            // Update Z offset (float upwards)
            instance.offset_z = elapsed * self.config.float_speed_z;

            // Update Scale (bounce effect)
            if elapsed < self.config.bounce_duration && self.config.bounce_duration > 0.0 {
                let t = elapsed / self.config.bounce_duration;
                let t_half = t * 2.0;
                if t_half < 1.0 {
                    instance.scale = smooth_interp(0.0, self.config.max_bounce_scale, t_half);
                } else {
                    instance.scale = smooth_interp(
                        self.config.max_bounce_scale,
                        self.config.base_scale,
                        t_half - 1.0,
                    );
                }
            } else {
                instance.scale = self.config.base_scale;
            }

            // Update Alpha (fade out)
            let time_remaining = self.config.lifetime - elapsed;
            if time_remaining < self.config.fade_duration && self.config.fade_duration > 0.0 {
                instance.alpha = time_remaining / self.config.fade_duration;
            } else {
                instance.alpha = 1.0;
            }
        }

        self.instances.retain(|inst| inst.is_active);
    }

    pub fn get_instances(&self) -> &[FloatingDamage] {
        &self.instances
    }

    pub fn clear(&mut self) {
        self.instances.clear();
    }
}
