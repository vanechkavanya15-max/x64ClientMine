use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpsLimit {
    Unlimited,
    Limited(u32),
}

pub struct FpsLimiter {
    limit: FpsLimit,
    last_frame_time: Instant,
    sleep_threshold: Duration,
}

impl Default for FpsLimiter {
    fn default() -> Self {
        Self::new(FpsLimit::Unlimited)
    }
}

impl FpsLimiter {
    pub fn new(limit: FpsLimit) -> Self {
        Self {
            limit,
            last_frame_time: Instant::now(),
            sleep_threshold: Duration::from_millis(2),
        }
    }

    pub fn limit(&self) -> FpsLimit {
        self.limit
    }

    pub fn set_limit(&mut self, limit: FpsLimit) {
        self.limit = limit;
        self.last_frame_time = Instant::now();
    }

    pub fn set_sleep_threshold(&mut self, threshold: Duration) {
        self.sleep_threshold = threshold;
    }

    pub fn wait(&mut self) {
        let fps = match self.limit {
            FpsLimit::Unlimited | FpsLimit::Limited(0) => {
                self.last_frame_time = Instant::now();
                return;
            }
            FpsLimit::Limited(f) => f,
        };

        let target_duration = Duration::from_nanos(1_000_000_000 / fps as u64);
        
        loop {
            let elapsed = self.last_frame_time.elapsed();
            if elapsed >= target_duration {
                break;
            }
            
            let remaining = target_duration - elapsed;
            if remaining > self.sleep_threshold {
                std::thread::sleep(remaining - self.sleep_threshold);
            } else {
                std::hint::spin_loop();
            }
        }

        let now = Instant::now();
        // Allow rendering to catch up if we are behind, but don't let it run unbounded
        if now.duration_since(self.last_frame_time) > target_duration * 2 {
            self.last_frame_time = now - target_duration;
        } else {
            self.last_frame_time += target_duration;
        }
    }
    
    pub fn should_render(&mut self) -> bool {
        let fps = match self.limit {
            FpsLimit::Unlimited | FpsLimit::Limited(0) => {
                self.last_frame_time = Instant::now();
                return true;
            }
            FpsLimit::Limited(f) => f,
        };

        let target_duration = Duration::from_nanos(1_000_000_000 / fps as u64);
        let elapsed = self.last_frame_time.elapsed();

        if elapsed >= target_duration {
            let now = Instant::now();
            if now.duration_since(self.last_frame_time) > target_duration * 2 {
                self.last_frame_time = now - target_duration;
            } else {
                self.last_frame_time += target_duration;
            }
            true
        } else {
            false
        }
    }
}
