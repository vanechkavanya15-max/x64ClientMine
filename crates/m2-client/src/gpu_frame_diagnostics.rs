use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiagnosticsData {
    pub fps: u32,
    pub cpu_frame_time_ms: f64,
    pub gpu_frame_time_ms: f64,
    pub triangle_count: u32,
    pub draw_calls: u32,
}

impl Default for DiagnosticsData {
    fn default() -> Self {
        Self {
            fps: 0,
            cpu_frame_time_ms: 0.0,
            gpu_frame_time_ms: 0.0,
            triangle_count: 0,
            draw_calls: 0,
        }
    }
}

pub struct GpuFrameDiagnostics {
    current_frame_start: Option<Instant>,
    
    // Per frame accumulators
    current_triangle_count: u32,
    current_draw_calls: u32,

    // FPS Calculation state
    frame_count_since_last_second: u32,
    last_fps_update: Instant,

    // Last completed frame data
    data: DiagnosticsData,
}

impl Default for GpuFrameDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuFrameDiagnostics {
    pub fn new() -> Self {
        Self {
            current_frame_start: None,
            current_triangle_count: 0,
            current_draw_calls: 0,
            frame_count_since_last_second: 0,
            last_fps_update: Instant::now(),
            data: DiagnosticsData::default(),
        }
    }

    /// Explicitly provide the initialization time, useful for testing
    pub fn new_with_time(now: Instant) -> Self {
        Self {
            current_frame_start: None,
            current_triangle_count: 0,
            current_draw_calls: 0,
            frame_count_since_last_second: 0,
            last_fps_update: now,
            data: DiagnosticsData::default(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.begin_frame_with_time(Instant::now());
    }

    pub fn begin_frame_with_time(&mut self, now: Instant) {
        self.current_frame_start = Some(now);
        self.current_triangle_count = 0;
        self.current_draw_calls = 0;
    }

    pub fn record_draw_call(&mut self, triangle_count: u32) {
        self.current_draw_calls += 1;
        self.current_triangle_count += triangle_count;
    }

    pub fn end_frame(&mut self, gpu_time_ms: f64) {
        self.end_frame_with_time(Instant::now(), gpu_time_ms);
    }

    pub fn end_frame_with_time(&mut self, now: Instant, gpu_time_ms: f64) {
        let cpu_time = if let Some(start) = self.current_frame_start {
            now.duration_since(start).as_secs_f64() * 1000.0
        } else {
            0.0
        };

        self.data.cpu_frame_time_ms = cpu_time;
        self.data.gpu_frame_time_ms = gpu_time_ms;
        self.data.triangle_count = self.current_triangle_count;
        self.data.draw_calls = self.current_draw_calls;
        
        self.frame_count_since_last_second += 1;
        
        let time_since_last_fps_update = now.duration_since(self.last_fps_update);
        if time_since_last_fps_update >= Duration::from_secs(1) {
            let elapsed_seconds = time_since_last_fps_update.as_secs_f64();
            if elapsed_seconds > 0.0 {
                self.data.fps = (self.frame_count_since_last_second as f64 / elapsed_seconds).round() as u32;
            } else {
                self.data.fps = self.frame_count_since_last_second;
            }
            self.frame_count_since_last_second = 0;
            self.last_fps_update = now;
        }
    }

    pub fn get_diagnostics(&self) -> DiagnosticsData {
        self.data
    }
}
