use std::collections::HashMap;

const MIN_SOUND_INTERVAL_MS: u32 = 300;
const RECORD_EXPIRATION_MS: u32 = 10_000;

#[derive(Default)]
pub struct SoundDebouncer {
    history: HashMap<(u32, u32), u32>,
}

impl SoundDebouncer {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    pub fn can_play(&mut self, actor_vid: u32, sound_hash: u32, cur_time_ms: u32) -> bool {
        let key = (actor_vid, sound_hash);

        if let Some(&last_time) = self.history.get(&key) {
            let elapsed = cur_time_ms.wrapping_sub(last_time);
            if elapsed < MIN_SOUND_INTERVAL_MS {
                return false;
            }
        }

        self.history.insert(key, cur_time_ms);
        true
    }

    pub fn cleanup_old_records(&mut self, cur_time_ms: u32) {
        self.history.retain(|_, &mut last_time| {
            let elapsed = cur_time_ms.wrapping_sub(last_time);
            elapsed <= RECORD_EXPIRATION_MS
        });
    }
}
