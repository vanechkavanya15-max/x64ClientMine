#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobClass {
    Normal,
    Boss,
    Huge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct PushBackState {
    pub current_pos: Position,
    pub start_pos: Position,
    pub dest_pos: Position,
    pub active: bool,
    pub elapsed_time: f32,
    pub total_duration: f32,
}

impl PushBackState {
    pub fn new(initial_pos: Position) -> Self {
        Self {
            current_pos: initial_pos.clone(),
            start_pos: initial_pos.clone(),
            dest_pos: initial_pos,
            active: false,
            elapsed_time: 0.0,
            total_duration: 0.0,
        }
    }

    pub fn start_pushback(
        &mut self,
        look_dir_x: f32,
        look_dir_y: f32,
        base_distance: f32,
        duration: f32,
        mob_class: MobClass,
    ) -> Result<(), &'static str> {
        // Redukcja do 0 dla BOSS i HUGE
        let mass_modifier = match mob_class {
            MobClass::Normal => 1.0,
            MobClass::Boss | MobClass::Huge => 0.0,
        };

        if mass_modifier == 0.0 {
            self.active = false;
            return Ok(());
        }

        let len = (look_dir_x * look_dir_x + look_dir_y * look_dir_y).sqrt();
        if len == 0.0 {
            return Err("Wektor ciosu nie moze miec dlugosci 0");
        }

        let push_dx = look_dir_x / len;
        let push_dy = look_dir_y / len;

        let distance = base_distance * mass_modifier;

        self.start_pos = self.current_pos.clone();
        self.dest_pos = Position {
            x: self.current_pos.x + push_dx * distance,
            y: self.current_pos.y + push_dy * distance,
        };

        if duration <= 0.0 {
            self.current_pos = self.dest_pos.clone();
            self.active = false;
            self.elapsed_time = 0.0;
            self.total_duration = 0.0;
        } else {
            self.active = true;
            self.elapsed_time = 0.0;
            self.total_duration = duration;
        }

        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }

        self.elapsed_time += delta_time;

        if self.elapsed_time >= self.total_duration {
            self.current_pos = self.dest_pos.clone();
            self.active = false;
        } else {
            let t = self.elapsed_time / self.total_duration;
            self.current_pos.x = self.start_pos.x + (self.dest_pos.x - self.start_pos.x) * t;
            self.current_pos.y = self.start_pos.y + (self.dest_pos.y - self.start_pos.y) * t;
        }
    }
}
