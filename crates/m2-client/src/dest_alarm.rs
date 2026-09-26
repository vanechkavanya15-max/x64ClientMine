pub struct DestAlarm {
    dest_pos: Option<(f32, f32)>,
    last_alarm_time_ms: u64,
}

impl DestAlarm {
    pub fn new() -> Self {
        Self {
            dest_pos: None,
            last_alarm_time_ms: 0,
        }
    }

    pub fn set_destination(&mut self, x: f32, y: f32, current_time_ms: u64) {
        self.dest_pos = Some((x, y));
        self.last_alarm_time_ms = current_time_ms;
    }

    pub fn clear_destination(&mut self) {
        self.dest_pos = None;
    }

    pub fn update(
        &mut self,
        current_x: f32,
        current_y: f32,
        current_time_ms: u64,
        is_moving: bool,
    ) -> Option<(f32, f32)> {
        if let Some((dx, dy)) = self.dest_pos {
            let distance = ((current_x - dx) * (current_x - dx) + (current_y - dy) * (current_y - dy)).sqrt();
            
            if distance < 100.0 {
                self.clear_destination();
                return None;
            }

            if is_moving {
                self.last_alarm_time_ms = current_time_ms;
                return None;
            } else {
                if current_time_ms.saturating_sub(self.last_alarm_time_ms) >= 500 {
                    self.last_alarm_time_ms = current_time_ms;
                    return Some((dx, dy));
                }
            }
        }
        
        None
    }
}
