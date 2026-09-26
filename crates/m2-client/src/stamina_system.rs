pub struct StaminaSystem {
    pub current_stamina: f32,
    pub max_stamina: f32,
    pub is_consuming_stamina: bool,
    pub consume_per_sec: f32,
}

impl StaminaSystem {
    pub const CONSUME_STAMINA_PER_SEC: f32 = 50.0;
    pub const REGEN_STAMINA_PER_SEC: f32 = 25.0;

    pub fn new() -> Self {
        Self {
            current_stamina: 1000.0,
            max_stamina: 1000.0,
            is_consuming_stamina: false,
            consume_per_sec: Self::CONSUME_STAMINA_PER_SEC,
        }
    }

    pub fn start_stamina_consume(&mut self, consume_per_sec: f32, current_stamina: f32) {
        self.is_consuming_stamina = true;
        self.consume_per_sec = consume_per_sec;
        self.current_stamina = current_stamina;
    }

    pub fn stop_stamina_consume(&mut self, current_stamina: f32) {
        self.is_consuming_stamina = false;
        self.consume_per_sec = 0.0;
        self.current_stamina = current_stamina;
    }

    pub fn stop_stamina(&mut self, current_stamina: f32) {
        self.stop_stamina_consume(current_stamina);
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_consuming_stamina {
            let rate = if self.consume_per_sec > 0.0 {
                self.consume_per_sec
            } else {
                Self::CONSUME_STAMINA_PER_SEC
            };
            self.current_stamina -= delta_time * rate;
            
            if self.current_stamina <= 0.0 {
                self.current_stamina = 0.0;
                self.is_consuming_stamina = false;
            }
        } else {
            self.current_stamina += delta_time * Self::REGEN_STAMINA_PER_SEC;
            if self.current_stamina > self.max_stamina {
                self.current_stamina = self.max_stamina;
            }
        }
    }

    pub fn get_point_stamina(&self) -> u32 {
        self.current_stamina as u32
    }
}
