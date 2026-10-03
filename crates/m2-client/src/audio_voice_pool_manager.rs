use rodio::Sink;

pub const MAX_VOICES: usize = 32;

pub struct Voice {
    pub sink: Sink,
    pub volume: f32,
}

pub struct AudioVoicePoolManager {
    pub voices: Vec<Voice>,
}

impl Default for AudioVoicePoolManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioVoicePoolManager {
    pub fn new() -> Self {
        Self {
            voices: Vec::with_capacity(MAX_VOICES),
        }
    }

    pub fn cleanup_finished_voices(&mut self) {
        self.voices.retain(|v| !v.sink.empty());
    }

    pub fn add_voice(&mut self, sink: Sink, volume: f32) -> bool {
        self.cleanup_finished_voices();

        if self.voices.len() < MAX_VOICES {
            self.voices.push(Voice { sink, volume });
            return true;
        }

        // Pool is full, find the quietest voice
        let mut min_idx = 0;
        let mut min_vol = self.voices[0].volume;

        for (i, v) in self.voices.iter().enumerate().skip(1) {
            if v.volume < min_vol {
                min_vol = v.volume;
                min_idx = i;
            }
        }

        // If the new voice is louder than the quietest voice in the pool
        if volume > min_vol {
            self.voices[min_idx].sink.pause(); // stop the old one explicitly
            self.voices[min_idx] = Voice { sink, volume };
            return true;
        }

        // The new voice is not louder than the quietest one, reject it
        false
    }
}
