#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCategory {
    SFX,
    Music,
    Voice,
    UI,
}

#[derive(Debug, Clone)]
pub struct SoundMixerCore {
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
    voice_volume: f32,
    ui_volume: f32,

    saved_master: f32,
    saved_sfx: f32,
    saved_music: f32,
    saved_voice: f32,
    saved_ui: f32,
}

impl Default for SoundMixerCore {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundMixerCore {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 1.0,
            voice_volume: 1.0,
            ui_volume: 1.0,

            saved_master: 1.0,
            saved_sfx: 1.0,
            saved_music: 1.0,
            saved_voice: 1.0,
            saved_ui: 1.0,
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_voice_volume(&mut self, volume: f32) {
        self.voice_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_ui_volume(&mut self, volume: f32) {
        self.ui_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn get_voice_volume(&self) -> f32 {
        self.voice_volume
    }

    pub fn get_ui_volume(&self) -> f32 {
        self.ui_volume
    }

    pub fn save_and_mute(&mut self) {
        self.saved_master = self.master_volume;
        self.saved_sfx = self.sfx_volume;
        self.saved_music = self.music_volume;
        self.saved_voice = self.voice_volume;
        self.saved_ui = self.ui_volume;

        self.master_volume = 0.0;
        self.sfx_volume = 0.0;
        self.music_volume = 0.0;
        self.voice_volume = 0.0;
        self.ui_volume = 0.0;
    }

    pub fn restore_volume(&mut self) {
        self.master_volume = self.saved_master;
        self.sfx_volume = self.saved_sfx;
        self.music_volume = self.saved_music;
        self.voice_volume = self.saved_voice;
        self.ui_volume = self.saved_ui;
    }

    pub fn compute_final_volume(&self, category: SoundCategory, base_vol: f32) -> f32 {
        let category_vol = match category {
            SoundCategory::SFX => self.sfx_volume,
            SoundCategory::Music => self.music_volume,
            SoundCategory::Voice => self.voice_volume,
            SoundCategory::UI => self.ui_volume,
        };

        (self.master_volume * category_vol * base_vol).clamp(0.0, 1.0)
    }
}
