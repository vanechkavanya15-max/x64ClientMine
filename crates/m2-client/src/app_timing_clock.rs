use std::time::Instant;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EngineTime {
    pub now_seconds: f64,
    pub delta_seconds: f32,
    pub global_time_ms: u64,
    pub frame_index: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FrameAccumulator {
    pub fixed_step_seconds: f32,
    pub accumulator: f32,
}

pub struct AppTimingClock {
    time: EngineTime,
    accumulator: FrameAccumulator,
    platform_clock: PlatformClock,
    fps_frames: u32,
    fps_timer: f32,
    current_fps: f32,
}

#[cfg(windows)]
extern "system" {
    fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> i32;
    fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> i32;
}

#[cfg(windows)]
struct PlatformClock {
    frequency: i64,
    start_counter: i64,
    last_counter: i64,
}

#[cfg(windows)]
impl PlatformClock {
    fn new() -> Self {
        let mut frequency = 0;
        let mut start_counter = 0;
        unsafe {
            QueryPerformanceFrequency(&mut frequency);
            QueryPerformanceCounter(&mut start_counter);
        }
        Self {
            frequency,
            start_counter,
            last_counter: start_counter,
        }
    }

    fn get_delta_and_update(&mut self) -> (f64, u64) {
        let mut now_counter = 0;
        unsafe {
            QueryPerformanceCounter(&mut now_counter);
        }
        let delta_counter = now_counter - self.last_counter;
        self.last_counter = now_counter;

        let delta_seconds = if self.frequency > 0 {
            delta_counter as f64 / self.frequency as f64
        } else {
            0.0
        };

        let global_time_ms = if self.frequency > 0 {
            ((now_counter - self.start_counter) as f64 * 1000.0 / self.frequency as f64) as u64
        } else {
            0
        };

        (delta_seconds, global_time_ms)
    }

    fn reset(&mut self) {
        let mut now_counter = 0;
        unsafe {
            QueryPerformanceCounter(&mut now_counter);
        }
        self.start_counter = now_counter;
        self.last_counter = now_counter;
    }
}

#[cfg(not(windows))]
struct PlatformClock {
    start_time: Instant,
    last_time: Instant,
}

#[cfg(not(windows))]
impl PlatformClock {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_time: now,
        }
    }

    fn get_delta_and_update(&mut self) -> (f64, u64) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time).as_secs_f64();
        self.last_time = now;
        let global_time_ms = now.duration_since(self.start_time).as_millis() as u64;
        (delta, global_time_ms)
    }

    fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_time = now;
    }
}

impl AppTimingClock {
    pub fn new(fixed_step: f32) -> Self {
        Self {
            time: EngineTime {
                now_seconds: 0.0,
                delta_seconds: 0.0,
                global_time_ms: 0,
                frame_index: 0,
            },
            accumulator: FrameAccumulator {
                fixed_step_seconds: fixed_step,
                accumulator: 0.0,
            },
            platform_clock: PlatformClock::new(),
            fps_frames: 0,
            fps_timer: 0.0,
            current_fps: 0.0,
        }
    }

    pub fn tick(&mut self) -> (f32, u64) {
        let (mut delta, global_ms) = self.platform_clock.get_delta_and_update();

        // Ochrona przed skokami czasu (spiral of death): maksymalny dopuszczalny delta_time = 0.1s
        if delta > 0.1 {
            delta = 0.1;
        }

        self.time.delta_seconds = delta as f32;
        self.time.now_seconds += delta;
        self.time.global_time_ms = global_ms;
        self.time.frame_index += 1;

        self.accumulator.accumulator += self.time.delta_seconds;

        self.fps_frames += 1;
        self.fps_timer += self.time.delta_seconds;
        
        if self.fps_timer >= 1.0 {
            self.current_fps = self.fps_frames as f32 / self.fps_timer;
            self.fps_frames = 0;
            self.fps_timer -= 1.0; // subtract 1.0 to carry over any excess time
        }

        (self.time.delta_seconds, self.time.global_time_ms)
    }

    pub fn get_current_fps(&self) -> f32 {
        self.current_fps
    }

    pub fn has_fixed_step(&mut self) -> bool {
        if self.accumulator.accumulator >= self.accumulator.fixed_step_seconds {
            self.accumulator.accumulator -= self.accumulator.fixed_step_seconds;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.platform_clock.reset();
        self.time = EngineTime {
            now_seconds: 0.0,
            delta_seconds: 0.0,
            global_time_ms: 0,
            frame_index: 0,
        };
        self.accumulator.accumulator = 0.0;
        self.fps_frames = 0;
        self.fps_timer = 0.0;
        self.current_fps = 0.0;
    }

    pub fn engine_time(&self) -> &EngineTime {
        &self.time
    }
}

/// Zwraca aktualny globalny czas silnika w milisekundach (zgodny z get_dword_time() w C++).
pub fn m2_get_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

