pub struct StaminaSystem {
    pub current_stamina: f32,
    pub max_stamina: f32,
    pub is_consuming_stamina: bool,
}

impl StaminaSystem {
    pub const CONSUME_STAMINA_PER_SEC: f32 = 50.0;
    pub const REGEN_STAMINA_PER_SEC: f32 = 25.0;

    pub fn new() -> Self {
        Self {
            current_stamina: 1000.0,
            max_stamina: 1000.0,
            is_consuming_stamina: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_consuming_stamina {
            self.current_stamina -= delta_time * Self::CONSUME_STAMINA_PER_SEC;
            
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
