use std::time::Instant;

/// A precise frame delta time clock with lag protection.
/// Caps the maximum delta time to 0.1 seconds to prevent physics explosions during lag spikes.
#[derive(Debug, Clone)]
pub struct FrameDeltaTimeClock {
    last_update: Option<Instant>,
    max_delta_seconds: f32,
}

impl FrameDeltaTimeClock {
    /// Creates a new `FrameDeltaTimeClock` with a maximum allowed delta of 0.1 seconds.
    pub fn new() -> Self {
        Self {
            last_update: None,
            max_delta_seconds: 0.1,
        }
    }

    /// Resets the clock. The next update will yield a delta of 0.0 seconds.
    pub fn reset(&mut self) {
        self.last_update = None;
    }

    /// Updates the clock and returns the delta time in seconds.
    /// Caps the delta time at `max_delta_seconds` (0.1s by default).
    pub fn update(&mut self) -> f32 {
        self.update_with_time(Instant::now())
    }

    /// Updates the clock given the current time and returns the delta time in seconds.
    /// Caps the delta time at `max_delta_seconds` (0.1s by default).
    pub fn update_with_time(&mut self, now: Instant) -> f32 {
        match self.last_update {
            Some(last_time) => {
                // Use saturating_duration_since to handle potential anomalies 
                // in monotonic clocks where 'now' might be slightly before 'last_time'.
                let delta_duration = now.saturating_duration_since(last_time);
                let delta_seconds = delta_duration.as_secs_f32();
                
                self.last_update = Some(now);
                
                if delta_seconds > self.max_delta_seconds {
                    self.max_delta_seconds
                } else {
                    delta_seconds
                }
            }
            None => {
                // First update after creation or reset returns 0.0
                self.last_update = Some(now);
                0.0
            }
        }
    }
}

impl Default for FrameDeltaTimeClock {
    fn default() -> Self {
        Self::new()
    }
}
