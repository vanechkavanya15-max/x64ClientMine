use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleHitEvent {
    pub attacker_vid: u32,
    pub victim_vid: u32,
    pub hit_type: u8,
    pub motion_code: u32,
}

pub struct BattleEventDispatcher {
    hit_event_queue: VecDeque<BattleHitEvent>,
    recent_hit_timestamps: VecDeque<Instant>,
}

impl BattleEventDispatcher {
    pub fn new() -> Self {
        Self {
            hit_event_queue: VecDeque::new(),
            recent_hit_timestamps: VecDeque::new(),
        }
    }

    pub fn push_hit(&mut self, event: BattleHitEvent) {
        let now = Instant::now();
        let one_second_ago = now.checked_sub(Duration::from_secs(1)).unwrap_or(now);

        // Remove old timestamps
        while let Some(&oldest) = self.recent_hit_timestamps.front() {
            if oldest < one_second_ago {
                self.recent_hit_timestamps.pop_front();
            } else {
                break;
            }
        }

        // Flood protection (max 10 hits per second)
        if self.recent_hit_timestamps.len() < 10 {
            self.recent_hit_timestamps.push_back(now);
            self.hit_event_queue.push_back(event);
        }
    }

    pub fn flush_hits(&mut self) -> Vec<BattleHitEvent> {
        self.hit_event_queue.drain(..).collect()
    }
}
