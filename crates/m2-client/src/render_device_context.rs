use std::time::Instant;

/// Manages the context for the 3D rendering device.
pub struct RenderDeviceContext {
    pub viewport_x: u32,
    pub viewport_y: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub clear_color: [f32; 4],
    pub clear_depth: f32,
    pub vsync_enabled: bool,
    
    is_in_frame: bool,
    
    frame_count: u32,
    last_fps_update: Instant,
    current_fps: u32,
    
    frame_start: Option<Instant>,
    cpu_overhead_ms: f32,
    gpu_overhead_ms: f32,
}

impl RenderDeviceContext {
    /// Creates a new RenderDeviceContext with default parameters.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            viewport_x: 0,
            viewport_y: 0,
            viewport_width: width,
            viewport_height: height,
            clear_color: [0.5, 0.6, 0.7, 1.0], // Default sky/fog color
            clear_depth: 1.0,
            vsync_enabled: true,
            
            is_in_frame: false,
            
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0,
            
            frame_start: None,
            cpu_overhead_ms: 0.0,
            gpu_overhead_ms: 0.0,
        }
    }

    /// Sets the rendering viewport dimensions.
    pub fn set_viewport(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.viewport_x = x;
        self.viewport_y = y;
        self.viewport_width = w;
        self.viewport_height = h;
    }

    /// Begins a new rendering frame.
    pub fn begin_frame(&mut self) -> Result<(), &'static str> {
        if self.is_in_frame {
            return Err("Frame already started");
        }
        if self.viewport_width == 0 || self.viewport_height == 0 {
            return Err("Invalid viewport dimensions");
        }

        self.is_in_frame = true;
        self.frame_start = Some(Instant::now());

        Ok(())
    }

    /// Ends the current rendering frame and calculates CPU overhead and FPS.
    pub fn end_frame(&mut self) {
        if !self.is_in_frame {
            return;
        }

        let now = Instant::now();
        
        // Calculate CPU overhead based on frame duration
        if let Some(start) = self.frame_start {
            let duration = now.duration_since(start);
            self.cpu_overhead_ms = duration.as_secs_f32() * 1000.0;
        }

        self.is_in_frame = false;
        self.frame_count += 1;

        // Calculate FPS if at least 1 second has passed
        let time_since_last_update = now.duration_since(self.last_fps_update);
        if time_since_last_update.as_secs_f32() >= 1.0 {
            self.current_fps = (self.frame_count as f32 / time_since_last_update.as_secs_f32()).round() as u32;
            self.frame_count = 0;
            self.last_fps_update = now;
        }
    }

    /// Explicitly updates the GPU overhead (usually fetched from a graphics API query).
    pub fn set_gpu_overhead(&mut self, ms: f32) {
        self.gpu_overhead_ms = ms;
    }

    /// Returns the current frames per second (FPS).
    pub fn get_fps(&self) -> u32 {
        self.current_fps
    }

    /// Returns the CPU overhead per frame in milliseconds.
    pub fn get_cpu_overhead_ms(&self) -> f32 {
        self.cpu_overhead_ms
    }

    /// Returns the GPU overhead per frame in milliseconds.
    pub fn get_gpu_overhead_ms(&self) -> f32 {
        self.gpu_overhead_ms
    }

    /// Returns whether the device is currently rendering a frame.
    pub fn is_in_frame(&self) -> bool {
        self.is_in_frame
    }
    
    /// Used for testing purposes to forcefully override the last fps update time
    #[cfg(test)]
    pub(crate) fn force_last_fps_update(&mut self, time: Instant) {
        self.last_fps_update = time;
    }
}
