pub struct MovementVector {
    pub cur_x: f32,
    pub cur_y: f32,
    pub dest_x: f32,
    pub dest_y: f32,
    pub move_speed: f32,
    pub is_moving: bool,
}

impl MovementVector {
    pub fn new(start_x: f32, start_y: f32) -> Self {
        Self {
            cur_x: start_x,
            cur_y: start_y,
            dest_x: start_x,
            dest_y: start_y,
            move_speed: 0.0,
            is_moving: false,
        }
    }

    pub fn set_destination(&mut self, dest_x: f32, dest_y: f32) {
        self.dest_x = dest_x;
        self.dest_y = dest_y;
        self.is_moving = true;
    }

    pub fn set_speed(&mut self, move_speed: f32) {
        self.move_speed = move_speed;
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        if !self.is_moving {
            return;
        }

        let dx = self.dest_x - self.cur_x;
        let dy = self.dest_y - self.cur_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            self.is_moving = false;
            return;
        }

        let speed_units_per_sec = self.move_speed; // speed_units_per_sec = move_speed * 100.0 / 100.0
        let step = speed_units_per_sec * delta_time_sec;

        if step >= dist {
            self.cur_x = self.dest_x;
            self.cur_y = self.dest_y;
            self.is_moving = false;
        } else {
            self.cur_x += (dx / dist) * step;
            self.cur_y += (dy / dist) * step;
        }
    }
}
