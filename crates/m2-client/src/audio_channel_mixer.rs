#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioChannel {
    Master,
    Bgm,
    Sfx,
    Voice,
}

#[derive(Debug, Clone)]
pub struct ChannelState {
    pub volume: f32, // 0.0 to 1.0
    pub muted: bool,
}

impl Default for ChannelState {
    fn default() -> Self {
        Self {
            volume: 1.0,
            muted: false,
        }
    }
}

pub struct AudioChannelMixer {
    master: ChannelState,
    bgm: ChannelState,
    sfx: ChannelState,
    voice: ChannelState,
}

impl Default for AudioChannelMixer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioChannelMixer {
    pub fn new() -> Self {
        Self {
            master: ChannelState::default(),
            bgm: ChannelState::default(),
            sfx: ChannelState::default(),
            voice: ChannelState::default(),
        }
    }

    fn get_channel_mut(&mut self, channel: AudioChannel) -> &mut ChannelState {
        match channel {
            AudioChannel::Master => &mut self.master,
            AudioChannel::Bgm => &mut self.bgm,
            AudioChannel::Sfx => &mut self.sfx,
            AudioChannel::Voice => &mut self.voice,
        }
    }

    fn get_channel(&self, channel: AudioChannel) -> &ChannelState {
        match channel {
            AudioChannel::Master => &self.master,
            AudioChannel::Bgm => &self.bgm,
            AudioChannel::Sfx => &self.sfx,
            AudioChannel::Voice => &self.voice,
        }
    }

    pub fn set_volume(&mut self, channel: AudioChannel, volume: f32) {
        let vol = volume.clamp(0.0, 1.0);
        self.get_channel_mut(channel).volume = vol;
    }

    pub fn get_volume(&self, channel: AudioChannel) -> f32 {
        self.get_channel(channel).volume
    }

    pub fn set_muted(&mut self, channel: AudioChannel, muted: bool) {
        self.get_channel_mut(channel).muted = muted;
    }

    pub fn is_muted(&self, channel: AudioChannel) -> bool {
        self.get_channel(channel).muted
    }

    /// Returns the effective volume for a specific channel, taking into account its own settings
    /// and the master settings. If the requested channel is Master, returns just the Master effective volume.
    pub fn get_effective_volume(&self, channel: AudioChannel) -> f32 {
        let master_eff = if self.master.muted { 0.0 } else { self.master.volume };
        
        if channel == AudioChannel::Master {
            return master_eff;
        }

        let ch = self.get_channel(channel);
        let ch_eff = if ch.muted { 0.0 } else { ch.volume };

        ch_eff * master_eff
    }
}
