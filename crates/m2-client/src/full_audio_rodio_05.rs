use crate::rodio_sfx_player::RodioSfxPlayer;

pub struct AudioEngine {
    pub sfx_player: RodioSfxPlayer,
}

impl AudioEngine {
    pub fn new() -> Result<Self, String> {
        let sfx_player = RodioSfxPlayer::new()?;
        Ok(Self {
            sfx_player,
        })
    }
}
