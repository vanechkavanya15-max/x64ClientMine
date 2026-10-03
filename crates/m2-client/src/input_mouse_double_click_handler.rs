use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleClickAction {
    UsePotion,
    EquipWeapon,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance_squared(&self, other: &Self) -> u32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy) as u32
    }
}

pub struct DoubleClickDetector {
    last_click_time: Option<Instant>,
    last_click_pos: Option<Position>,
    double_click_timeout: Duration,
    max_distance_squared: u32,
}

impl Default for DoubleClickDetector {
    fn default() -> Self {
        Self::new(Duration::from_millis(500), 25) // 5 pixels distance -> 25 squared
    }
}

impl DoubleClickDetector {
    pub fn new(timeout: Duration, max_dist_sq: u32) -> Self {
        Self {
            last_click_time: None,
            last_click_pos: None,
            double_click_timeout: timeout,
            max_distance_squared: max_dist_sq,
        }
    }

    /// Determines if a left click triggers a double click action.
    /// If it does, we return `DoubleClickAction::UsePotion` as an example action.
    /// In a real scenario, this would depend on the target clicked.
    /// For the scope of this implementation, we map a double click to an action based on context,
    /// but since context isn't provided as a parameter yet, we simply return a generic Action or None.
    /// We'll modify it to accept an optional 'context' or just return `UsePotion` for testing.
    pub fn on_left_click(&mut self, current_time: Instant, pos: Position, context: Option<DoubleClickAction>) -> DoubleClickAction {
        if let (Some(last_time), Some(last_pos)) = (self.last_click_time, self.last_click_pos) {
            let time_elapsed = current_time.duration_since(last_time);
            let distance_sq = pos.distance_squared(&last_pos);

            if time_elapsed <= self.double_click_timeout && distance_sq <= self.max_distance_squared {
                // Double click detected!
                // Reset to prevent triple-click counting as two double-clicks
                self.last_click_time = None;
                self.last_click_pos = None;
                
                return context.unwrap_or(DoubleClickAction::UsePotion);
            }
        }

        // Single click or invalid double click (too far / too slow)
        self.last_click_time = Some(current_time);
        self.last_click_pos = Some(pos);
        
        DoubleClickAction::None
    }
}
