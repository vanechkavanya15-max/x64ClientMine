#[derive(Debug, Clone)]
pub struct GrannyAnimationClock {
    duration_seconds: f32,
    frames_per_second: f32,
    is_looping: bool,
    current_time: f32,
    speed_factor: f32,
    finished: bool,
}

impl GrannyAnimationClock {
    pub fn new(duration_seconds: f32, frames_per_second: f32, is_looping: bool) -> Self {
        Self {
            duration_seconds: duration_seconds.max(0.0),
            frames_per_second: frames_per_second.max(0.0),
            is_looping,
            current_time: 0.0,
            speed_factor: 1.0,
            finished: duration_seconds <= 0.0,
        }
    }

    pub fn advance(&mut self, delta_time: f32) {
        if self.duration_seconds <= 0.0 {
            self.current_time = 0.0;
            self.finished = true;
            return;
        }

        self.current_time += delta_time * self.speed_factor;

        if self.is_looping {
            self.finished = false;
            if self.current_time >= self.duration_seconds {
                self.current_time %= self.duration_seconds;
            } else if self.current_time < 0.0 {
                let rem = self.current_time % self.duration_seconds;
                if rem < 0.0 {
                    self.current_time = rem + self.duration_seconds;
                } else {
                    self.current_time = rem;
                }
                
                // Handle edge case where precision might cause it to exactly equal duration_seconds
                if self.current_time >= self.duration_seconds {
                    self.current_time = 0.0;
                }
            }
        } else {
            if self.current_time >= self.duration_seconds {
                self.current_time = self.duration_seconds;
                self.finished = self.speed_factor >= 0.0;
            } else if self.current_time <= 0.0 {
                self.current_time = 0.0;
                self.finished = self.speed_factor <= 0.0;
            } else {
                self.finished = false;
            }
        }
    }

    pub fn get_current_time(&self) -> f32 {
        self.current_time
    }

    pub fn get_current_frame(&self) -> f32 {
        self.current_time * self.frames_per_second
    }

    pub fn get_current_frame_index(&self) -> usize {
        let frame = self.get_current_frame();
        if frame < 0.0 {
            0
        } else {
            frame.floor() as usize
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed_factor = speed;
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.finished = self.duration_seconds <= 0.0;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
