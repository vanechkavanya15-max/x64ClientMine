#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SoundCategory {
    Effect = 0,
    Music = 1,
    Voice = 2,
    Ui = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EMusicState {
    Off = 0,
    Play = 1,
    FadeIn = 2,
    FadeOut = 3,
    FadeLimitOut = 4,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoundInstance {
    pub id: u32,
    pub file_name: String,
    pub category: SoundCategory,
    volume: f32,
    pan: f32,
    pub pitch: f32,
    pub is_looping: bool,
    pub play_count: i32,
}

impl SoundInstance {
    pub fn new(
        id: u32,
        file_name: String,
        category: SoundCategory,
        volume: f32,
        pan: f32,
        pitch: f32,
        is_looping: bool,
        play_count: i32,
    ) -> Self {
        let mut instance = Self {
            id,
            file_name,
            category,
            volume: 0.0,
            pan: 0.0,
            pitch,
            is_looping,
            play_count,
        };
        instance.set_volume(volume);
        instance.set_pan(pan);
        instance
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn pan(&self) -> f32 {
        self.pan
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }
}
