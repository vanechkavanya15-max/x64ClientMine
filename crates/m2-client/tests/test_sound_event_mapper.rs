#[path = "../src/sound_event_mapper.rs"]
mod sound_event_mapper;

use sound_event_mapper::{GameSoundEvent, SoundEventMapper, SurfaceType};

#[test]
fn test_ui_sound_paths() {
    assert_eq!(SoundEventMapper::get_sound_path(GameSoundEvent::PickMoney), "sound/ui/money.wav");
    assert_eq!(SoundEventMapper::get_sound_path(GameSoundEvent::DropItem), "sound/ui/drop.wav");
    assert_eq!(SoundEventMapper::get_sound_path(GameSoundEvent::ButtonClick), "sound/ui/click.wav");
    assert_eq!(SoundEventMapper::get_sound_path(GameSoundEvent::SkillSuccess), "sound/ui/loginbutton.wav");
    assert_eq!(SoundEventMapper::get_sound_path(GameSoundEvent::CriticalHit), "sound/common/spell/spell_magical_hit.wav");
}

#[test]
fn test_footstep_sounds_surfaces() {
    let grass_left = SoundEventMapper::get_step_sound(SurfaceType::Grass, 0);
    let grass_right = SoundEventMapper::get_step_sound(SurfaceType::Grass, 1);
    assert_ne!(grass_left, grass_right);
    assert!(grass_left.contains("grass"));
    assert!(grass_right.contains("grass"));

    let stone_left = SoundEventMapper::get_step_sound(SurfaceType::Stone, 2);
    let stone_right = SoundEventMapper::get_step_sound(SurfaceType::Stone, 3);
    assert_ne!(stone_left, stone_right);
    assert!(stone_left.contains("stone"));
    assert!(stone_right.contains("stone"));

    let water_left = SoundEventMapper::get_step_sound(SurfaceType::Water, 0);
    assert!(water_left.contains("water"));

    let dirt_left = SoundEventMapper::get_step_sound(SurfaceType::Dirt, 0);
    assert!(dirt_left.contains("dirt"));
}
