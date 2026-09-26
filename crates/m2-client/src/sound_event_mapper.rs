// Modul mapowania zdarzen gry na sciezki dzwiekow Metin2 (Faza B - Etap B5).
// Czysty kod bez polskich znakow diakrytycznych.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceType {
    Grass,
    Stone,
    Water,
    Dirt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSoundEvent {
    PickMoney,
    DropItem,
    ButtonClick,
    SkillSuccess,
    CriticalHit,
    Footstep(SurfaceType),
}

pub struct SoundEventMapper;

impl SoundEventMapper {
    pub fn get_sound_path(event: GameSoundEvent) -> &'static str {
        match event {
            GameSoundEvent::PickMoney => "sound/ui/money.wav",
            GameSoundEvent::DropItem => "sound/ui/drop.wav",
            GameSoundEvent::ButtonClick => "sound/ui/click.wav",
            GameSoundEvent::SkillSuccess => "sound/ui/loginbutton.wav",
            GameSoundEvent::CriticalHit => "sound/common/spell/spell_magical_hit.wav",
            GameSoundEvent::Footstep(surface) => Self::get_step_sound(surface, 0),
        }
    }

    pub fn get_step_sound(surface: SurfaceType, step_index: u8) -> &'static str {
        let is_left = (step_index % 2) == 0;
        match surface {
            SurfaceType::Grass => {
                if is_left {
                    "sound/common/walk/walk_grass_01.wav"
                } else {
                    "sound/common/walk/walk_grass_02.wav"
                }
            }
            SurfaceType::Stone => {
                if is_left {
                    "sound/common/walk/walk_stone_01.wav"
                } else {
                    "sound/common/walk/walk_stone_02.wav"
                }
            }
            SurfaceType::Water => {
                if is_left {
                    "sound/common/walk/walk_water_01.wav"
                } else {
                    "sound/common/walk/walk_water_02.wav"
                }
            }
            SurfaceType::Dirt => {
                if is_left {
                    "sound/common/walk/walk_dirt_01.wav"
                } else {
                    "sound/common/walk/walk_dirt_02.wav"
                }
            }
        }
    }
}
