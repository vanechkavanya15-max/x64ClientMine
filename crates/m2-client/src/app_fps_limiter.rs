use std::time::{Duration, Instant};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpsLimit {
    Fps30,
    Fps60,
    Fps120,
    Fps144,
    Fps240,
    Uncapped,
}

impl FpsLimit {
    pub fn budget_us(&self) -> u64 {
        match self {
            FpsLimit::Fps30 => 1_000_000 / 30,
            FpsLimit::Fps60 => 1_000_000 / 60,
            FpsLimit::Fps120 => 1_000_000 / 120,
            FpsLimit::Fps144 => 1_000_000 / 144,
            FpsLimit::Fps240 => 1_000_000 / 240,
            FpsLimit::Uncapped => 0,
        }
    }
}

pub struct FramePacer {
    target_fps: FpsLimit,
    frame_budget_us: u64,
    last_frame_instant: Instant,
}

impl FramePacer {
    pub fn new(target_fps: FpsLimit) -> Self {
        Self {
            target_fps,
            frame_budget_us: target_fps.budget_us(),
            last_frame_instant: Instant::now(),
        }
    }

    pub fn set_target_fps(&mut self, limit: FpsLimit) {
        self.target_fps = limit;
        self.frame_budget_us = limit.budget_us();
    }

    pub fn get_target_fps(&self) -> FpsLimit {
        self.target_fps
    }

    pub fn pace_frame(&mut self) {
        if self.target_fps == FpsLimit::Uncapped {
            self.last_frame_instant = Instant::now();
            return;
        }

        let mut elapsed = self.last_frame_instant.elapsed().as_micros() as u64;

        if elapsed < self.frame_budget_us {
            let remaining = self.frame_budget_us - elapsed;

            // Hybrid sleep: OS sleep for big chunks (>2ms), then spin loop for microsecond precision
            if remaining > 2000 {
                let sleep_us = remaining - 2000;
                std::thread::sleep(Duration::from_micros(sleep_us));
            }

            // Spin-wait for the rest
            while {
                elapsed = self.last_frame_instant.elapsed().as_micros() as u64;
                elapsed < self.frame_budget_us
            } {
                std::hint::spin_loop();
            }
        }

        // Set the start of the next frame to *now* (after pacing)
        // Alternatively, it could be advanced by exactly frame_budget_us to be extremely precise
        // and eliminate jitter cumulatively, but Instant::now() prevents death spirals if frames take too long.
        self.last_frame_instant = Instant::now();
    }
}
